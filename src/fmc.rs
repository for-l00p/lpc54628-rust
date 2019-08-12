#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub fctr: FCTR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Wait state register"]
    pub fbwst: FBWST,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved4: [u8; 4usize],
    #[doc = "0x2c - Words of 128-bit signature word"]
    pub fmsw: [FMSW; 4],
    _reserved5: [u8; 4004usize],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: FMSTAT,
    _reserved6: [u8; 4usize],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "Control register"]
pub struct FCTR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod fctr;
#[doc = "Wait state register"]
pub struct FBWST {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Wait state register"]
pub mod fbwst;
#[doc = "Signature start address register"]
pub struct FMSSTART {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "Signature stop-address register"]
pub struct FMSSTOP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "Words of 128-bit signature word"]
pub struct FMSW {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Words of 128-bit signature word"]
pub mod fmsw;
#[doc = "Signature generation status register"]
pub struct FMSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "Signature generation status clear register"]
pub struct FMSTATCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
