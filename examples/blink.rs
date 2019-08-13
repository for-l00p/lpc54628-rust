#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::{entry, pre_init};

#[pre_init]
unsafe fn enable_sram() {
    *(0x4000_0220 as *mut u32) = 0x38;
}

#[entry]
fn main() -> ! {
    let dp = lpc54628::Peripherals::take().unwrap();

    let syscon = dp.SYSCON;

    syscon.asyncapbctrl.write(|w| w.enable().enabled());
    syscon.ahbclkctrl0.modify(|_, w| {
        w.iocon().set_bit();
        w.gpio3().set_bit();
        w
    });

    let iocon = dp.IOCON;
    iocon.pio314.write(|w| {
        w.func().alt0();
        w.mode().pull_up();
        w.digimode().digital();
        w.od().open_drain();
        w
    });

    let gpio = dp.GPIO;
    gpio.dirset[3].write(|w| unsafe { w.dirsetp().bits(1 << 14) });

    loop {
        gpio.b[3].b_[14].write(|w| w.pbyte().set_bit());
        for _ in 0..10_000 {}
        gpio.b[3].b_[14].write(|w| w.pbyte().clear_bit());
        for _ in 0..10_000 {}
    }
}
