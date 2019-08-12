#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_div: [u8; 3180usize],
    _reserved1: [u8; 404usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved4: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    #[doc = "0xe24 - FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fifowr48h: FIFOWR48H,
    _reserved9: [u8; 8usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    #[doc = "0xe34 - FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48h: FIFORD48H,
    _reserved11: [u8; 8usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    #[doc = "0xe44 - FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
    pub fiford48hnopop: FIFORD48HNOPOP,
    _reserved13: [u8; 4020usize],
    #[doc = "0x1dfc - I2S Module identification"]
    pub id: ID,
}
impl RegisterBlock {
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub fn secchannel0(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SECCHANNEL) }
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub fn secchannel0_mut(&self) -> &mut SECCHANNEL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut SECCHANNEL) }
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub fn secchannel1(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const SECCHANNEL) }
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub fn secchannel1_mut(&self) -> &mut SECCHANNEL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut SECCHANNEL) }
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub fn secchannel2(&self) -> &SECCHANNEL {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const SECCHANNEL) }
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub fn secchannel2_mut(&self) -> &mut SECCHANNEL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(64usize) as *mut SECCHANNEL) }
    }
    #[doc = "0xc00 - Configuration register 1 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg1(&self) -> &CFG1 {
        unsafe { &*(((self as *const Self) as *const u8).add(3072usize) as *const CFG1) }
    }
    #[doc = "0xc00 - Configuration register 1 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg1_mut(&self) -> &mut CFG1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3072usize) as *mut CFG1) }
    }
    #[doc = "0xc04 - Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg2(&self) -> &CFG2 {
        unsafe { &*(((self as *const Self) as *const u8).add(3076usize) as *const CFG2) }
    }
    #[doc = "0xc04 - Configuration register 2 for the primary channel pair."]
    #[inline(always)]
    pub fn cfg2_mut(&self) -> &mut CFG2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3076usize) as *mut CFG2) }
    }
    #[doc = "0xc08 - Status register for the primary channel pair."]
    #[inline(always)]
    pub fn stat(&self) -> &STAT {
        unsafe { &*(((self as *const Self) as *const u8).add(3080usize) as *const STAT) }
    }
    #[doc = "0xc08 - Status register for the primary channel pair."]
    #[inline(always)]
    pub fn stat_mut(&self) -> &mut STAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3080usize) as *mut STAT) }
    }
    #[doc = "0xc1c - Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub fn div(&self) -> &DIV {
        unsafe { &*(((self as *const Self) as *const u8).add(3100usize) as *const DIV) }
    }
    #[doc = "0xc1c - Clock divider, used by all channel pairs."]
    #[inline(always)]
    pub fn div_mut(&self) -> &mut DIV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3100usize) as *mut DIV) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SECCHANNEL {
    _reserved0: [u8; 3104usize],
    #[doc = "0xc20 - Configuration register 1 for channel pair"]
    pub pcfg1: self::secchannel::PCFG1,
    #[doc = "0xc24 - Configuration register 2 for channel pair"]
    pub pcfg2: self::secchannel::PCFG2,
    #[doc = "0xc28 - Status register for channel pair"]
    pub pstat: self::secchannel::PSTAT,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod secchannel;
#[doc = "Configuration register 1 for the primary channel pair."]
pub struct CFG1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configuration register 1 for the primary channel pair."]
pub mod cfg1;
#[doc = "Configuration register 2 for the primary channel pair."]
pub struct CFG2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configuration register 2 for the primary channel pair."]
pub mod cfg2;
#[doc = "Status register for the primary channel pair."]
pub struct STAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status register for the primary channel pair."]
pub mod stat;
#[doc = "Clock divider, used by all channel pairs."]
pub struct DIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clock divider, used by all channel pairs."]
pub mod div;
#[doc = "FIFO configuration and enable register."]
pub struct FIFOCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFO status register."]
pub struct FIFOSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub struct FIFOTRIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub struct FIFOINTENSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub struct FIFOINTENCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFO interrupt status register."]
pub struct FIFOINTSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFO write data."]
pub struct FIFOWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub struct FIFOWR48H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fifowr48h;
#[doc = "FIFO read data."]
pub struct FIFORD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub struct FIFORD48H {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48h;
#[doc = "FIFO data read with no FIFO pop."]
pub struct FIFORDNOPOP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub struct FIFORD48HNOPOP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO data read for upper data bits with no FIFO pop. May only be used if the I2S is configured for 2x 24-bit data and not using DMA."]
pub mod fiford48hnopop;
#[doc = "I2S Module identification"]
pub struct ID {
    register: vcell::VolatileCell<u32>,
}
#[doc = "I2S Module identification"]
pub mod id;
