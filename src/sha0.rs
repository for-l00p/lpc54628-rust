#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Enable register"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Clear register"]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Memory Control register"]
    pub memctrl: MEMCTRL,
    #[doc = "0x14 - Memory Address register"]
    pub memaddr: MEMADDR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Input Data register"]
    pub indata: INDATA,
    #[doc = "0x24 - Alias register"]
    pub alias: [ALIAS; 7],
    #[doc = "0x40 - Digest register"]
    pub digest: [DIGEST; 8],
}
#[doc = "Control register"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Status register"]
pub struct STATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod status;
#[doc = "Interrupt Enable register"]
pub struct INTENSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable register"]
pub mod intenset;
#[doc = "Interrupt Clear register"]
pub struct INTENCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear register"]
pub mod intenclr;
#[doc = "Memory Control register"]
pub struct MEMCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Memory Control register"]
pub mod memctrl;
#[doc = "Memory Address register"]
pub struct MEMADDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Memory Address register"]
pub mod memaddr;
#[doc = "Input Data register"]
pub struct INDATA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Input Data register"]
pub mod indata;
#[doc = "Alias register"]
pub struct ALIAS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Alias register"]
pub mod alias;
#[doc = "Digest register"]
pub struct DIGEST {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digest register"]
pub mod digest;
