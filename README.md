# Rusty

Gonna try to make an operating system entrirely written in rust in this repository

## Building

### The OS

- install rustup [here](https://rustup.rs/)
- navigate to the repository where this code is and run 'rustup toolchain install nightly'
- run 'rustup override set nightly'
- run 'rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc'
- finally run 'cargo build' and the binary should be in /target/x86_64-rusty/debug

### The bootloader

- install NASM [here](https://www.nasm.us/)
- navigate to the directory where NASM installed
- create a new empty file called 'boot.com'
- run  nasm -f bin /(Directory where this was extracted to)/boot.asm -o /(Directory where this was extracted to)/boot.com
