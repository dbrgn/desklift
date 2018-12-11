#![deny(unsafe_code)]
#![no_main]
#![no_std]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger


use cortex_m_semihosting::hprintln;
use nb::block;
use rtfm::app;
use stm32f103xx::Interrupt;
use stm32f103xx_hal::prelude::*;
use stm32f103xx_hal::serial::{Serial, Event, Tx, Rx};

#[app(device = stm32f103xx)]
const APP: () = {
    /// Serial peripheral
    static mut TX: Tx<stm32f103xx::USART1> = ();
    static mut RX: Rx<stm32f103xx::USART1> = ();

    /// Initialiation happens here.
    ///
    /// The init function will run with interrupts disabled and has exclusive
    /// access to Cortex-M and device specific peripherals through the `core`
    /// and `device` variables, which are injected in the scope of init by the
    /// app attribute.
    #[init]
    fn init() {
        hprintln!("init").unwrap();

        // Cortex-M peripherals
        let _core: rtfm::Peripherals = core;

        // Device specific peripherals
        let device: stm32f103xx::Peripherals = device;

        // Get reference to peripherals required for USART
        let mut rcc = device.RCC.constrain();
        let mut afio = device.AFIO.constrain(&mut rcc.apb2);
        let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
        let mut flash = device.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        // Set up serial communication
        let tx_pin = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
        let rx_pin = gpiob.pb7;
        let mut serial = Serial::usart1(
            device.USART1,
            (tx_pin, rx_pin),
            &mut afio.mapr,
            9600.bps(),
            clocks,
            &mut rcc.apb2,
        );

        // Enable USART1 RX interrupt
        serial.listen(Event::Rxne);

        let (tx, rx) = serial.split();
        TX = tx;
        RX = rx;
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

    #[interrupt(resources = [RX], spawn = [move_table])]
    fn USART1() {
        //hprintln!("USART1 interrupt called").unwrap();
        let byte_read = block!(resources.RX.read()).expect("Could not read byte");
        spawn.move_table(byte_read).expect("Could not spawn move_table task");
    }

    #[task(capacity = 64)]
    fn move_table(command: u8) {
        //hprintln!("move: {}", command).unwrap();
    }

    // RTFM requires that free interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn SPI1();
        fn SPI2();
    }

};
