#![no_main]
#![no_std]

extern crate panic_semihosting;

use core::ops::DerefMut;
use core::{f32::consts::PI, ptr};
use micromath::F32Ext;

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
//use stm32f3xx_hal::{pac, prelude::*, serial::Serial};

use aligned::{Aligned, A4};
use byteorder::{ByteOrder, LE};
use cast::{f32, i32};
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use embedded_hal::digital::v1_compat::OldOutputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::Qei;
//se f3::{

use stm32f3xx_hal::{  i2c::I2c, spi::Spi, 
            serial::{Rx, Serial, Tx},
            stm32::{self, USART2}, 
            timer::Timer, time::MonoTimer,
            pwm::tim3,flash::FlashExt,
            };


use l3gd20::{self, Odr};
use lsm303dlhc::{AccelOdr, MagOdr};
//};
use motor_driver::Motor;
use rotary_encoder_hal::{Direction, Rotary};
use pid::Pid;
use madgwick::{F32x3, Marg};
use nb::block;
//use dcmimu::DCMIMU;

//use ahrs::{Ahrs, Madgwick, Mahony};
//use nalgebra::{Vector3, Quaternion};

// Number of samples to use for gyroscope calibration
const NSAMPLES: i32 = 256;

// Sensitivities of the accelerometer and gyroscope, respectively
const K_G: f32 = 2. / (1 << 15) as f32; // LSB -> g (ACCEL)
const K_AR: f32 = -8.75e-3; //* PI / 180.; // LSB -> Deg/S (GYRO)

// Madgwick filter parameters
const SAMPLE_FREQ: u32 = 220;
const BETA: f32 = 1.;

const BUFF_SIZE: usize = 40;

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);

      // Serial USART pins
      let pd5 = gpiod.pd5.into_af7(&mut gpiod.moder, &mut gpiod.afrl);
      let pd6 = gpiod.pd6.into_af7(&mut gpiod.moder, &mut gpiod.afrl);

      let serial = Serial::usart2(
          dp.USART2,
          (pd5, pd6),
          230400.bps(),
          clocks,
          &mut rcc.apb1
      );

      let (tx, rx) = serial.split();

      let dma1 = dp.DMA1.split(&mut rcc.ahb);


      // transmit data
      let tx_buf = singleton!(: [u8; 9] = *b"hello DMA").unwrap();
      
      // receive data
      let rx_buf = singleton!(: [u8; 9] = [0; 9]).unwrap();

      let (tx_channel, rx_channel) = (dma1.ch4, dma1.ch5);

    let sending = tx.write_all(tx_buf, tx_channel);
    let receiving = rx.read_exact(rx_buf, rx_channel);

    let (tx_buf, tx_channel, tx) = sending.wait();
    let (rx_buf, rx_channel, rx) = receiving.wait();

    //let (mut tx, mut rx) = serial.split();

    //let usart2 = unsafe {&mut *(stm32::USART2::ptr() as *mut _)};
                                           // stm32f3xx_hal::time::MonoTimer::new(cp.DWT, clocks),
                                           // cp.ITM)};
 
    //usart2.tdr.write(|w| w.tdr().bits(u16::from(b'X')));

    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high().unwrap();
    let mut led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

// motor controls
    let pb12 = gpiob.pb12.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let pb13 = gpiob.pb13.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let pb14 = gpiob.pb14.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let pb15 = gpiob.pb15.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

