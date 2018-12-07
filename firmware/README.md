# Desk Lift Firmware

# Building

Use Rust 1.31+.

Add rust-std component:

    rustup target add thumbv7em-none-eabihf

Start OpenOCD:

    ./openocd.sh

Build and run binary:

    cargo run --release --target thumbv7em-none-eabihf

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
