#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::{entry, pre_init};
use cortex_m_semihosting::hprintln;

#[pre_init]
unsafe fn enable_sram() {
    *(0x4000_0220 as *mut u32) = 0x38;
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    loop {}
}
