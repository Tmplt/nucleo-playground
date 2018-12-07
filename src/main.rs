#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let _x = 42;

    loop {}
}

// #[panic_handler]
// fn panic(_panic: &PanicInfo<'_>) -> ! {
//     loop {}
// }
