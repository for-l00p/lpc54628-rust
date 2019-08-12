#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Trigger select register for DMA channel"]
    pub sct0_inmux: [SCT0_INMUX; 7],
    _reserved1: [u8; 164usize],
    #[doc = "0xc0 - Pin interrupt select register"]
    pub pintsel: [PINTSEL; 8],
    #[doc = "0xe0 - Trigger select register for DMA channel"]
    pub dma_itrig_inmux: [DMA_ITRIG_INMUX; 30],
    _reserved3: [u8; 8usize],
    #[doc = "0x160 - DMA output trigger selection to become DMA trigger"]
    pub dma_otrig_inmux: [DMA_OTRIG_INMUX; 4],
    _reserved4: [u8; 16usize],
    #[doc = "0x180 - Selection for frequency measurement reference clock"]
    pub freqmeas_ref: FREQMEAS_REF,
    #[doc = "0x184 - Selection for frequency measurement target clock"]
    pub freqmeas_target: FREQMEAS_TARGET,
}
#[doc = "Trigger select register for DMA channel"]
pub struct SCT0_INMUX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trigger select register for DMA channel"]
pub mod sct0_inmux;
#[doc = "Pin interrupt select register"]
pub struct PINTSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Pin interrupt select register"]
pub mod pintsel;
#[doc = "Trigger select register for DMA channel"]
pub struct DMA_ITRIG_INMUX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trigger select register for DMA channel"]
pub mod dma_itrig_inmux;
#[doc = "DMA output trigger selection to become DMA trigger"]
pub struct DMA_OTRIG_INMUX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMA output trigger selection to become DMA trigger"]
pub mod dma_otrig_inmux;
#[doc = "Selection for frequency measurement reference clock"]
pub struct FREQMEAS_REF {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Selection for frequency measurement reference clock"]
pub mod freqmeas_ref;
#[doc = "Selection for frequency measurement target clock"]
pub struct FREQMEAS_TARGET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Selection for frequency measurement target clock"]
pub mod freqmeas_target;
