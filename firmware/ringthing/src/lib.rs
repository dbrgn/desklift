//! A simple lock-free circular buffer / ring buffer implementation that can
//! hold 64 bytes.
//!
//! (Once const generics are available, this will become parametrizable.)
//!
//! Some inspiration: https://embedjournal.com/implementing-circular-buffer-embedded-c/

#![no_std]

mod index;
mod ringbuf;

pub use ringbuf::{RingBuf, RingBufError};
