[![Rust](https://github.com/Ferryistaken/operating_system/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Ferryistaken/operating_system/actions/workflows/rust.yml)
![Languages](https://img.shields.io/github/languages/count/ferryistaken/operating_system)
![Issues](https://img.shields.io/github/issues/Ferryistaken/operating_system)
![Forks](https://img.shields.io/github/forks/Ferryistaken/operating_system)
![Stars](https://img.shields.io/github/stars/Ferryistaken/operating_system)
![License](https://img.shields.io/github/license/Ferryistaken/operating_system)
![Size](https://img.shields.io/github/languages/code-size/ferryistaken/operating_system)
![Lines](https://img.shields.io/tokei/lines/github/ferryistaken/operating_system)

# Bare metal program written entirely in rust

This was created to learn about bare metal programming, the x86_64 architecture, and rust in general

# How to run it

## Requirements

Here are the required packages:

Rust:

-   rust nightly compiler >= `v1.53.0` (run `rustup toolchain install nightly`)
-   `rustup run nightly rustup component add llvm-tools-preview rust-src`
-   `cargo install bootimage`

Operating System:

-   `qemu-system-x86_64`
-   `cargo`
-   `rustup`

```shell
rustup run nightly cargo build --release
```

Now you will have the image under `target/x86_64-test_os/debug/bootimage-test_os.bin`.  
To run in in `qemu` use this command:

```shell
qemu-system-x86_64 -drive format=raw,file=target/x86_64-test_os/debug/bootimage-test_os.bin
```
