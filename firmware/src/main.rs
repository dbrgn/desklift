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

// Debug logging
#[cfg(feature = "debug")]
use cortex_m_semihosting::hprintln;

use cortex_m::asm::delay;
use embedded_hal::digital::v2::OutputPin;
use rtfm::app;
use stm32_usbd::{UsbBus, UsbBusType};
use stm32f1xx_hal::{prelude::*, pac};
use stm32f1xx_hal::delay::{Delay};
use stm32f1xx_hal::gpio::{Output, PushPull, OpenDrain, State, gpiob, gpioc};
use usb_device::bus;
use usb_device::device::UsbDevice;
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use desklift_command::{Command, Direction};

const ERRNO_GPIO_UPDN: u8 = 1;
const ERRNO_GPIO_LED: u8 = 2;
const ERRNO_GPIO_DEBUG: u8 = 3;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[app(device = stm32f1::stm32f103)]
const APP: () = {
    // GPIO pins
    static mut GPIO_UP: gpiob::PB4<Output<PushPull>> = ();
    static mut GPIO_DOWN: gpiob::PB5<Output<PushPull>> = ();

    // LED pins
    static mut LED: gpioc::PC13<Output<OpenDrain>> = ();

    // Delay peripheral
    static mut DELAY: Delay = ();

    // USB
    static mut USB_DEV: UsbDevice<'static, UsbBusType> = ();
    static mut USB_SERIAL: SerialPort<'static, UsbBusType> = ();

    /// Initialiation happens here.
    ///
    /// The init function will run with interrupts disabled and has exclusive
    /// access to Cortex-M and device specific peripherals through the `core`
    /// and `device` variables, which are injected in the scope of init by the
    /// app attribute.
    #[init]
    fn init() {
        static mut USB_BUS: Option<bus::UsbBusAllocator<UsbBusType>> = None;

        #[cfg(feature = "debug")]
        hprintln!("init").unwrap();

        // Cortex-M peripherals
        let core: rtfm::Peripherals = core;

        // Device specific peripherals
        let device: pac::Peripherals = device;

        // Get reference to peripherals required for USART
        let mut rcc = device.RCC.constrain();
        let mut afio = device.AFIO.constrain(&mut rcc.apb2);
        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let mut flash = device.FLASH.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(48.mhz())
            .pclk1(24.mhz())
            .freeze(&mut flash.acr);
        assert!(clocks.usbclk_valid());

        // Disable JTAG to free up pins PA15, PB3 and PB4 for normal use
        let (_pa15, _pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // BluePill board has a pull-up resistor on the USB D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        usb_dp.set_low().unwrap();
        delay(clocks.sysclk().0 / 100);

        // Initialize USB bus
        let usb_dm = gpioa.pa11;
        let usb_dp = usb_dp.into_floating_input(&mut gpioa.crh);
        *USB_BUS = Some(UsbBus::new(device.USB, (usb_dm, usb_dp)));

        // Initialize serial port over USB
        let usb_serial = SerialPort::new(USB_BUS.as_ref().unwrap());

        // Create USB device
        let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
            .manufacturer("dbrgn")
            .product("Desklift v3")
            .serial_number(VERSION)
            .device_class(USB_CLASS_CDC)
            .build();

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
        GPIO_UP = gpio_up;
        GPIO_DOWN = gpio_down;
        LED = led;
        DELAY = delay;
        USB_DEV = usb_dev;
        USB_SERIAL = usb_serial;
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

    #[interrupt(resources = [USB_DEV, USB_SERIAL, GPIO_UP, GPIO_DOWN, DELAY, LED])]
    fn USB_LP_CAN_RX0() {
        usb_poll(
            &mut resources.USB_DEV,
            &mut resources.USB_SERIAL,
            &mut resources.GPIO_UP,
            &mut resources.GPIO_DOWN,
            &mut resources.DELAY,
            &mut resources.LED,
        );
    }

    // RTFM requires that free interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn SPI1();
        fn SPI2();
    }
};

fn usb_poll<B: bus::UsbBus>(
    usb_dev: &mut UsbDevice<'static, B>,
    serial: &mut SerialPort<'static, B>,
    gpio_up: &mut gpiob::PB4<Output<PushPull>>,
    gpio_down: &mut gpiob::PB5<Output<PushPull>>,
    delay: &mut Delay,
    led: &mut gpioc::PC13<Output<OpenDrain>>,
) {
    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    let mut buf = [0u8; 8];

    match serial.read(&mut buf) {
        Ok(count) if count > 0 => {
            // Echo back in upper case
            for byte in &buf[0..count] {
                let command = Command::from_u8(*byte);
                match move_table(command, gpio_up, gpio_down, delay, led) {
                    Ok(_) => serial.write(&[0]).ok(),
                    Err(errno) => serial.write(&[errno]).ok(),
                };
            }
        }
        _ => {}
    }
}

fn move_table(
    command: Command,
    gpio_up: &mut gpiob::PB4<Output<PushPull>>,
    gpio_down: &mut gpiob::PB5<Output<PushPull>>,
    delay: &mut Delay,
    led: &mut gpioc::PC13<Output<OpenDrain>>,
) -> Result<(), u8> {
    #[cfg(feature = "debug")]
    hprintln!("Move: {}", command).map_err(|_| ERRNO_GPIO_DEBUG)?;
    match command.get_direction() {
        Direction::Up => gpio_up.set_high().map_err(|_| ERRNO_GPIO_UPDN)?,
        Direction::Down => gpio_down.set_high().map_err(|_| ERRNO_GPIO_UPDN)?,
    };
    led.set_low().map_err(|_| ERRNO_GPIO_LED)?;
    delay.delay_ms(command.get_ms());
    gpio_up.set_low().map_err(|_| ERRNO_GPIO_UPDN)?;
    gpio_down.set_low().map_err(|_| ERRNO_GPIO_UPDN)?;
    led.set_high().map_err(|_| ERRNO_GPIO_LED)?;
    Ok(())
}
