#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare value LSB register"]
    pub compval: COMPVAL,
    #[doc = "0x04 - Mask LSB register"]
    pub mask: MASK,
    #[doc = "0x08 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Counter LSB register"]
    pub counter: COUNTER,
    #[doc = "0x10 - Compare value MSB register"]
    pub compval_h: COMPVAL_H,
    #[doc = "0x14 - Mask MSB register"]
    pub mask_h: MASK_H,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Counter MSB register"]
    pub counter_h: COUNTER_H,
}
#[doc = "Compare value LSB register"]
pub struct COMPVAL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare value LSB register"]
pub mod compval;
#[doc = "Mask LSB register"]
pub struct MASK {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mask LSB register"]
pub mod mask;
#[doc = "Control register"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Counter LSB register"]
pub struct COUNTER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Counter LSB register"]
pub mod counter;
#[doc = "Compare value MSB register"]
pub struct COMPVAL_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Compare value MSB register"]
pub mod compval_h;
#[doc = "Mask MSB register"]
pub struct MASK_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mask MSB register"]
pub mod mask_h;
#[doc = "Counter MSB register"]
pub struct COUNTER_H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Counter MSB register"]
pub mod counter_h;
