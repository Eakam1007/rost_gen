# Getting started
1. Make sure you have [Rust, Cargo](https://www.rust-lang.org/learn/get-started), and [rustup](https://rustup.rs/) installed on your device
2. Install nightly toolchain: `rustup toolchain install nightly`
3. Install rustfmt nightly: `rustup component add rustfmt --toolchain nightly`
4. Install clippy: `rustup component add clippy`

# Style Guidelines
It is recommended to use VSCode and use the recommended extensions.  This will ensure that rustfmt and clippy are run automatically on save.
- Run clippy, and fix any warnings or errors:  
  `cargo clippy`
- Run rustfmt nightly:  
  `cargo +nightly fmt`

# Tests
## Running tests
In the project directory, run tests by running `cargo test`  
To run a single test, use `cargo test name_of_test`  
For example, to run the creates_html_file test, use `cargo test creates_html_file`

## Adding tests
Tests should be added to the test module at the end of the main.rs file.  
_Note: This file should eventually be split into multiple smaller files, which would allow better test organization._
