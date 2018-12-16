# Desk Lift Firmware


# Debugging

Use Rust 1.31+.

Add rust-std component:

    rustup target add thumbv7m-none-eabi

Start OpenOCD:

    ./openocd.sh

Build and run binary:

    cargo run --release

Connect via miniterm:

    miniterm.py -e --raw /dev/ttyUSB0 115200


# Release Builds

Like debugging, but without default features (this will exclude things like
semihosting):

    cargo build --release --no-default-features


# Testing

Run the test script:

    ./test.sh


# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
