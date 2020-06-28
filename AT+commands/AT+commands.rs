/* way 1:

Step 1: Input low level to PIN34->EN PIN. (it can also be floating)
Step 2: Supply power to the module. 
Step 3: Input high level to the EN PIN(PIN34)(you can press the button this do this action,if you buy HC-05 from DSD TECH) 
Then the module will enter to AT mode. The baud rate is as same as the communication time, default is  9600

 Connect with cable to 3V!!

ONLY 1 COMMAND AT A TIME.

*/



//! Target board: STM32F3DISCOVERY
#![no_std]
#![no_main]

//extern crate f3;

use panic_semihosting as _;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;
use embedded_hal::PwmPin;
use embedded_hal::digital::v1_compat::OldOutputPin;

use stm32f3::stm32f303;
use stm32f3xx_hal::flash::FlashExt;
use stm32f3xx_hal::gpio::GpioExt;
use stm32f3xx_hal::pwm::{tim16, tim2, tim3, tim8};
use stm32f3xx_hal::rcc::RccExt;
use stm32f3xx_hal::time::U32Ext;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::spi::Spi;
use stm32f3xx_hal::serial::{Rx, Serial, Tx};
use l3gd20::L3gd20;
use postcard::{
   serialize_with_flavor,
   flavors::{Cobs, Slice},
};













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
    let pa4 = gpioa.pa4.into_af2(&mut gpioa.moder, &mut gpioa.afrl);
   // let pa9 = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
   // let pa10 = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

  // Setup SPI for gyroscope
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

    // nss to enable Gyroscope SPI
      let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
      nss.set_high().unwrap();

    // grab gyroscope output
      let mut l3gd20 = L3gd20::new(spi, OldOutputPin::from(nss)).unwrap();
      // sanity check: the WHO_AM_I register always contains this value
      assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);


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

     
      for byte in "AT+NAME=Centennial\r\n".as_bytes() {
        while block!(tx.write(*byte)).is_err(){};
      }
     for byte in "AT+UART=115200,1,0\r\n".as_bytes() {
        while block!(tx.write(*byte)).is_err(){};
      }

      for byte in "AT+PSWD=0101\r\n".as_bytes() {
        while block!(tx.write(*byte)).is_err(){};
      }

      //block!(tx.write("AT+UART=115200,1,0".as_bytes())).ok();

      let sent = b'A';


      

      let mut buf = [0u8; 32];

      let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];
      let buffer = &mut [0u8; 32];
      let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>(
      data,
      Cobs::try_new(Slice::new(buffer)).unwrap(),
      ).unwrap();

    loop {
    }

}
