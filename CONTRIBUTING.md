# Getting started
1. Make sure you have [Rust, Cargo](https://www.rust-lang.org/learn/get-started), and [rustup](https://rustup.rs/) installed on your device
2. Install nightly toolchain: `rustup toolchain install nightly`
3. Install rustfmt nightly: `rustup component add rustfmt --toolchain nightly`
4. Install clippy: `rustup component add clippy`

# Style Guidelines
- Run clippy, and fix any warnings or errors:  
  `cargo clippy`
- Run rustfmt nightly:  
  `cargo +nightly fmt`
