# Getting started
1. Make sure you have [Rust, Cargo](https://www.rust-lang.org/learn/get-started), and [rustup](https://rustup.rs/) installed on your device
2. Install nightly toolchain: `rustup toolchain install nightly`
3. Install rustfmt nightly (if needed): `rustup component add rustfmt --toolchain nightly`

# Style Guidelines
Before committing, run rustfmt nightly:  
`cargo +nightly fmt`
