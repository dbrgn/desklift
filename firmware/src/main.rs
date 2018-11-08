//! Desk lift firmware
//!
//! Serial protocol:
//!
//! - Baudrate 9600
//! - Every command is a single signed byte
//! - Positive values move up, negative values move down
//! - The value must be multiplied by 0.1s to get the move duration

#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate nb;
extern crate panic_semihosting;
extern crate ringthing;
extern crate stm32f103xx;
extern crate stm32f103xx_hal as hal;

use core::fmt::Write;

use cortex_m::singleton;
use hal::delay::Delay;
use hal::prelude::*;
use hal::serial::{Serial, Event as SerialEvent};
use nb::block;
use rt::{ExceptionFrame, entry, exception};
use sh::hio;
use stm32f103xx::{Interrupt, interrupt};

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
struct Command(i8);

impl Command {
    fn from_u8(byte: u8) -> Self {
        Command(byte as i8)
    }

    fn get_ms(&self) -> u16 {
        (self.0 as u16) * 10
    }

    fn get_direction(&self) -> Direction {
        if self.0 < 0 {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}

//struct CommandReader {
//    rx: serial::Rx<stm32f103xx::USART1>,
//    dma_chan: hal::dma::dma1::C5,
//    buf: &'static [u8; 8],
//}
//
//impl CommandReader {
//    fn new(rx: serial::Rx<stm32f103xx::USART1>, dma_chan: hal::dma::dma1::C5) -> Self {
//        CommandReader {
//            rx,
//            dma_chan,
//            buf: singleton!(: [u8; 8] = [0; 8]).unwrap(),
//        }
//    }
//
//    fn await(&mut self) -> Result<Command, serial::Error> {
//        self.rx.read_exact(self.dma_chan, &mut self.buf).wait();
//        Ok(Command::from_u8(self.buf[0]))
//    }
//}

// Entry point
#[entry]
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut nvic = cp.NVIC;

    // Set up logging through semihosting
    let mut hstdout = hio::hstdout().unwrap();
    let mut hstderr = hio::hstderr().unwrap();
    writeln!(hstdout, "Initializing desklift...").unwrap();

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

    // Set up DMA
    let channels = dp.DMA1.split(&mut rcc.ahb);

    // Set up serial communication
    let tx_pin = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let rx_pin = gpiob.pb7;
    let mut serial = Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );
    serial.listen(SerialEvent::Rxne);
    let (mut tx, mut rx) = serial.split();

    // Enable USART interrupts
    nvic.enable(Interrupt::USART1);

    //let mut cmd_reader = CommandReader::new(rx, channels.5);
    let buf = singleton!(: [u8; 1] = [0; 1]).unwrap();
    let mut chan = channels.5;

    writeln!(hstdout, "Reading byte...").unwrap();
    let (buf_, _chan, _rx) = rx.read_exact(chan, buf).wait();
    writeln!(hstdout, "Buf: {:?}", &buf_);

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

        //match cmd_reader.await() {
        //    Ok(cmd) => {
        //        writeln!(hstdout, "Read byte: {:?}ms {:?}", cmd.get_ms(), cmd.get_direction());
        //    },
        //    Err(e) => {
        //        writeln!(hstderr, "Could not read byte: {:?}", e);
        //    },
        //};
    }
}

// Serial RX interrupt
interrupt!(USART1, usart1_rx);
fn usart1_rx() {
    panic!("USART1 RX");
}

// Define the hard fault handler
#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// Define the default exception handler
#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
