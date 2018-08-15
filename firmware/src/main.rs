#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)] extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f103xx;
extern crate stm32f103xx_hal as hal;

use hal::delay::Delay;
use hal::prelude::*;
use rt::ExceptionFrame;

// Entry point
entry!(main);

fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Get reference to GPIO peripherals
    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Set up timer
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set up output pins
    let _pin_up = gpiob.pb4.into_push_pull_output(&mut gpiob.crl);
    let _pin_down = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    let mut pin_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Main loop
    loop {
        pin_led.set_high();
        delay.delay_ms(1_000_u16);
        pin_led.set_low();
        delay.delay_ms(1_000_u16);
    }
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
