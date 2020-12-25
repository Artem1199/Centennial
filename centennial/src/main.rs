#![no_main]
#![no_std]

extern crate panic_semihosting;


use cobs;
use core::ops::DerefMut;
use core::{f32::consts::PI, ptr};
use micromath::F32Ext;

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;

use either::Either;

use aligned::{Aligned, A4};
use byteorder::{ByteOrder, LE};
use cast::{f32, i32};
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use embedded_hal::digital::v1_compat::OldOutputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::Qei;

    use panic_semihosting as _;
use hal::{ i2c, spi, prelude::*,
            serial::{Rx, Serial, Tx},
            stm32::{self, USART2},
            stm32::Interrupt,
            timer::Timer, time::MonoTimer,
            timer,
            pwm::{TIM3_CH1, TIM3_CH4, tim3},flash::FlashExt,
            pac, gpio
            };

use hal::dma;
use hal::dma::Channel;
use hal::dma::Event;
use hal::dma::Transfer;

use l3gd20::{self, L3gd20, Odr};
use lsm303dlhc::{AccelOdr, Lsm303dlhc, MagOdr};
//use rtfm::{app, cyccnt::U32Ext, Exclusive};

//};
use motor_driver::Motor;
use motor_driver::ic::L298;
use rotary_encoder_hal::{Direction, Rotary};
use pid::Pid;
use madgwick::{F32x3, Marg};
use nb::block;

// Number of samples to use for gyroscope calibration
const NSAMPLES: i32 = 256;

// Sensitivities of the accelerometer and gyroscope, respectively
const K_G: f32 = 2. / (1 << 15) as f32; // LSB -> g (ACCEL)
const K_AR: f32 = -8.75e-3; //* PI / 180.; // LSB -> Deg/S (GYRO)

// Madgwick filter parameters
const SAMPLE_FREQ: u32 = 400;
const BETA: f32 = 1.;

const BUFF_SIZE: usize = 40;

type I2C = i2c::I2c<
    stm32::I2C1,
    (
        gpio::gpiob::PB6<gpio::AF4>,
        gpio::gpiob::PB7<gpio::AF4>,
    ),
>;
type Spi = spi::Spi<
    stm32::SPI1,
    (
        gpio::gpioa::PA5<gpio::AF5>,
        gpio::gpioa::PA6<gpio::AF5>,
        gpio::gpioa::PA7<gpio::AF5>,
    ),
>;
type Gyro = L3gd20<
        Spi,
        OldOutputPin::<gpio::gpioe::PE3<gpio::Output<gpio::PushPull>>>>;
type Accel = Lsm303dlhc<I2C>;

type m1 = motor_driver::Motor<
    gpio::gpiob::PB12<gpio::Output<gpio::PushPull>>,
    gpio::gpiob::PB13<gpio::Output<gpio::PushPull>>,
    hal::pwm::PwmChannel<TIM3_CH1, hal::pwm::WithPins>,
    L298,
>;

type m2 = motor_driver::Motor<
    gpio::gpiob::PB14<gpio::Output<gpio::PushPull>>,
    gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>,
    hal::pwm::PwmChannel<TIM3_CH4, hal::pwm::WithPins>,
    L298,
>;

type TX = Tx<USART2>;
type RX = Rx<USART2>;
type TX_BUF = &'static mut [u8; TX_SZ];
type RX_BUF = &'static mut [u8; RX_SZ];
type pid_control = Pid<f32>;
type TX_chn = Option<Either<(TX_BUF, dma::dma1::C7, TX), Transfer<TX_BUF, dma::dma1::C7, TX>>>;
type RX_chn = Option<Either<(RX_BUF, dma::dma1::C6, RX), Transfer<RX_BUF, dma::dma1::C6, RX>>>;


const TX_SZ: usize = 18;
const RX_SZ: usize = 24;


