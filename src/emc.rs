#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls operation of the memory controller"]
    pub control: CONTROL,
    #[doc = "0x04 - Provides EMC status information"]
    pub status: STATUS,
    #[doc = "0x08 - Configures operation of the memory controller"]
    pub config: CONFIG,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - Controls dynamic memory operation"]
    pub dynamiccontrol: DYNAMICCONTROL,
    #[doc = "0x24 - Configures dynamic memory refresh"]
    pub dynamicrefresh: DYNAMICREFRESH,
    #[doc = "0x28 - Configures dynamic memory read strategy"]
    pub dynamicreadconfig: DYNAMICREADCONFIG,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - Precharge command period"]
    pub dynamicrp: DYNAMICRP,
    #[doc = "0x34 - Active to precharge command period"]
    pub dynamicras: DYNAMICRAS,
    #[doc = "0x38 - Self-refresh exit time"]
    pub dynamicsrex: DYNAMICSREX,
    #[doc = "0x3c - Last-data-out to active command time"]
    pub dynamicapr: DYNAMICAPR,
    #[doc = "0x40 - Data-in to active command time"]
    pub dynamicdal: DYNAMICDAL,
    #[doc = "0x44 - Write recovery time"]
    pub dynamicwr: DYNAMICWR,
    #[doc = "0x48 - Selects the active to active command period"]
    pub dynamicrc: DYNAMICRC,
    #[doc = "0x4c - Selects the auto-refresh period"]
    pub dynamicrfc: DYNAMICRFC,
    #[doc = "0x50 - Time for exit self-refresh to active command"]
    pub dynamicxsr: DYNAMICXSR,
    #[doc = "0x54 - Latency for active bank A to active bank B"]
    pub dynamicrrd: DYNAMICRRD,
    #[doc = "0x58 - Time for load mode register to active command"]
    pub dynamicmrd: DYNAMICMRD,
    _reserved17: [u8; 36usize],
    #[doc = "0x80 - Time for long static memory read and write transfers"]
    pub staticextendedwait: STATICEXTENDEDWAIT,
    _reserved18: [u8; 124usize],
    #[doc = "0x100 - no description available"]
    pub dynamic0: DYNAMIC,
    _reserved19: [u8; 24usize],
    #[doc = "0x120 - no description available"]
    pub dynamic1: DYNAMIC,
    _reserved20: [u8; 24usize],
    #[doc = "0x140 - no description available"]
    pub dynamic2: DYNAMIC,
    _reserved21: [u8; 24usize],
    #[doc = "0x160 - no description available"]
    pub dynamic3: DYNAMIC,
    _reserved22: [u8; 152usize],
    #[doc = "0x200 - no description available"]
    pub static0: STATIC,
    _reserved23: [u8; 4usize],
    #[doc = "0x220 - no description available"]
    pub static1: STATIC,
    _reserved24: [u8; 4usize],
    #[doc = "0x240 - no description available"]
    pub static2: STATIC,
    _reserved25: [u8; 4usize],
    #[doc = "0x260 - no description available"]
    pub static3: STATIC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DYNAMIC {
    #[doc = "0x00 - Configuration information for EMC_DYCSx"]
    pub dynamicconfig: self::dynamic::DYNAMICCONFIG,
    #[doc = "0x04 - RAS and CAS latencies for EMC_DYCSx"]
    pub dynamicrascas: self::dynamic::DYNAMICRASCAS,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod dynamic;
#[doc = r"Register block"]
#[repr(C)]
pub struct STATIC {
    #[doc = "0x00 - Configuration for EMC_CSx"]
    pub staticconfig: self::static_::STATICCONFIG,
    #[doc = "0x04 - Delay from EMC_CSx to write enable"]
    pub staticwaitwen: self::static_::STATICWAITWEN,
    #[doc = "0x08 - Delay from EMC_CSx or address change, whichever is later, to output enable"]
    pub staticwaitoen: self::static_::STATICWAITOEN,
    #[doc = "0x0c - Delay from EMC_CSx to a read access"]
    pub staticwaitrd: self::static_::STATICWAITRD,
    #[doc = "0x10 - Delay for asynchronous page mode sequential accesses for EMC_CSx"]
    pub staticwaitpage: self::static_::STATICWAITPAGE,
    #[doc = "0x14 - Delay from EMC_CSx to a write access"]
    pub staticwaitwr: self::static_::STATICWAITWR,
    #[doc = "0x18 - Number of bus turnaround cycles EMC_CSx"]
    pub staticwaitturn: self::static_::STATICWAITTURN,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod static_;
#[doc = "Controls operation of the memory controller"]
pub struct CONTROL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Controls operation of the memory controller"]
pub mod control;
#[doc = "Provides EMC status information"]
pub struct STATUS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Provides EMC status information"]
pub mod status;
#[doc = "Configures operation of the memory controller"]
pub struct CONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configures operation of the memory controller"]
pub mod config;
#[doc = "Controls dynamic memory operation"]
pub struct DYNAMICCONTROL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Controls dynamic memory operation"]
pub mod dynamiccontrol;
#[doc = "Configures dynamic memory refresh"]
pub struct DYNAMICREFRESH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configures dynamic memory refresh"]
pub mod dynamicrefresh;
#[doc = "Configures dynamic memory read strategy"]
pub struct DYNAMICREADCONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configures dynamic memory read strategy"]
pub mod dynamicreadconfig;
#[doc = "Precharge command period"]
pub struct DYNAMICRP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Precharge command period"]
pub mod dynamicrp;
#[doc = "Active to precharge command period"]
pub struct DYNAMICRAS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Active to precharge command period"]
pub mod dynamicras;
#[doc = "Self-refresh exit time"]
pub struct DYNAMICSREX {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Self-refresh exit time"]
pub mod dynamicsrex;
#[doc = "Last-data-out to active command time"]
pub struct DYNAMICAPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Last-data-out to active command time"]
pub mod dynamicapr;
#[doc = "Data-in to active command time"]
pub struct DYNAMICDAL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Data-in to active command time"]
pub mod dynamicdal;
#[doc = "Write recovery time"]
pub struct DYNAMICWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write recovery time"]
pub mod dynamicwr;
#[doc = "Selects the active to active command period"]
pub struct DYNAMICRC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Selects the active to active command period"]
pub mod dynamicrc;
#[doc = "Selects the auto-refresh period"]
pub struct DYNAMICRFC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Selects the auto-refresh period"]
pub mod dynamicrfc;
#[doc = "Time for exit self-refresh to active command"]
pub struct DYNAMICXSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time for exit self-refresh to active command"]
pub mod dynamicxsr;
#[doc = "Latency for active bank A to active bank B"]
pub struct DYNAMICRRD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Latency for active bank A to active bank B"]
pub mod dynamicrrd;
#[doc = "Time for load mode register to active command"]
pub struct DYNAMICMRD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time for load mode register to active command"]
pub mod dynamicmrd;
#[doc = "Time for long static memory read and write transfers"]
pub struct STATICEXTENDEDWAIT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time for long static memory read and write transfers"]
pub mod staticextendedwait;