// Motor PWM pins
    let pb4 = gpiob.pb4.into_af2(&mut gpiob.moder, &mut gpiob.afrl);  // pb4 set for TIM3_CH1 (GPIO alternate function low register)
    let pb1 = gpiob.pb1.into_af2(&mut gpiob.moder, &mut gpiob.afrl);  // pb5 set for TIM3_CH3

     // TIM3
     // A four channel general purpose timer that's broadly available
  let tim3_channels = tim3(
       dp.TIM3,
       1280,    // resolution of duty cycle
        25000.hz(), // frequency
       &clocks, // To get the timer's clock speed
   );

    let mut pwm1 = tim3_channels.0.output_to_pb4(pb4);  //setup pb4 PWM output
    let mut pwm2 = tim3_channels.3.output_to_pb1(pb1);  //setup pb5 PWM output

    let mut m1 = Motor::l298(pb12, pb13, pwm1);  //IN1 IN2 PWM
    let mut m2 = Motor::l298(pb14, pb15, pwm2);

    // Encoder inputs 
    let pb8 = gpiob.pb8.into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let pb9 = gpiob.pb9.into_pull_up_input(&mut gpiob.moder, &mut gpiob.pupdr);
    let pa2 = gpioa.pa2.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);
    let pa3 = gpioa.pa3.into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

    let mut enc1 = Rotary::new(pb8, pb9);
    let mut enc2 = Rotary::new(pa2, pa3);

    // setup gyroscope
    let mut l3gd20 = L3gd20::new(spi, OldOutputPin::from(nss)).unwrap();
    l3gd20.set_odr(Odr::Hz380).unwrap();


    // setup i2c for accel
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    // setup accel and mag
    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
    lsm303dlhc.accel_odr(AccelOdr::Hz400).unwrap(); // set teh operation frequency
    lsm303dlhc.mag_odr(MagOdr::Hz220).unwrap();
    
    // setup 380Hz timer, matching gyro
    let mut timer = Timer::tim2(dp.TIM2, 380.hz(), clocks, &mut rcc.apb1);

    // Calibrate the gyroscope

    let mut timer = Timer::tim2(timer.release(), SAMPLE_FREQ.hz(), clocks, &mut rcc.apb1);


    let mut tx_buf: Aligned<A4, [u8; BUFF_SIZE+2]> = Aligned([0; BUFF_SIZE+2]);

    let mut count = 1.;

    let setpoint = -0.010;

    let max = m1.get_max_duty() as f32;

    let Kp = 1800.;
    let Ki = 150.;
    let Kd = 0.;
    let Dz = 0.48;
    

    let maxp = max - Dz * max;

    let mut pid = Pid::new(Kp, Ki, Kd, maxp, maxp, maxp, setpoint);

    let mut errSum = 0.05;
    //let mut pitch = 0.9;
    let mut delay = 0;
    let mut error: f32 = 0.;
    let mut output = 0.0;

    let mut R = 0.0;

    let mut aca = F32x3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    let mut gya = F32x3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let mut firstSample = false;
    let mut RwEst = F32x3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut RwGyro = F32x3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mut Awz = F32x3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut avg_cnt = 0;
    loop {
        block!(timer.wait()).unwrap();

       // let m = lsm303dlhc.mag().unwrap();
        let gyr = l3gd20.all().unwrap().gyro;  // Gyroscope values
        let acc = lsm303dlhc.accel().unwrap();  // grab acell measurements dps

        gya.x += f32(gyr.x);
        gya.y += f32(gyr.y);
        gya.z += f32(gyr.z);
        aca.x += f32(acc.x);
        aca.y += f32(acc.y);
        aca.z += f32(acc.z);

        avg_cnt += 1;
        let samples = 6;

        if avg_cnt == samples {  // convert to 6 and shift

            aca.x = (aca.x/f32(samples))*K_G;
            aca.y = (aca.y/f32(samples))*K_G;
            aca.z = (aca.z/f32(samples))*K_G;
            gya.y = (gya.y/f32(samples))*K_AR;
            gya.x = (gya.x/f32(samples))*K_AR;
            gya.z = (gya.z/f32(samples))*K_AR;


            R = (aca.x * aca.x + aca.y * aca.y + aca.z * aca.z).sqrt();
            aca.x = aca.x/R;
            aca.y = aca.y/R;
            aca.z = aca.z/R;


            if firstSample {
                RwEst.x = aca.x;             
                RwEst.y = aca.y;
                RwEst.z = aca.z;
                firstSample = false;
            } else {
                if (RwEst.z).abs() < 0.1 {
                    RwGyro.x = RwEst.x; // use previous value is z is too small
                    RwGyro.y = RwEst.y; // use previous value is z is too small
                    RwGyro.z = RwEst.z;
                } else { //z is not too small
                    // gyrorate is dps
                    Awz.y = (RwEst.y).atan2(RwEst.z) * (180./PI);  // get angle convert to deg
                    Awz.y += gya.y * f32(samples)/f32(SAMPLE_FREQ);  //# of samples / sampling rate

                    Awz.x = (RwEst.x).atan2(RwEst.z) * (180./PI);  // get angle convert to deg
                    Awz.x += gya.x * f32(samples)/f32(SAMPLE_FREQ);  //# of samples / sampling rate
                

                let signRzGyro = if (Awz.x * (PI/180.)).cos() >= 0. {1.} else {-1.};

                    RwGyro.x = (Awz.x * PI / 180.).sin();
                    RwGyro.x /= ( 1. + squared((Awz.x * PI / 180.).cos()) * squared((Awz.y * PI / 180.).tan()) ).sqrt();
                    RwGyro.y = (Awz.y * PI / 180.).sin();
                    RwGyro.y /= ( 1. + squared((Awz.y * PI / 180.).cos()) * squared((Awz.x * PI / 180.).tan()) ).sqrt(); 

                    RwGyro.z = signRzGyro * (1. - squared(RwGyro.x) - squared(RwGyro.y)).sqrt();
                
                }  //z is too msall end
                // add for statement back
                let wGyro = 9.;
                RwEst.x = (aca.x + wGyro * RwGyro.x) / (1. + wGyro);
                RwEst.y = (aca.y + wGyro * RwGyro.y) / (1. + wGyro);
                RwEst.z = (aca.z + wGyro * RwGyro.z) / (1. + wGyro);

                R = (RwEst.x * RwEst.x + RwEst.y * RwEst.y + RwEst.z * RwEst.z).sqrt();
                RwEst.x = RwEst.x/R;
                RwEst.y = RwEst.y/R;
                RwEst.z = RwEst.z/R;

            }  // end else not first sample
        
        
        if delay > 100 {

    let temp = pid.next_control_output(RwEst.y);
    output = f32(temp.output);

        if RwEst.y < 0. {
            output = output + max * Dz;
            m1.cw();
            m2.cw();
        } else {
            output = (max * -Dz + output).abs();  //output from PID is negative
            m1.ccw();
            m2.ccw();
        }

        if (RwEst.y).abs() > 0.6 {
            output = 0.;
            errSum = 0.;
        }

            m1.duty(output as u16);
            m2.duty(output as u16);

        }  // if delay has not been reached
        else {
            m1.duty(0);
            m2.duty(0);
            delay += 1;
            if delay == 500 {
            led.set_high().unwrap();
            }
        }


        // Serialize the quaternion
        let mut start = 0;
            let mut buf = [0; BUFF_SIZE];
            LE::write_f32(&mut buf[start..start + 4], RwEst.x);
            start += 4;
            LE::write_f32(&mut buf[start..start + 4], RwEst.y);  // g for accell (force g)
            start += 4;
            LE::write_f32(&mut buf[start..start + 4], RwEst.z);
            start += 4;

            LE::write_f32(&mut buf[start..start + 4], aca.x);
            start += 4;
            LE::write_f32(&mut buf[start..start + 4], aca.y);
            start += 4;
            LE::write_f32(&mut buf[start..start + 4], aca.z);
            start += 4;

            LE::write_f32(&mut buf[start..start + 4], gya.x);
            start += 4;
            LE::write_f32(&mut buf[start..start + 4], gya.y);  //8
            start += 4;
            LE::write_f32(&mut buf[start..start + 4], gya.z);
            start += 4;

            LE::write_f32(&mut buf[start..start + 4], output);

            count += 1.;
            // Log data
            cobs::encode(&buf, tx_buf.deref_mut());

            //itm::write_aligned(&mut cp.ITM.stim[0], &tx_buf);
           // tx.write(|w| w.tdr().bits(u16::from(b'X')));


            gya.x = 0.;
            gya.y = 0.;
            gya.z = 0.;
            aca.x = 0.;
            aca.y = 0.;
            aca.z = 0.;
            avg_cnt = 0;
    
        }
    }
}


fn squared(x:f32) -> f32{
    x * x
}
