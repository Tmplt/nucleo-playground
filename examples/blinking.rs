//! A simple no-HAL example for blinking an LED
//!
//! # Usage
//! `cargo [build|run] --example blinking`

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f401::GPIOA;

fn led_is_on(gpioa: &GPIOA) -> bool {
    gpioa.odr.read().odr5().bit_is_set()
}

fn set_led(gpioa: &GPIOA, state: bool) {
    gpioa.odr.modify(|_, w| w.odr5().bit(state));
}

fn wait(i: u32) {
    for _ in 0..i {
        cortex_m::asm::nop();
    }
}

#[entry]
fn main() -> ! {
    // Take device peripherals
    let dp = stm32f401::Peripherals::take().unwrap();

    // power of GPIOA
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

    // configure PA5 as output
    dp.GPIOA.moder.modify(|_, w| unsafe { w.moder5().bits(1) });

    loop {
        if led_is_on(&dp.GPIOA) {
            // Turn the LED off
            set_led(&dp.GPIOA, false);
        } else {
            // Turn the LED on
            set_led(&dp.GPIOA, true);
        }

        // Wait a little bit
        wait(10_000_u32);
    }
}
