#![feature(custom_attribute)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;
use rtfm::app;

#[app(device = stm32f103xx)]
const APP: () = {
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

        hprintln!("init").unwrap();
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

    #[interrupt(spawn = [move_table])]
    fn USART1() {
        hprintln!("USART1 interrupt called").unwrap();

        let byte_read = 42u8; // TODO

        spawn.move_table(byte_read).unwrap();
    }

    #[task(capacity = 64)]
    fn move_table(command: u8) {
        hprintln!("move: {}", command).unwrap();
    }

    // RTFM requires that free interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn SPI1();
        fn SPI2();
    }

};
