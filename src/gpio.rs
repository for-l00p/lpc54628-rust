#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub b: [B; 6],
    _reserved1: [u8; 3904usize],
    #[doc = "0x1000 - no description available"]
    pub w: [W; 6],
    _reserved2: [u8; 3328usize],
    #[doc = "0x2000 - Direction registers"]
    pub dir: [DIR; 6],
    _reserved3: [u8; 104usize],
    #[doc = "0x2080 - Mask register"]
    pub mask: [MASK; 6],
    _reserved4: [u8; 104usize],
    #[doc = "0x2100 - Port pin register"]
    pub pin: [PIN; 6],
    _reserved5: [u8; 104usize],
    #[doc = "0x2180 - Masked port register"]
    pub mpin: [MPIN; 6],
    _reserved6: [u8; 104usize],
    #[doc = "0x2200 - Write: Set register for port Read: output bits for port"]
    pub set: [SET; 6],
    _reserved7: [u8; 104usize],
    #[doc = "0x2280 - Clear port"]
    pub clr: [CLR; 6],
    _reserved8: [u8; 104usize],
    #[doc = "0x2300 - Toggle port"]
    pub not: [NOT; 6],
    _reserved9: [u8; 104usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset: [DIRSET; 6],
    _reserved10: [u8; 104usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr: [DIRCLR; 6],
    _reserved11: [u8; 104usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot: [DIRNOT; 6],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b_: [self::b::B_; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod b;
#[doc = r"Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w_: [self::w::W_; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod w;
#[doc = "Direction registers"]
pub struct DIR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Direction registers"]
pub mod dir;
#[doc = "Mask register"]
pub struct MASK {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mask register"]
pub mod mask;
#[doc = "Port pin register"]
pub struct PIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Port pin register"]
pub mod pin;
#[doc = "Masked port register"]
pub struct MPIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Masked port register"]
pub mod mpin;
#[doc = "Write: Set register for port Read: output bits for port"]
pub struct SET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set;
#[doc = "Clear port"]
pub struct CLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear port"]
pub mod clr;
#[doc = "Toggle port"]
pub struct NOT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Toggle port"]
pub mod not;
#[doc = "Set pin direction bits for port"]
pub struct DIRSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "Clear pin direction bits for port"]
pub struct DIRCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "Toggle pin direction bits for port"]
pub struct DIRNOT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
