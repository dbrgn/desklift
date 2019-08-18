//! Desk lift firmware
//!
//! Serial protocol:
//!
//! - Baudrate 115'200
//! - Every command is a single signed byte
//! - Positive values move up, negative values move down
//! - The value must be multiplied by 10 ms to get the move duration
//!
//! Physical wiring:
//!
//! - Serial communication over USART1 (PB6, PB7)
//! - "Up" connected to PB4
//! - "Down" connected to PB5

#![deny(unsafe_code)]
#![no_main]
#![cfg_attr(not(test), no_std)]

// Panicking behavior
#[cfg(not(feature = "debug"))]
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
#[cfg(feature = "debug")]
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

#[cfg(feature = "debug")]
use cortex_m_semihosting::hprintln;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use rtfm::app;
use stm32f1xx_hal::{prelude::*, pac};
use stm32f1xx_hal::delay::{Delay};
use stm32f1xx_hal::gpio::{Output, PushPull, OpenDrain, State, gpiob, gpioc};
use stm32f1xx_hal::serial::{self, Serial, Event, Tx, Rx, Parity, StopBits};

use desklift_command::{Command, Direction};

#[app(device = stm32f1::stm32f103)]
const APP: () = {
    // Serial pins
    static mut TX: Tx<pac::USART1> = ();
    static mut RX: Rx<pac::USART1> = ();

    // GPIO pins
    static mut GPIO_UP: gpiob::PB4<Output<PushPull>> = ();
    static mut GPIO_DOWN: gpiob::PB5<Output<PushPull>> = ();

    // LED pins
    static mut LED: gpioc::PC13<Output<OpenDrain>> = ();

    // Delay peripheral
    static mut DELAY: Delay = ();

    /// Initialiation happens here.
    ///
    /// The init function will run with interrupts disabled and has exclusive
    /// access to Cortex-M and device specific peripherals through the `core`
    /// and `device` variables, which are injected in the scope of init by the
    /// app attribute.
    #[init]
    fn init() {
        #[cfg(feature = "debug")]
        hprintln!("init").unwrap();

        // Cortex-M peripherals
        let core: rtfm::Peripherals = core;

        // Device specific peripherals
        let device: pac::Peripherals = device;

        // Get reference to peripherals required for USART
        let mut rcc = device.RCC.constrain();
        let mut afio = device.AFIO.constrain(&mut rcc.apb2);
        let gpioa = device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let mut flash = device.FLASH.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        // Disable JTAG to free up pins PA15, PB3 and PB4 for normal use
        let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Set up serial communication
        let tx_pin = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
        let rx_pin = gpiob.pb7;
        let mut serial = Serial::usart1(
            device.USART1,
            (tx_pin, rx_pin),
            &mut afio.mapr,
            serial::Config {
                baudrate: 115_200.bps(),
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
            },
            clocks,
            &mut rcc.apb2,
        );

        // Enable USART1 RX interrupt
        serial.listen(Event::Rxne);

        // Split serial ports
        let (tx, rx) = serial.split();

        // Set up GPIO outputs
        let mut gpio_up = pb4.into_push_pull_output(&mut gpiob.crl);
        let mut gpio_down = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
        gpio_up.set_low().unwrap();
        gpio_down.set_low().unwrap();

        // Set up delay provider
        let mut delay = Delay::new(core.SYST, clocks);

        // Set up status LED and blink twice
        let mut led = gpioc.pc13.into_open_drain_output_with_state(&mut gpioc.crh, State::High);
        led.set_low().unwrap();
        delay.delay_ms(200u16);
        led.set_high().unwrap();
        delay.delay_ms(200u16);
        led.set_low().unwrap();
        delay.delay_ms(200u16);
        led.set_high().unwrap();

        // Assign resources
        TX = tx;
        RX = rx;
        GPIO_UP = gpio_up;
        GPIO_DOWN = gpio_down;
        LED = led;
        DELAY = delay;
    }

    // /// The runtime will execute the idle task after init. Unlike init, idle
    // /// will run with interrupts enabled and it's not allowed to return so it
    // /// runs forever.
    // #[idle]
    // fn idle() -> ! {
    //     #[cfg(feature = "debug")]
    //     hprintln!("idle").unwrap();
    //
    //     // Busy-loop. In production, remove the `idle` function to fall back to
    //     // the default implementation which puts the device to sleep.
    //     loop {}
    // }

    #[interrupt(resources = [RX], spawn = [move_table])]
    fn USART1() {
        let byte_read: u8 = block!(resources.RX.read()).expect("Could not read byte");
        let command = Command::from_u8(byte_read);
        spawn.move_table(command).expect("Could not spawn move_table task");
    }

    #[task(capacity = 64, resources = [GPIO_UP, GPIO_DOWN, DELAY, LED])]
    fn move_table(command: Command) {
        #[cfg(feature = "debug")]
        hprintln!("Move: {}", command).unwrap();
        match command.get_direction() {
            Direction::Up => resources.GPIO_UP.set_high().unwrap(),
            Direction::Down => resources.GPIO_DOWN.set_high().unwrap(),
        };
        resources.LED.set_low().unwrap();
        resources.DELAY.delay_ms(command.get_ms());
        resources.GPIO_UP.set_low().unwrap();
        resources.GPIO_DOWN.set_low().unwrap();
        resources.LED.set_high().unwrap();
    }

    // RTFM requires that free interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn SPI1();
        fn SPI2();
    }

};
