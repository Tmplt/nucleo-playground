//! A simple no-HAL example for blinking an LED, wrapped in RTFM
//!
//! # Usage
//! `cargo [build|run] --example blinking`
#![no_main]
#![no_std]

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rtfm::{app, Threshold};

// Configuration
const FREQUENCY: u32 = 1; // Hz

// Tasks & Resources
app! {
    device: stm32f401,

    resources: {
        static LED_ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: toggle_led,
            resources: [LED_ON],
        },
    },
}

// Initialization phase
fn init(p: init::Peripherals, _r: init::Resources) {
    // power on GPIOA
    p.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

    // configure PA5 as output
    p.GPIOA.moder.modify(|_, w| unsafe { w.moder5().bits(1) });

    // Clock settings
    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(64_000_000 / FREQUENCY); // Sets blink freq.
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

// Idle loop
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// Tasks
// Toogle the state of the LED
fn toggle_led(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.LED_ON = !**r.LED_ON;

    // TODO
}

fn set_led(gpioa: &GPIOA, state: bool) {
    gpioa.odr.modify(|_, w| w.odr5().bit(state));
}
