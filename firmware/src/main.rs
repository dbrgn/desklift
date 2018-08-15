#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)] extern crate cortex_m_rt as rt;
#[macro_use(block)] extern crate nb;
extern crate panic_semihosting;
extern crate stm32f103xx;
extern crate stm32f103xx_hal as hal;

use hal::delay::Delay;
use hal::prelude::*;
use hal::serial::Serial;
use rt::ExceptionFrame;

// Entry point
entry!(main);

fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Get reference to GPIO peripherals
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

    // Set up serial communication
    let tx_pin = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx_pin = gpiob.pb7;
    let serial = Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );
    let (mut tx, _rx) = serial.split();

    // Main loop
    loop {
        block!(tx.write(b'H')).ok();
        block!(tx.write(b'i')).ok();
        block!(tx.write(b'\n')).ok();
        pin_led.set_high();
        delay.delay_ms(1_000_u16);

        block!(tx.write(b'L')).ok();
        block!(tx.write(b'o')).ok();
        block!(tx.write(b'\n')).ok();
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
