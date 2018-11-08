#![feature(custom_attribute)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use ringthing::RingBuf;
use rtfm::app;
use stm32f103xx::Interrupt;

#[app(device = stm32f103xx)]
const APP: () = {
    static mut SERIAL_BUF: RingBuf = RingBuf::new();

    /// Initialiation happens here.
    ///
    /// The init function will run with interrupts disabled and has exclusive
    /// access to Cortex-M and device specific peripherals through the `core`
    /// and `device` variables, which are injected in the scope of init by the
    /// app attribute.
    #[init]
    fn init() {
        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;

        // Device specific peripherals
        let _device: stm32f103xx::Peripherals = device;

        //rtfm::pend(Interrupt::USART1);

        hprintln!("init").unwrap();
        debug::exit(debug::EXIT_SUCCESS);
    }

    /// The runtime will execute the idle task after init. Unlike init, idle
    /// will run with interrupts enabled and it's not allowed to return so it
    /// runs forever.
    #[idle]
    fn idle() -> ! {
        hprintln!("idle").unwrap();

        // Busy-loop. In production, remove the `idle` function to fall back to
        // the default implementation which puts the device to sleep.
        loop {}
    }

    #[interrupt(resources = [SERIAL_BUF])]
    fn USART1() {
        hprintln!("USART1 interrupt called").unwrap();
        resources.SERIAL_BUF.push(42).unwrap();
    }

};