#[rtic::app(device = hal::stm32, peripherals = true)]
const APP: () = {

    struct Resources {
        Gyro: Gyro,
        Accel: Accel,
        m1: m1,
        m2: m2,
        R: f32,
        firstSample: bool,
        RwEst:  F32x3,
        RwGyro: F32x3,
        Awz:    F32x3,
        wGyro:  f32,
        aca:    F32x3,
        gya:    F32x3,
        avg_cnt: f32,
        dc_deadzone: f32,
        pid_control: pid_control,
        motor_offset: f32,
      //  TX_BUF: TX_BUF,
      //  RX_BUF: RX_BUF,
        TX: TX_chn,
        RX: RX_chn,
        ON: bool,


    } // end Resources

    #[init]
    fn init (cx: init::Context) -> init::LateResources{

       // let mut cp = cortex_m::Peripherals::take().unwrap();
       //let cx = pac::Peripherals::take().unwrap();
    
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
    
        let clocks = rcc
            .cfgr
            .sysclk(64.mhz())
            .pclk1(32.mhz())
            .freeze(&mut flash.acr);

        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.ahb);
        let mut gpioe = cx.device.GPIOE.split(&mut rcc.ahb);
        let mut gpiod = cx.device.GPIOD.split(&mut rcc.ahb);

            // Serial USART pins
        let pd5 = gpiod.pd5.into_af7(&mut gpiod.moder, &mut gpiod.afrl);
        let pd6 = gpiod.pd6.into_af7(&mut gpiod.moder, &mut gpiod.afrl);
            // USART2 init
        let serial = Serial::usart2(
            cx.device.USART2,
            (pd5, pd6),
            230400.bps(),
            clocks,
            &mut rcc.apb1
        );
            // reveal tx, rx pins
        let (mut usart_tx, usart_rx) = serial.split();

            // create buffers for send and receive
        let tx_buf = singleton!(: [u8; TX_SZ] = [0; TX_SZ]).unwrap();
        let rx_buf = singleton!(: [u8; RX_SZ] = [0; RX_SZ]).unwrap();
        // let tx_buf: [u8; TX_SZ] = [0; TX_SZ];
        // let rx_buf: [u8; RX_SZ] = [0; RX_SZ];

            // setup DMA1
        let mut dma1 = cx.device.DMA1.split(&mut rcc.ahb);
        
        usart_tx.write(0x00).ok().unwrap();
        let TX = Some(Either::Left((tx_buf, dma1.ch7, usart_tx)));

        dma1.ch6.listen(dma::Event::TransferComplete);
        let RX: RX_chn = Some(Either::Right(usart_rx.read_exact(rx_buf, dma1.ch6)));


            // led pin
        let mut led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

            // accel pin
        let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
         nss.set_high().unwrap();
            // spi pins
        let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
        let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
        let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
            // init spi
        let spi = Spi::spi1(
            cx.device.SPI1,
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
        let pb4 = gpiob.pb4.into_af2(&mut gpiob.moder, &mut gpiob.afrl);
        let pb1 = gpiob.pb1.into_af2(&mut gpiob.moder, &mut gpiob.afrl);
            // TIM3 for PWM
        let tim3_channels = tim3(
            cx.device.TIM3,
            1280,    // resolution of duty cycle
            25000.hz(), // frequency
            &clocks, // To get the timer's clock speed
        );
            // init PWM pins
        let mut pwm1 = tim3_channels.0.output_to_pb4(pb4);  //setup pb4 PWM output
        let mut pwm2 = tim3_channels.3.output_to_pb1(pb1);  //setup pb5 PWM output
            // init motor output
        let mut m1 = Motor::l298(pb12, pb13, pwm1);  //IN1 IN2 PWM
        let mut m2 = Motor::l298(pb14, pb15, pwm2);
            // setup gyroscope
        let mut l3gd20 = L3gd20::new(spi, OldOutputPin::from(nss)).unwrap();
        l3gd20.set_odr(Odr::Hz380).unwrap();  // 380Hz op freq for gyroscope
            // I2c for Accel
        let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
        let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
        let i2c = i2c::I2c::new(cx.device.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
            // setup accel
        let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
        lsm303dlhc.accel_odr(AccelOdr::Hz400).unwrap(); // 400Hz op freq for Accel
            // setup tick interrupts
        let mut timer = timer::Timer::tim2(cx.device.TIM2, 380.hz(), clocks, &mut rcc.apb1);
        timer.listen(timer::Event::Update);

            // init pid values & setpoint
        let Kp = 6400.0;
        let Ki = 100.0;
        let Kd = 0.5;
        let Dz = 0.35;
        let setpoint = 0.000;
        let Wgyro = 40.0;

        let max_dc = m1.get_max_duty();                   // find the max duty cycle (1280)
        let dc_deadzone = f32(max_dc) * Dz;
        let max_offset = f32(max_dc) - f32(max_dc) * Dz;  // find the max PID output (1280 - DZ * 1280)
            // setup PID
        let pid_control = Pid::new(Kp, Ki, Kd, max_offset, max_offset, max_offset, max_offset, setpoint);

        init::LateResources {
            m1,
            m2,
            Gyro: l3gd20,
            Accel: lsm303dlhc,
            R: 0.0,
            firstSample: false,
            RwEst:  F32x3 {x: 0.0, y: 0.0, z: 0.0},
            RwGyro: F32x3 {x: 0.0, y: 0.0, z: 0.0},
            Awz:    F32x3 {x: 0.0, y: 0.0, z: 0.0},
            aca:    F32x3 {x: 0.0, y: 0.0, z: 0.0},
            gya:    F32x3 {x: 0.0, y: 0.0, z: 0.0},
            pid_control: pid_control,
            avg_cnt: 0.0,
            dc_deadzone: dc_deadzone,
            wGyro: Wgyro,
            motor_offset: 0.0,
           // TX_BUF: tx_buf,
           // RX_BUF: rx_buf,
            TX: TX,
            RX: RX,
            ON: true,
        } //end init::LateResources
    } // end init

    #[idle]
    fn idle(cx: idle::Context) -> !{

        loop {
        }
    }

    #[task(binds = TIM2, resources = [m1, m2, Gyro, Accel, R, firstSample, RwEst, RwGyro, Awz, aca, gya, pid_control, avg_cnt, dc_deadzone, TX, wGyro, motor_offset])]
    fn tick(cx: tick::Context){

        let gyr = cx.resources.Gyro.all().unwrap().gyro;  // Gyroscope values
        let acc = cx.resources.Accel.accel().unwrap();  // grab acell measurements dps

        cx.resources.gya.x += f32(gyr.x);
        cx.resources.gya.y += f32(gyr.y);
        cx.resources.gya.z += f32(gyr.z);
        cx.resources.aca.x += f32(acc.x);
        cx.resources.aca.y += f32(acc.y);
        cx.resources.aca.z += f32(acc.z);

        *cx.resources.avg_cnt += 1.0;
        let samples = 4.0;

        if *cx.resources.avg_cnt == samples {  // convert to 6 and shift

            cx.resources.aca.x = (cx.resources.aca.x/f32(samples))*K_G;
            cx.resources.aca.y = (cx.resources.aca.y/f32(samples))*K_G;
            cx.resources.aca.z = (cx.resources.aca.z/f32(samples))*K_G;
            cx.resources.gya.y = (cx.resources.gya.y/f32(samples))*K_AR;
            cx.resources.gya.x = (cx.resources.gya.x/f32(samples))*K_AR;
            cx.resources.gya.z = (cx.resources.gya.z/f32(samples))*K_AR;

            *cx.resources.R = (cx.resources.aca.x * cx.resources.aca.x + cx.resources.aca.y * cx.resources.aca.y + cx.resources.aca.z * cx.resources.aca.z).sqrt();
            cx.resources.aca.x = cx.resources.aca.x/ *cx.resources.R;
            cx.resources.aca.y = cx.resources.aca.y/ *cx.resources.R;
            cx.resources.aca.z = cx.resources.aca.z/ *cx.resources.R;

            if *cx.resources.firstSample {
                cx.resources.RwEst.x = cx.resources.aca.x;             
                cx.resources.RwEst.y = cx.resources.aca.y;
                cx.resources.RwEst.z = cx.resources.aca.z;
                *cx.resources.firstSample = false;
            } else {
                if (cx.resources.RwEst.z).abs() < 0.1 {
                    cx.resources.RwGyro.x = cx.resources.RwEst.x; // use previous value is z is too small
                    cx.resources.RwGyro.y = cx.resources.RwEst.y; // use previous value is z is too small
                    cx.resources.RwGyro.z = cx.resources.RwEst.z;
                } else { //z is not too small
                    // gyrorate is dps
                    cx.resources.Awz.y = (cx.resources.RwEst.y).atan2(cx.resources.RwEst.z) * (180./PI);  // get angle convert to deg
                    cx.resources.Awz.y += cx.resources.gya.y * f32(samples)/f32(SAMPLE_FREQ);  //# of samples / sampling rate

                    cx.resources.Awz.x = (cx.resources.RwEst.x).atan2(cx.resources.RwEst.z) * (180./PI);  // get angle convert to deg
                    cx.resources.Awz.x += cx.resources.gya.x * f32(samples)/f32(SAMPLE_FREQ);  //# of samples / sampling rate
                

                let signRzGyro = if (cx.resources.Awz.x * (PI/180.)).cos() >= 0. {1.} else {-1.};

                cx.resources.RwGyro.x = (cx.resources.Awz.x * PI / 180.).sin();
                cx.resources.RwGyro.x /= ( 1. + squared((cx.resources.Awz.x * PI / 180.).cos()) * squared((cx.resources.Awz.y * PI / 180.).tan()) ).sqrt();
                cx.resources.RwGyro.y = (cx.resources.Awz.y * PI / 180.).sin();
                cx.resources.RwGyro.y /= ( 1. + squared((cx.resources.Awz.y * PI / 180.).cos()) * squared((cx.resources.Awz.x * PI / 180.).tan()) ).sqrt(); 

                cx.resources.RwGyro.z = signRzGyro * (1. - squared(cx.resources.RwGyro.x) - squared(cx.resources.RwGyro.y)).sqrt();
                
                }  //z is too msall end
                // add for statement back
                cx.resources.RwEst.x = (cx.resources.aca.x + *cx.resources.wGyro * cx.resources.RwGyro.x) / (1. + *cx.resources.wGyro);
                cx.resources.RwEst.y = (cx.resources.aca.y + *cx.resources.wGyro * cx.resources.RwGyro.y) / (1. + *cx.resources.wGyro);
                cx.resources.RwEst.z = (cx.resources.aca.z + *cx.resources.wGyro * cx.resources.RwGyro.z) / (1. + *cx.resources.wGyro);

                *cx.resources.R = (cx.resources.RwEst.x * cx.resources.RwEst.x + cx.resources.RwEst.y * cx.resources.RwEst.y + cx.resources.RwEst.z * cx.resources.RwEst.z).sqrt();
                cx.resources.RwEst.x = cx.resources.RwEst.x/ *cx.resources.R;
                cx.resources.RwEst.y = cx.resources.RwEst.y/ *cx.resources.R;
                cx.resources.RwEst.z = cx.resources.RwEst.z/ *cx.resources.R;

                
            }  // end else not first sample

    let temp = cx.resources.pid_control.next_control_output(cx.resources.RwEst.y);
    let local_db_RwEst_y = cx.resources.RwEst.y;
    let mut output = temp.output;
        
    let mut balance = 1.0;

        if cx.resources.RwEst.y.abs() < 0.01 {
            balance = 0.0;
        }

        if output > 0. {
            output = output + *cx.resources.dc_deadzone * balance;
            cx.resources.m1.cw();
            cx.resources.m2.cw();
        } else {
            output = (-*cx.resources.dc_deadzone * balance + output).abs();  //output from PID is negative
            cx.resources.m1.ccw();
            cx.resources.m2.ccw();
        }

        if (cx.resources.RwEst.y).abs() > 0.6 {
            output = 0.0;
        }

            cx.resources.m1.duty((output + *cx.resources.motor_offset) as u16);
            cx.resources.m2.duty((output - *cx.resources.motor_offset) as u16);
            *cx.resources.avg_cnt = 0.0;

        let mut data = [0; TX_SZ - 2];
        LE::write_f32(&mut data[0..4], cx.resources.RwEst.y);
        LE::write_f32(&mut data[4..8], *cx.resources.dc_deadzone);
        LE::write_f32(&mut data[8..12], temp.output);
        LE::write_f32(&mut data[12..16], output);
        
        // terminate last transfer
        let (buf, c, tx) = match cx.resources.TX.take().unwrap() {
            Either::Left((buf, c, tx)) => (buf, c, tx),
            Either::Right(transfer) => transfer.wait(),
        };
         cobs::encode(&data, buf);
         *cx.resources.TX = Some(Either::Right(tx.write_all(buf, c)));

    }  // end if sampled > 6
}
    #[task(binds = DMA1_CH6, resources = [RX, ON, pid_control, wGyro, motor_offset])]
    fn rx_dat(cx: rx_dat::Context){
        let (buf, c, rx) = match cx.resources.RX.take().unwrap(){
            Either::Left((buf, c, rx)) => (buf, c, rx),
            Either::Right(transfer) => transfer.wait(),
        };

        cx.resources.pid_control.kp = LE::read_f32(&buf[0..4]);
        cx.resources.pid_control.ki = LE::read_f32(&buf[4..8]);
        cx.resources.pid_control.kd = LE::read_f32(&buf[8..12]);
        *cx.resources.wGyro = LE::read_f32(&buf[12..16]);
        cx.resources.pid_control.setpoint = LE::read_f32(&buf[16..20]);
        *cx.resources.motor_offset = LE::read_f32(&buf[20..24]);
        cx.resources.pid_control.reset_integral_term();
        //cx.reset_integra
        *cx.resources.RX = Some(Either::Right(rx.read_exact(buf, c)));
        *cx.resources.ON = true;
    }
};

fn squared(x:f32) -> f32{
    x * x
}
