#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub channel0: CHANNEL,
    _reserved1: [u8; 108usize],
    #[doc = "0x100 - no description available"]
    pub channel1: CHANNEL,
    _reserved2: [u8; 3436usize],
    #[doc = "0xf00 - Channel Enable register"]
    pub chanen: CHANEN,
    _reserved3: [u8; 8usize],
    #[doc = "0xf0c - I/O Configuration register"]
    pub iocfg: IOCFG,
    #[doc = "0xf10 - Use 2FS register"]
    pub use2fs: USE2FS,
    _reserved5: [u8; 108usize],
    #[doc = "0xf80 - HWVAD input gain register"]
    pub hwvadgain: HWVADGAIN,
    #[doc = "0xf84 - HWVAD filter control register"]
    pub hwvadhpfs: HWVADHPFS,
    #[doc = "0xf88 - HWVAD control register"]
    pub hwvadst10: HWVADST10,
    #[doc = "0xf8c - HWVAD filter reset register"]
    pub hwvadrstt: HWVADRSTT,
    #[doc = "0xf90 - HWVAD noise estimator gain register"]
    pub hwvadthgn: HWVADTHGN,
    #[doc = "0xf94 - HWVAD signal estimator gain register"]
    pub hwvadthgs: HWVADTHGS,
    #[doc = "0xf98 - HWVAD noise envelope estimator register"]
    pub hwvadlowz: HWVADLOWZ,
    _reserved12: [u8; 96usize],
    #[doc = "0xffc - Module Identification register"]
    pub id: ID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Oversample Rate register 0"]
    pub osr: self::channel::OSR,
    #[doc = "0x04 - DMIC Clock Register 0"]
    pub divhfclk: self::channel::DIVHFCLK,
    #[doc = "0x08 - Pre-Emphasis Filter Coefficient for 2 FS register"]
    pub preac2fscoef: self::channel::PREAC2FSCOEF,
    #[doc = "0x0c - Pre-Emphasis Filter Coefficient for 4 FS register"]
    pub preac4fscoef: self::channel::PREAC4FSCOEF,
    #[doc = "0x10 - Decimator Gain Shift register"]
    pub gainshift: self::channel::GAINSHIFT,
    _reserved5: [u8; 108usize],
    #[doc = "0x80 - FIFO Control register 0"]
    pub fifo_ctrl: self::channel::FIFO_CTRL,
    #[doc = "0x84 - FIFO Status register 0"]
    pub fifo_status: self::channel::FIFO_STATUS,
    #[doc = "0x88 - FIFO Data Register 0"]
    pub fifo_data: self::channel::FIFO_DATA,
    #[doc = "0x8c - PDM Source Configuration register 0"]
    pub phy_ctrl: self::channel::PHY_CTRL,
    #[doc = "0x90 - DC Control register 0"]
    pub dc_ctrl: self::channel::DC_CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "Channel Enable register"]
pub struct CHANEN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable register"]
pub mod chanen;
#[doc = "I/O Configuration register"]
pub struct IOCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "I/O Configuration register"]
pub mod iocfg;
#[doc = "Use 2FS register"]
pub struct USE2FS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Use 2FS register"]
pub mod use2fs;
#[doc = "HWVAD input gain register"]
pub struct HWVADGAIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD input gain register"]
pub mod hwvadgain;
#[doc = "HWVAD filter control register"]
pub struct HWVADHPFS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD filter control register"]
pub mod hwvadhpfs;
#[doc = "HWVAD control register"]
pub struct HWVADST10 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD control register"]
pub mod hwvadst10;
#[doc = "HWVAD filter reset register"]
pub struct HWVADRSTT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD filter reset register"]
pub mod hwvadrstt;
#[doc = "HWVAD noise estimator gain register"]
pub struct HWVADTHGN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD noise estimator gain register"]
pub mod hwvadthgn;
#[doc = "HWVAD signal estimator gain register"]
pub struct HWVADTHGS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD signal estimator gain register"]
pub mod hwvadthgs;
#[doc = "HWVAD noise envelope estimator register"]
pub struct HWVADLOWZ {
    register: vcell::VolatileCell<u32>,
}
#[doc = "HWVAD noise envelope estimator register"]
pub mod hwvadlowz;
#[doc = "Module Identification register"]
pub struct ID {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Module Identification register"]
pub mod id;
