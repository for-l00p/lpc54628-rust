#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Async peripheral reset control"]
    pub asyncpresetctrl: ASYNCPRESETCTRL,
    #[doc = "0x04 - Set bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlset: ASYNCPRESETCTRLSET,
    #[doc = "0x08 - Clear bits in ASYNCPRESETCTRL"]
    pub asyncpresetctrlclr: ASYNCPRESETCTRLCLR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Async peripheral clock control"]
    pub asyncapbclkctrl: ASYNCAPBCLKCTRL,
    #[doc = "0x14 - Set bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlset: ASYNCAPBCLKCTRLSET,
    #[doc = "0x18 - Clear bits in ASYNCAPBCLKCTRL"]
    pub asyncapbclkctrlclr: ASYNCAPBCLKCTRLCLR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Async APB clock source select A"]
    pub asyncapbclksela: ASYNCAPBCLKSELA,
}
#[doc = "Async peripheral reset control"]
pub struct ASYNCPRESETCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Async peripheral reset control"]
pub mod asyncpresetctrl;
#[doc = "Set bits in ASYNCPRESETCTRL"]
pub struct ASYNCPRESETCTRLSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlset;
#[doc = "Clear bits in ASYNCPRESETCTRL"]
pub struct ASYNCPRESETCTRLCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in ASYNCPRESETCTRL"]
pub mod asyncpresetctrlclr;
#[doc = "Async peripheral clock control"]
pub struct ASYNCAPBCLKCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Async peripheral clock control"]
pub mod asyncapbclkctrl;
#[doc = "Set bits in ASYNCAPBCLKCTRL"]
pub struct ASYNCAPBCLKCTRLSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlset;
#[doc = "Clear bits in ASYNCAPBCLKCTRL"]
pub struct ASYNCAPBCLKCTRLCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in ASYNCAPBCLKCTRL"]
pub mod asyncapbclkctrlclr;
#[doc = "Async APB clock source select A"]
pub struct ASYNCAPBCLKSELA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Async APB clock source select A"]
pub mod asyncapbclksela;
