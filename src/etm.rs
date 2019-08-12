#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Code Register"]
    pub ccr: CCR,
    #[doc = "0x08 - Trigger Event Register"]
    pub trigger: TRIGGER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: SR,
    #[doc = "0x14 - System Configuration Register"]
    pub scr: SCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - Trace Enable Event Register"]
    pub eevr: EEVR,
    #[doc = "0x24 - Trace Enable Control 1 Register"]
    pub tecr1: TECR1,
    #[doc = "0x28 - FIFOFULL Level Register"]
    pub fflr: FFLR,
    _reserved8: [u8; 276usize],
    #[doc = "0x140 - Free-running counter reload value"]
    pub cntrldvr1: CNTRLDVR1,
    _reserved9: [u8; 156usize],
    #[doc = "0x1e0 - Synchronization Frequency Register"]
    pub syncfr: SYNCFR,
    #[doc = "0x1e4 - ID Register"]
    pub idr: IDR,
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    pub ccer: CCER,
    _reserved12: [u8; 4usize],
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: TESSEICR,
    _reserved13: [u8; 4usize],
    #[doc = "0x1f8 - Timestamp Event Register"]
    pub tsevr: TSEVR,
    _reserved14: [u8; 4usize],
    #[doc = "0x200 - CoreSight Trace ID Register"]
    pub traceidr: TRACEIDR,
    _reserved15: [u8; 4usize],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: IDR2,
    _reserved16: [u8; 264usize],
    #[doc = "0x314 - Device Power-Down Status Register"]
    pub pdsr: PDSR,
    _reserved17: [u8; 3016usize],
    #[doc = "0xee0 - Integration Test Miscelaneous Inputs Register"]
    pub _itmiscin: _ITMISCIN,
    _reserved18: [u8; 4usize],
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    pub _ittrigout: _ITTRIGOUT,
    _reserved19: [u8; 4usize],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    pub _itatbctr2: _ITATBCTR2,
    _reserved20: [u8; 4usize],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    pub _itatbctr0: _ITATBCTR0,
    _reserved21: [u8; 4usize],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved22: [u8; 156usize],
    #[doc = "0xfa0 - Claim Tag Set Register"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - Claim Tag Clear Register"]
    pub claimclr: CLAIMCLR,
    _reserved24: [u8; 8usize],
    #[doc = "0xfb0 - Lock Access Register"]
    pub lar: LAR,
    #[doc = "0xfb4 - Lock Status Register"]
    pub lsr: LSR,
    #[doc = "0xfb8 - Authentication Status Register"]
    pub authstatus: AUTHSTATUS,
    _reserved27: [u8; 16usize],
    #[doc = "0xfcc - CoreSight Device Type Register"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Peripheral Identification Register 4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Peripheral Identification Register 5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Peripheral Identification Register 6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Peripheral Identification Register 7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Peripheral Identification Register 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral Identification Register 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral Identification Register 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral Identification Register 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component Identification Register 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component Identification Register 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component Identification Register 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component Identification Register 3"]
    pub cidr3: CIDR3,
}
#[doc = "Main Control Register"]
pub struct CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Main Control Register"]
pub mod cr;
#[doc = "Configuration Code Register"]
pub struct CCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configuration Code Register"]
pub mod ccr;
#[doc = "Trigger Event Register"]
pub struct TRIGGER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trigger Event Register"]
pub mod trigger;
#[doc = "ETM Status Register"]
pub struct SR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "System Configuration Register"]
pub struct SCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System Configuration Register"]
pub mod scr;
#[doc = "Trace Enable Event Register"]
pub struct EEVR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trace Enable Event Register"]
pub mod eevr;
#[doc = "Trace Enable Control 1 Register"]
pub struct TECR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trace Enable Control 1 Register"]
pub mod tecr1;
#[doc = "FIFOFULL Level Register"]
pub struct FFLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFOFULL Level Register"]
pub mod fflr;
#[doc = "Free-running counter reload value"]
pub struct CNTRLDVR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Free-running counter reload value"]
pub mod cntrldvr1;
#[doc = "Synchronization Frequency Register"]
pub struct SYNCFR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "ID Register"]
pub struct IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ID Register"]
pub mod idr;
#[doc = "Configuration Code Extension Register"]
pub struct CCER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub struct TESSEICR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "Timestamp Event Register"]
pub struct TSEVR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Event Register"]
pub mod tsevr;
#[doc = "CoreSight Trace ID Register"]
pub struct TRACEIDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "ETM ID Register 2"]
pub struct IDR2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "Device Power-Down Status Register"]
pub struct PDSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "Integration Test Miscelaneous Inputs Register"]
pub struct _ITMISCIN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Integration Test Miscelaneous Inputs Register"]
pub mod _itmiscin;
#[doc = "Integration Test Trigger Out Register"]
pub struct _ITTRIGOUT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Integration Test Trigger Out Register"]
pub mod _ittrigout;
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub struct _ITATBCTR2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod _itatbctr2;
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub struct _ITATBCTR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod _itatbctr0;
#[doc = "Integration Mode Control Register"]
pub struct ITCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Integration Mode Control Register"]
pub mod itctrl;
#[doc = "Claim Tag Set Register"]
pub struct CLAIMSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Claim Tag Set Register"]
pub mod claimset;
#[doc = "Claim Tag Clear Register"]
pub struct CLAIMCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "Lock Access Register"]
pub struct LAR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Lock Access Register"]
pub mod lar;
#[doc = "Lock Status Register"]
pub struct LSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Lock Status Register"]
pub mod lsr;
#[doc = "Authentication Status Register"]
pub struct AUTHSTATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Authentication Status Register"]
pub mod authstatus;
#[doc = "CoreSight Device Type Register"]
pub struct DEVTYPE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CoreSight Device Type Register"]
pub mod devtype;
#[doc = "Peripheral Identification Register 4"]
pub struct PIDR4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 4"]
pub mod pidr4;
#[doc = "Peripheral Identification Register 5"]
pub struct PIDR5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 5"]
pub mod pidr5;
#[doc = "Peripheral Identification Register 6"]
pub struct PIDR6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 6"]
pub mod pidr6;
#[doc = "Peripheral Identification Register 7"]
pub struct PIDR7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 7"]
pub mod pidr7;
#[doc = "Peripheral Identification Register 0"]
pub struct PIDR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 0"]
pub mod pidr0;
#[doc = "Peripheral Identification Register 1"]
pub struct PIDR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 1"]
pub mod pidr1;
#[doc = "Peripheral Identification Register 2"]
pub struct PIDR2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 2"]
pub mod pidr2;
#[doc = "Peripheral Identification Register 3"]
pub struct PIDR3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Identification Register 3"]
pub mod pidr3;
#[doc = "Component Identification Register 0"]
pub struct CIDR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Component Identification Register 0"]
pub mod cidr0;
#[doc = "Component Identification Register 1"]
pub struct CIDR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Component Identification Register 1"]
pub mod cidr1;
#[doc = "Component Identification Register 2"]
pub struct CIDR2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Component Identification Register 2"]
pub mod cidr2;
#[doc = "Component Identification Register 3"]
pub struct CIDR3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Component Identification Register 3"]
pub mod cidr3;
