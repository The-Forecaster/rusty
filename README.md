# Rusty

Gonna try to make an operating system entrirely written in rust in this repository

## Building

1. install rustup [here](https://rustup.rs/)
2. navigate to the repository where this code is and run 'rustup toolchain install nightly'
3. run 'rustup override set nightly'
4. run 'rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc'
5. finally run 'cargo build' and the binary should be in /target/x86_64-rusty/debug/
