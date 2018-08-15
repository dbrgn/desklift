#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)] extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f103xx;
extern crate stm32f103xx_hal as hal;

use hal::prelude::*;
use rt::ExceptionFrame;

// Entry point
entry!(main);

fn main() -> ! {
    let p = stm32f103xx::Peripherals::take().unwrap();

    // Get reference to GPIO peripherals
    let mut rcc = p.RCC.constrain();
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);

    // Output pins
    let pin_up = gpiob.pb4;
    let pin_down = gpiob.pb5;
    let pin_led = gpioc.pc13;

    // Set up output pins
    pin_up.into_push_pull_output(&mut gpiob.crl);
    pin_down.into_push_pull_output(&mut gpiob.crl);

    // Turn on LED
    pin_led.into_push_pull_output(&mut gpioc.crh);

    loop { }
}

// Define the hard fault handler
exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// Define the default exception handler
exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
