//! Example of using a number of timer channels in PWM mode.
//! Target board: STM32F3DISCOVERY
#![no_std]
#![no_main]

//extern crate f3;

use panic_semihosting as _;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;
use embedded_hal::PwmPin;
use embedded_hal::digital::v1_compat::OldOutputPin;

use byteorder::{ByteOrder, BE, LE};

use stm32f3::stm32f303;
use stm32f3xx_hal::flash::FlashExt;
use stm32f3xx_hal::gpio::GpioExt;
use stm32f3xx_hal::pwm::{tim16, tim2, tim3, tim8, tim4};
use stm32f3xx_hal::rcc::RccExt;
use stm32f3xx_hal::time::U32Ext;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::spi::Spi;
use stm32f3xx_hal::timer::Timer;
use stm32f3xx_hal::serial::{Rx, Serial, Tx};
use f3::l3gd20;
use cast::{f32, i32};
/*use postcard::{
   serialize_with_flavor,
   flavors::{Cobs, Slice},
};*/

use nb::block;  // used with serial data transfer


#[entry]
fn main() -> ! {
    // Get our peripherals
    let dp = stm32f303::Peripherals::take().unwrap();

    //let mut dc = 20;

    // Configure our clocks
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    // Prep the pins we need in their correct alternate function
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
//    let pa1 = gpioa.pa1.into_af1(&mut gpioa.moder, &mut gpioa.afrl);  
  //  let pa4 = gpioa.pa4.into_af2(&mut gpioa.moder, &mut gpioa.afrl);  (TIM3_CH2)
   // let pa9 = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);  ()
   // let pa10 = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

  // Setup SPI for gyroscope
      let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
      let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
      let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

      /*let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );*/

    // nss to enable Gyroscope SPI
      let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
      nss.set_high().unwrap();

    // grab gyroscope output
      //let mut l3gd20 = L3gd20::new(spi, OldOutputPin::from(nss)).unwrap();
      // sanity check: the WHO_AM_I register always contains this value
      assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

     // l3gd20.set_odr(l3gd20::Odr::Hz190);
     // l3gd20.set_scale(l3gd20::Scale::Dps250);


   // Serial USART pins
      let pa9 = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
      let pa10 = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
      let pa11 = gpioa.pa11.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

      let serial = Serial::usart1(
          dp.USART1,
          (pa9, pa10),
          115200.bps(),
          clocks,
          &mut rcc.apb2
      );

      let (mut tx, mut rx) = serial.split();

      // tx.write(0x00).ok().unwrap();

     
    /*  for byte in "AT+NAME=Cent\r\n".as_bytes() {
        while block!(tx.write(*byte)).is_err(){};
      }*/
     for byte in "AT+UART=230400,1,0\r\n".as_bytes() {
        while block!(tx.write(*byte)).is_err(){};
      }

     // for byte in "AT+PSWD=0101\r\n".as_bytes() {
     //   while block!(tx.write(*byte)).is_err(){};
    //  }

      //block!(tx.write("AT+UART=115200,1,0".as_bytes())).ok();



      /*let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
      let buffer = &mut [0u8; 32];
      let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>(
      data,
      Cobs::try_new(Slice::new(buffer)).unwrap(),
      ).unwrap();*/

    // TIM3
    //
    // A four channel general purpose timer that's broadly available
 //   let tim3_channels = tim3(
//        dp.TIM3,
//        1280,    // resolution of duty cycle
///        100.hz(), // frequency of period
//        &clocks, // To get the timer's clock speed
//    );


  //  let mut tim3_ch2 = tim3_channels
  //      .1
  //      .output_to_pa4(pa4);
   //     .output_to_pa7(pa7)
    //    .output_to_pb5(pb5);
  //  tim3_ch2.set_duty(tim3_ch2.get_max_duty() / 4 ); // 7.5% duty cyle
   // tim3_ch2.enable();

    // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // gyroscope and the temperature sensor readings
    

    

  //  let mut degy = l3gd20.scale().unwrap().degrees(_y);
   // let mut hoe_2;
   // let mut hoe;
   
//   block!(tx.write(b'A')).ok();
//   block!(tx.write(b'A')).ok();

let mut buf = [0u8; 2];
let mut angle = 0.;

let mut total = 0.;
let mut count = 0.;
let mut average = 0.;


let mut timer = Timer::tim2(dp.TIM2, 380.hz(), clocks, &mut rcc.apb1);
let mut ar_bias_y = 0;

for _ in 0..256 {
  
  let ar = l3gd20.gyro().unwrap();
  ar_bias_y += i32(ar.y);


}

let ar_bias_y = (ar_bias_y / 256) as i16;

   

    loop {

//      block!(tx.write(b'A')).ok();
//      block!(tx.write(b'A')).ok();

 let mut gyro_status = l3gd20.status().unwrap();


      if (gyro_status.new_data) {

        let mut _m = l3gd20.all().unwrap();

      let gx = _m.gyro.x;
      let gy = _m.gyro.y;
      let gz = _m.gyro.z;

    let gy_deg = l3gd20.scale().unwrap().degrees(gy-ar_bias_y);

      total += gy as f32;
      count += 1.;
      

      let mut gy_pos = gy_deg as f32 * 8.75;
      angle += gy_pos / 190.;

      average = total/count * 1000000.;

        LE::write_i16(&mut buf[0..2], angle as i16);

      } else {

        LE::write_i16(&mut buf[0..2], 0);


      }

    

    //  block!(tx.write(b'A')).ok();

    //  buf[0] = 170; //AA
    //  buf[1] = 187; //BB
     // LE::write_i8(&mut buf[1], 3);
    //  LE::write_i16(&mut buf[2..4], gy);
    //  LE::write_i16(&mut buf[4..6], gz);
    //  LE::write_i16(&mut buf[6..8], gz);
   //   LE::write_i16(&mut buf[8..10], gz);

      



  //    BE::write_i16(&mut buf[6..7], 5);

   //   BE::write_i16(&mut buf[2..4], gy);
   //   BE::write_i16(&mut buf[4..6], gz);



     

    for byte in &buf {
      while block!(tx.write(*byte)).is_err(){};
    }
 //   let byte = block!(rx.read()).unwrap();

  // block!(tx.write(write_i16(5))).ok();

    }
    // loop {
    //     let t = 0;
    //     _m = l3gd20.all().unwrap();
    //     _y = _m.gyro.y;

    //     degy = l3gd20.scale().unwrap().degrees(_y) * 1280.0/180.0;

    //     if (degy < 0.0) {
    //         hoe = -1.0 * degy;
    //     } else {
    //        hoe = degy;
    //     }

    //     hoe_2 = hoe as u16;
    //     hoe_2 += 1;

    //     if (hoe_2 > 100){
    //         hoe_2 = 100;
    //     }

    //     for x in 1..10 {
    //         tim3_ch2.set_duty(tim3_ch2.get_max_duty() / x );
    //     }
    //    //let joy: u16 = degy;

    //  //   tim3_ch2.set_duty(hoe_2); // 7.5% duty cyle
    //  //   asm::bkpt();
     
    //  let t = 0;
    // }
}
