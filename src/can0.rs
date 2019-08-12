#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Data Bit Timing Prescaler Register"]
    pub dbtp: DBTP,
    #[doc = "0x10 - Test Register"]
    pub test: TEST,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - CC Control Register"]
    pub cccr: CCCR,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler Register"]
    pub nbtp: NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration"]
    pub tscc: TSCC,
    #[doc = "0x24 - Timestamp Counter Value"]
    pub tscv: TSCV,
    #[doc = "0x28 - Timeout Counter Configuration"]
    pub tocc: TOCC,
    #[doc = "0x2c - Timeout Counter Value"]
    pub tocv: TOCV,
    _reserved8: [u8; 16usize],
    #[doc = "0x40 - Error Counter Register"]
    pub ecr: ECR,
    #[doc = "0x44 - Protocol Status Register"]
    pub psr: PSR,
    #[doc = "0x48 - Transmitter Delay Compensator Register"]
    pub tdcr: TDCR,
    _reserved11: [u8; 4usize],
    #[doc = "0x50 - Interrupt Register"]
    pub ir: IR,
    #[doc = "0x54 - Interrupt Enable"]
    pub ie: IE,
    #[doc = "0x58 - Interrupt Line Select"]
    pub ils: ILS,
    #[doc = "0x5c - Interrupt Line Enable"]
    pub ile: ILE,
    _reserved15: [u8; 32usize],
    #[doc = "0x80 - Global Filter Configuration"]
    pub gfc: GFC,
    #[doc = "0x84 - Standard ID Filter Configuration"]
    pub sidfc: SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration"]
    pub xidfc: XIDFC,
    _reserved18: [u8; 4usize],
    #[doc = "0x90 - Extended ID AND Mask"]
    pub xidam: XIDAM,
    #[doc = "0x94 - High Priority Message Status"]
    pub hpms: HPMS,
    #[doc = "0x98 - New Data 1"]
    pub ndat1: NDAT1,
    #[doc = "0x9c - New Data 2"]
    pub ndat2: NDAT2,
    #[doc = "0xa0 - Rx FIFO 0 Configuration"]
    pub rxf0c: RXF0C,
    #[doc = "0xa4 - Rx FIFO 0 Status"]
    pub rxf0s: RXF0S,
    #[doc = "0xa8 - Rx FIFO 0 Acknowledge"]
    pub rxf0a: RXF0A,
    #[doc = "0xac - Rx Buffer Configuration"]
    pub rxbc: RXBC,
    #[doc = "0xb0 - Rx FIFO 1 Configuration"]
    pub rxf1c: RXF1C,
    #[doc = "0xb4 - Rx FIFO 1 Status"]
    pub rxf1s: RXF1S,
    #[doc = "0xb8 - Rx FIFO 1 Acknowledge"]
    pub rxf1a: RXF1A,
    #[doc = "0xbc - Rx Buffer and FIFO Element Size Configuration"]
    pub rxesc: RXESC,
    #[doc = "0xc0 - Tx Buffer Configuration"]
    pub txbc: TXBC,
    #[doc = "0xc4 - Tx FIFO/Queue Status"]
    pub txfqs: TXFQS,
    #[doc = "0xc8 - Tx Buffer Element Size Configuration"]
    pub txesc: TXESC,
    #[doc = "0xcc - Tx Buffer Request Pending"]
    pub txbrp: TXBRP,
    #[doc = "0xd0 - Tx Buffer Add Request"]
    pub txbar: TXBAR,
    #[doc = "0xd4 - Tx Buffer Cancellation Request"]
    pub txbcr: TXBCR,
    #[doc = "0xd8 - Tx Buffer Transmission Occurred"]
    pub txbto: TXBTO,
    #[doc = "0xdc - Tx Buffer Cancellation Finished"]
    pub txbcf: TXBCF,
    #[doc = "0xe0 - Tx Buffer Transmission Interrupt Enable"]
    pub txbtie: TXBTIE,
    #[doc = "0xe4 - Tx Buffer Cancellation Finished Interrupt Enable"]
    pub txbcie: TXBCIE,
    _reserved40: [u8; 8usize],
    #[doc = "0xf0 - Tx Event FIFO Configuration"]
    pub txefc: TXEFC,
    #[doc = "0xf4 - Tx Event FIFO Status"]
    pub txefs: TXEFS,
    #[doc = "0xf8 - Tx Event FIFO Acknowledge"]
    pub txefa: TXEFA,
    _reserved43: [u8; 260usize],
    #[doc = "0x200 - CAN Message RAM Base Address"]
    pub mrba: MRBA,
    _reserved44: [u8; 508usize],
    #[doc = "0x400 - External Timestamp Counter Configuration"]
    pub etscc: ETSCC,
    _reserved45: [u8; 508usize],
    #[doc = "0x600 - External Timestamp Counter Value"]
    pub etscv: ETSCV,
}
#[doc = "Data Bit Timing Prescaler Register"]
pub struct DBTP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Data Bit Timing Prescaler Register"]
pub mod dbtp;
#[doc = "Test Register"]
pub struct TEST {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Test Register"]
pub mod test;
#[doc = "CC Control Register"]
pub struct CCCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CC Control Register"]
pub mod cccr;
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub struct NBTP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub mod nbtp;
#[doc = "Timestamp Counter Configuration"]
pub struct TSCC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Counter Configuration"]
pub mod tscc;
#[doc = "Timestamp Counter Value"]
pub struct TSCV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timestamp Counter Value"]
pub mod tscv;
#[doc = "Timeout Counter Configuration"]
pub struct TOCC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timeout Counter Configuration"]
pub mod tocc;
#[doc = "Timeout Counter Value"]
pub struct TOCV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Timeout Counter Value"]
pub mod tocv;
#[doc = "Error Counter Register"]
pub struct ECR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "Protocol Status Register"]
pub struct PSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "Transmitter Delay Compensator Register"]
pub struct TDCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Delay Compensator Register"]
pub mod tdcr;
#[doc = "Interrupt Register"]
pub struct IR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod ir;
#[doc = "Interrupt Enable"]
pub struct IE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod ie;
#[doc = "Interrupt Line Select"]
pub struct ILS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Line Select"]
pub mod ils;
#[doc = "Interrupt Line Enable"]
pub struct ILE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Line Enable"]
pub mod ile;
#[doc = "Global Filter Configuration"]
pub struct GFC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Filter Configuration"]
pub mod gfc;
#[doc = "Standard ID Filter Configuration"]
pub struct SIDFC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Standard ID Filter Configuration"]
pub mod sidfc;
#[doc = "Extended ID Filter Configuration"]
pub struct XIDFC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Extended ID Filter Configuration"]
pub mod xidfc;
#[doc = "Extended ID AND Mask"]
pub struct XIDAM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Extended ID AND Mask"]
pub mod xidam;
#[doc = "High Priority Message Status"]
pub struct HPMS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "High Priority Message Status"]
pub mod hpms;
#[doc = "New Data 1"]
pub struct NDAT1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "New Data 1"]
pub mod ndat1;
#[doc = "New Data 2"]
pub struct NDAT2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "New Data 2"]
pub mod ndat2;
#[doc = "Rx FIFO 0 Configuration"]
pub struct RXF0C {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO 0 Configuration"]
pub mod rxf0c;
#[doc = "Rx FIFO 0 Status"]
pub struct RXF0S {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO 0 Status"]
pub mod rxf0s;
#[doc = "Rx FIFO 0 Acknowledge"]
pub struct RXF0A {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO 0 Acknowledge"]
pub mod rxf0a;
#[doc = "Rx Buffer Configuration"]
pub struct RXBC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx Buffer Configuration"]
pub mod rxbc;
#[doc = "Rx FIFO 1 Configuration"]
pub struct RXF1C {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO 1 Configuration"]
pub mod rxf1c;
#[doc = "Rx FIFO 1 Status"]
pub struct RXF1S {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO 1 Status"]
pub mod rxf1s;
#[doc = "Rx FIFO 1 Acknowledge"]
pub struct RXF1A {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx FIFO 1 Acknowledge"]
pub mod rxf1a;
#[doc = "Rx Buffer and FIFO Element Size Configuration"]
pub struct RXESC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Rx Buffer and FIFO Element Size Configuration"]
pub mod rxesc;
#[doc = "Tx Buffer Configuration"]
pub struct TXBC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Configuration"]
pub mod txbc;
#[doc = "Tx FIFO/Queue Status"]
pub struct TXFQS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx FIFO/Queue Status"]
pub mod txfqs;
#[doc = "Tx Buffer Element Size Configuration"]
pub struct TXESC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Element Size Configuration"]
pub mod txesc;
#[doc = "Tx Buffer Request Pending"]
pub struct TXBRP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Request Pending"]
pub mod txbrp;
#[doc = "Tx Buffer Add Request"]
pub struct TXBAR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Add Request"]
pub mod txbar;
#[doc = "Tx Buffer Cancellation Request"]
pub struct TXBCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Cancellation Request"]
pub mod txbcr;
#[doc = "Tx Buffer Transmission Occurred"]
pub struct TXBTO {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Transmission Occurred"]
pub mod txbto;
#[doc = "Tx Buffer Cancellation Finished"]
pub struct TXBCF {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Cancellation Finished"]
pub mod txbcf;
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub struct TXBTIE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Transmission Interrupt Enable"]
pub mod txbtie;
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub struct TXBCIE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable"]
pub mod txbcie;
#[doc = "Tx Event FIFO Configuration"]
pub struct TXEFC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Event FIFO Configuration"]
pub mod txefc;
#[doc = "Tx Event FIFO Status"]
pub struct TXEFS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Event FIFO Status"]
pub mod txefs;
#[doc = "Tx Event FIFO Acknowledge"]
pub struct TXEFA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Tx Event FIFO Acknowledge"]
pub mod txefa;
#[doc = "CAN Message RAM Base Address"]
pub struct MRBA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CAN Message RAM Base Address"]
pub mod mrba;
#[doc = "External Timestamp Counter Configuration"]
pub struct ETSCC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "External Timestamp Counter Configuration"]
pub mod etscc;
#[doc = "External Timestamp Counter Value"]
pub struct ETSCV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "External Timestamp Counter Value"]
pub mod etscv;
