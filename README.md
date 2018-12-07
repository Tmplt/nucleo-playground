Nucleo Playground
===

This repository is a playground for running embedded Rust code on the ST STM32F401 Nucleo.
At present, a very simple Cortex-M binary is built and linked via `cargo build` and then programmed unto the Nucleo via `cargo run`.
Later on I'll see about playing around with LEDs, serial interfaces, GPIO pins and other peripherals.
Crates will be added only then they are required.

Setup
====

Follow [this setup guide](https://docs.rust-embedded.org/discovery/03-setup/index.html).

Flash Instructions
===

1. Run `openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg` in a separate terminal emulator.
2. Run `cargo run`; you will be dropped into a GDB shell at the start of your `main` function.
