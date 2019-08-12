#[doc = "Configuration for EMC_CSx"]
pub struct STATICCONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configuration for EMC_CSx"]
pub mod staticconfig;
#[doc = "Delay from EMC_CSx to write enable"]
pub struct STATICWAITWEN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Delay from EMC_CSx to write enable"]
pub mod staticwaitwen;
#[doc = "Delay from EMC_CSx or address change, whichever is later, to output enable"]
pub struct STATICWAITOEN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Delay from EMC_CSx or address change, whichever is later, to output enable"]
pub mod staticwaitoen;
#[doc = "Delay from EMC_CSx to a read access"]
pub struct STATICWAITRD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Delay from EMC_CSx to a read access"]
pub mod staticwaitrd;
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CSx"]
pub struct STATICWAITPAGE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Delay for asynchronous page mode sequential accesses for EMC_CSx"]
pub mod staticwaitpage;
#[doc = "Delay from EMC_CSx to a write access"]
pub struct STATICWAITWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Delay from EMC_CSx to a write access"]
pub mod staticwaitwr;
#[doc = "Number of bus turnaround cycles EMC_CSx"]
pub struct STATICWAITTURN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Number of bus turnaround cycles EMC_CSx"]
pub mod staticwaitturn;
