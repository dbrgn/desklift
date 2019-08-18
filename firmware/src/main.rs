#![deny(unsafe_code)]
#![no_main]
#![cfg_attr(not(test), no_std)]

// Panicking behavior
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

#[macro_use]
extern crate cortex_m_rt;

use cortex_m_semihosting::hprintln;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{prelude::*, pac};

#[entry]
fn main() -> ! {
    hprintln!("init").unwrap();

    let device: pac::Peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = device.RCC.constrain();
    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_alternate_push_pull(&mut gpioc.crh);
    led.set_low().unwrap();

    hprintln!("loop").unwrap();
    loop {
    }
}
