#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 4usize],
    _reserved_1_dlm: [u8; 4usize],
    _reserved_2_fcr: [u8; 4usize],
    #[doc = "0x0c - Line Control Register"]
    pub lcr: LCR,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Line Status Register"]
    pub lsr: LSR,
    _reserved5: [u8; 4usize],
    #[doc = "0x1c - Scratch Pad Register"]
    pub scr: SCR,
    _reserved6: [u8; 12usize],
    #[doc = "0x2c - Oversampling register"]
    pub osr: OSR,
    _reserved7: [u8; 24usize],
    #[doc = "0x48 - Smart Card Interface control register"]
    pub scictrl: SCICTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub fn thr(&self) -> &THR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const THR) }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub fn thr_mut(&self) -> &mut THR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut THR) }
    }
    #[doc = "0x00 - Receiver Buffer Register"]
    #[inline(always)]
    pub fn rbr(&self) -> &RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR) }
    }
    #[doc = "0x00 - Receiver Buffer Register"]
    #[inline(always)]
    pub fn rbr_mut(&self) -> &mut RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut RBR) }
    }
    #[doc = "0x00 - Divisor Latch LSB"]
    #[inline(always)]
    pub fn dll(&self) -> &DLL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DLL) }
    }
    #[doc = "0x00 - Divisor Latch LSB"]
    #[inline(always)]
    pub fn dll_mut(&self) -> &mut DLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DLL) }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const IER) }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_mut(&self) -> &mut IER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut IER) }
    }
    #[doc = "0x04 - Divisor Latch MSB"]
    #[inline(always)]
    pub fn dlm(&self) -> &DLM {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DLM) }
    }
    #[doc = "0x04 - Divisor Latch MSB"]
    #[inline(always)]
    pub fn dlm_mut(&self) -> &mut DLM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut DLM) }
    }
    #[doc = "0x08 - Interrupt ID Register"]
    #[inline(always)]
    pub fn iir(&self) -> &IIR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IIR) }
    }
    #[doc = "0x08 - Interrupt ID Register"]
    #[inline(always)]
    pub fn iir_mut(&self) -> &mut IIR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IIR) }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub fn fcr(&self) -> &FCR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const FCR) }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub fn fcr_mut(&self) -> &mut FCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut FCR) }
    }
}
#[doc = "Divisor Latch LSB"]
pub struct DLL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch LSB"]
pub mod dll;
#[doc = "Receiver Buffer Register"]
pub struct RBR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receiver Buffer Register"]
pub mod rbr;
#[doc = "Transmit Holding Register"]
pub struct THR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Divisor Latch MSB"]
pub struct DLM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Divisor Latch MSB"]
pub mod dlm;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "FIFO Control Register"]
pub struct FCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "Interrupt ID Register"]
pub struct IIR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt ID Register"]
pub mod iir;
#[doc = "Line Control Register"]
pub struct LCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "Line Status Register"]
pub struct LSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "Scratch Pad Register"]
pub struct SCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Scratch Pad Register"]
pub mod scr;
#[doc = "Oversampling register"]
pub struct OSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Oversampling register"]
pub mod osr;
#[doc = "Smart Card Interface control register"]
pub struct SCICTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Smart Card Interface control register"]
pub mod scictrl;
