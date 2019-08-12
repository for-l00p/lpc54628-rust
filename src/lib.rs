#![doc = "Peripheral access API for LPC54628 microcontrollers (generated using svd2rust v0.15.2)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.15.2/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT_BOD();
    fn DMA0();
    fn GINT0();
    fn GINT1();
    fn PIN_INT0();
    fn PIN_INT1();
    fn PIN_INT2();
    fn PIN_INT3();
    fn UTICK0();
    fn MRT0();
    fn CTIMER0();
    fn CTIMER1();
    fn SCT0();
    fn CTIMER3();
    fn FLEXCOMM0();
    fn FLEXCOMM1();
    fn FLEXCOMM2();
    fn FLEXCOMM3();
    fn FLEXCOMM4();
    fn FLEXCOMM5();
    fn FLEXCOMM6();
    fn FLEXCOMM7();
    fn ADC0_SEQA();
    fn ADC0_SEQB();
    fn ADC0_THCMP();
    fn DMIC0();
    fn HWVAD0();
    fn USB0_NEEDCLK();
    fn USB0();
    fn RTC();
    fn PIN_INT4();
    fn PIN_INT5();
    fn PIN_INT6();
    fn PIN_INT7();
    fn CTIMER2();
    fn CTIMER4();
    fn RIT();
    fn SPIFI0();
    fn FLEXCOMM8();
    fn FLEXCOMM9();
    fn SDIO();
    fn CAN0_IRQ0();
    fn CAN0_IRQ1();
    fn CAN1_IRQ0();
    fn CAN1_IRQ1();
    fn USB1();
    fn USB1_NEEDCLK();
    fn ETHERNET();
    fn ETHERNET_PMT();
    fn ETHERNET_MACLP();
    fn EEPROM();
    fn LCD();
    fn SMARTCARD0();
    fn SMARTCARD1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 57] = [
    Vector { _handler: WDT_BOD },
    Vector { _handler: DMA0 },
    Vector { _handler: GINT0 },
    Vector { _handler: GINT1 },
    Vector { _handler: PIN_INT0 },
    Vector { _handler: PIN_INT1 },
    Vector { _handler: PIN_INT2 },
    Vector { _handler: PIN_INT3 },
    Vector { _handler: UTICK0 },
    Vector { _handler: MRT0 },
    Vector { _handler: CTIMER0 },
    Vector { _handler: CTIMER1 },
    Vector { _handler: SCT0 },
    Vector { _handler: CTIMER3 },
    Vector {
        _handler: FLEXCOMM0,
    },
    Vector {
        _handler: FLEXCOMM1,
    },
    Vector {
        _handler: FLEXCOMM2,
    },
    Vector {
        _handler: FLEXCOMM3,
    },
    Vector {
        _handler: FLEXCOMM4,
    },
    Vector {
        _handler: FLEXCOMM5,
    },
    Vector {
        _handler: FLEXCOMM6,
    },
    Vector {
        _handler: FLEXCOMM7,
    },
    Vector {
        _handler: ADC0_SEQA,
    },
    Vector {
        _handler: ADC0_SEQB,
    },
    Vector {
        _handler: ADC0_THCMP,
    },
    Vector { _handler: DMIC0 },
    Vector { _handler: HWVAD0 },
    Vector {
        _handler: USB0_NEEDCLK,
    },
    Vector { _handler: USB0 },
    Vector { _handler: RTC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PIN_INT4 },
    Vector { _handler: PIN_INT5 },
    Vector { _handler: PIN_INT6 },
    Vector { _handler: PIN_INT7 },
    Vector { _handler: CTIMER2 },
    Vector { _handler: CTIMER4 },
    Vector { _handler: RIT },
    Vector { _handler: SPIFI0 },
    Vector {
        _handler: FLEXCOMM8,
    },
    Vector {
        _handler: FLEXCOMM9,
    },
    Vector { _handler: SDIO },
    Vector {
        _handler: CAN0_IRQ0,
    },
    Vector {
        _handler: CAN0_IRQ1,
    },
    Vector {
        _handler: CAN1_IRQ0,
    },
    Vector {
        _handler: CAN1_IRQ1,
    },
    Vector { _handler: USB1 },
    Vector {
        _handler: USB1_NEEDCLK,
    },
    Vector { _handler: ETHERNET },
    Vector {
        _handler: ETHERNET_PMT,
    },
    Vector {
        _handler: ETHERNET_MACLP,
    },
    Vector { _handler: EEPROM },
    Vector { _handler: LCD },
    Vector { _reserved: 0 },
    Vector {
        _handler: SMARTCARD0,
    },
    Vector {
        _handler: SMARTCARD1,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - WDT_BOD"]
    WDT_BOD,
    #[doc = "1 - DMA0"]
    DMA0,
    #[doc = "2 - GINT0"]
    GINT0,
    #[doc = "3 - GINT1"]
    GINT1,
    #[doc = "4 - PIN_INT0"]
    PIN_INT0,
    #[doc = "5 - PIN_INT1"]
    PIN_INT1,
    #[doc = "6 - PIN_INT2"]
    PIN_INT2,
    #[doc = "7 - PIN_INT3"]
    PIN_INT3,
    #[doc = "8 - UTICK0"]
    UTICK0,
    #[doc = "9 - MRT0"]
    MRT0,
    #[doc = "10 - CTIMER0"]
    CTIMER0,
    #[doc = "11 - CTIMER1"]
    CTIMER1,
    #[doc = "12 - SCT0"]
    SCT0,
    #[doc = "13 - CTIMER3"]
    CTIMER3,
    #[doc = "14 - FLEXCOMM0"]
    FLEXCOMM0,
    #[doc = "15 - FLEXCOMM1"]
    FLEXCOMM1,
    #[doc = "16 - FLEXCOMM2"]
    FLEXCOMM2,
    #[doc = "17 - FLEXCOMM3"]
    FLEXCOMM3,
    #[doc = "18 - FLEXCOMM4"]
    FLEXCOMM4,
    #[doc = "19 - FLEXCOMM5"]
    FLEXCOMM5,
    #[doc = "20 - FLEXCOMM6"]
    FLEXCOMM6,
    #[doc = "21 - FLEXCOMM7"]
    FLEXCOMM7,
    #[doc = "22 - ADC0_SEQA"]
    ADC0_SEQA,
    #[doc = "23 - ADC0_SEQB"]
    ADC0_SEQB,
    #[doc = "24 - ADC0_THCMP"]
    ADC0_THCMP,
    #[doc = "25 - DMIC0"]
    DMIC0,
    #[doc = "26 - HWVAD0"]
    HWVAD0,
    #[doc = "27 - USB0_NEEDCLK"]
    USB0_NEEDCLK,
    #[doc = "28 - USB0"]
    USB0,
    #[doc = "29 - RTC"]
    RTC,
    #[doc = "32 - PIN_INT4"]
    PIN_INT4,
    #[doc = "33 - PIN_INT5"]
    PIN_INT5,
    #[doc = "34 - PIN_INT6"]
    PIN_INT6,
    #[doc = "35 - PIN_INT7"]
    PIN_INT7,
    #[doc = "36 - CTIMER2"]
    CTIMER2,
    #[doc = "37 - CTIMER4"]
    CTIMER4,
    #[doc = "38 - RIT"]
    RIT,
    #[doc = "39 - SPIFI0"]
    SPIFI0,
    #[doc = "40 - FLEXCOMM8"]
    FLEXCOMM8,
    #[doc = "41 - FLEXCOMM9"]
    FLEXCOMM9,
    #[doc = "42 - SDIO"]
    SDIO,
    #[doc = "43 - CAN0_IRQ0"]
    CAN0_IRQ0,
    #[doc = "44 - CAN0_IRQ1"]
    CAN0_IRQ1,
    #[doc = "45 - CAN1_IRQ0"]
    CAN1_IRQ0,
    #[doc = "46 - CAN1_IRQ1"]
    CAN1_IRQ1,
    #[doc = "47 - USB1"]
    USB1,
    #[doc = "48 - USB1_NEEDCLK"]
    USB1_NEEDCLK,
    #[doc = "49 - ETHERNET"]
    ETHERNET,
    #[doc = "50 - ETHERNET_PMT"]
    ETHERNET_PMT,
    #[doc = "51 - ETHERNET_MACLP"]
    ETHERNET_MACLP,
    #[doc = "52 - EEPROM"]
    EEPROM,
    #[doc = "53 - LCD"]
    LCD,
    #[doc = "55 - SMARTCARD0"]
    SMARTCARD0,
    #[doc = "56 - SMARTCARD1"]
    SMARTCARD1,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WDT_BOD => 0,
            Interrupt::DMA0 => 1,
            Interrupt::GINT0 => 2,
            Interrupt::GINT1 => 3,
            Interrupt::PIN_INT0 => 4,
            Interrupt::PIN_INT1 => 5,
            Interrupt::PIN_INT2 => 6,
            Interrupt::PIN_INT3 => 7,
            Interrupt::UTICK0 => 8,
            Interrupt::MRT0 => 9,
            Interrupt::CTIMER0 => 10,
            Interrupt::CTIMER1 => 11,
            Interrupt::SCT0 => 12,
            Interrupt::CTIMER3 => 13,
            Interrupt::FLEXCOMM0 => 14,
            Interrupt::FLEXCOMM1 => 15,
            Interrupt::FLEXCOMM2 => 16,
            Interrupt::FLEXCOMM3 => 17,
            Interrupt::FLEXCOMM4 => 18,
            Interrupt::FLEXCOMM5 => 19,
            Interrupt::FLEXCOMM6 => 20,
            Interrupt::FLEXCOMM7 => 21,
            Interrupt::ADC0_SEQA => 22,
            Interrupt::ADC0_SEQB => 23,
            Interrupt::ADC0_THCMP => 24,
            Interrupt::DMIC0 => 25,
            Interrupt::HWVAD0 => 26,
            Interrupt::USB0_NEEDCLK => 27,
            Interrupt::USB0 => 28,
            Interrupt::RTC => 29,
            Interrupt::PIN_INT4 => 32,
            Interrupt::PIN_INT5 => 33,
            Interrupt::PIN_INT6 => 34,
            Interrupt::PIN_INT7 => 35,
            Interrupt::CTIMER2 => 36,
            Interrupt::CTIMER4 => 37,
            Interrupt::RIT => 38,
            Interrupt::SPIFI0 => 39,
            Interrupt::FLEXCOMM8 => 40,
            Interrupt::FLEXCOMM9 => 41,
            Interrupt::SDIO => 42,
            Interrupt::CAN0_IRQ0 => 43,
            Interrupt::CAN0_IRQ1 => 44,
            Interrupt::CAN1_IRQ0 => 45,
            Interrupt::CAN1_IRQ1 => 46,
            Interrupt::USB1 => 47,
            Interrupt::USB1_NEEDCLK => 48,
            Interrupt::ETHERNET => 49,
            Interrupt::ETHERNET_PMT => 50,
            Interrupt::ETHERNET_MACLP => 51,
            Interrupt::EEPROM => 52,
            Interrupt::LCD => 53,
            Interrupt::SMARTCARD0 => 55,
            Interrupt::SMARTCARD1 => 56,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r" Common register and bit access and modify traits"]
pub mod generic;
#[doc = "LPC5460x System configuration (SYSCON)"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "LPC5460x System configuration (SYSCON)"]
pub mod syscon;
#[doc = "LPC5411x I/O pin configuration (IOCON)"]
pub struct IOCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOCON {}
impl IOCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iocon::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for IOCON {
    type Target = iocon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOCON::ptr() }
    }
}
#[doc = "LPC5411x I/O pin configuration (IOCON)"]
pub mod iocon;
#[doc = "LPC5411x Group GPIO input interrupt (GINT0/1)"]
pub struct GINT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GINT0 {}
impl GINT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gint0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for GINT0 {
    type Target = gint0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GINT0::ptr() }
    }
}
#[doc = "LPC5411x Group GPIO input interrupt (GINT0/1)"]
pub mod gint0;
#[doc = "LPC5411x Group GPIO input interrupt (GINT0/1)"]
pub struct GINT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GINT1 {}
impl GINT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gint0::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for GINT1 {
    type Target = gint0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GINT1::ptr() }
    }
}
#[doc = "LPC5411x Pin interrupt and pattern match (PINT)"]
pub struct PINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINT {}
impl PINT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pint::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for PINT {
    type Target = pint::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PINT::ptr() }
    }
}
#[doc = "LPC5411x Pin interrupt and pattern match (PINT)"]
pub mod pint;
#[doc = "LPC5411x Input multiplexing (INPUT MUX)"]
pub struct INPUTMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INPUTMUX {}
impl INPUTMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const inputmux::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for INPUTMUX {
    type Target = inputmux::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*INPUTMUX::ptr() }
    }
}
#[doc = "LPC5411x Input multiplexing (INPUT MUX)"]
pub mod inputmux;
#[doc = "LPC5411x Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER0 {}
impl CTIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for CTIMER0 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER0::ptr() }
    }
}
#[doc = "LPC5411x Standard counter/timers (CTIMER0 to 4)"]
pub mod ctimer0;
#[doc = "LPC5411x Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER1 {}
impl CTIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for CTIMER1 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER1::ptr() }
    }
}
#[doc = "LPC5411x Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER2 {}
impl CTIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for CTIMER2 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER2::ptr() }
    }
}
#[doc = "LPC5411x Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER3 {}
impl CTIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for CTIMER3 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER3::ptr() }
    }
}
#[doc = "LPC5411x Standard counter/timers (CTIMER0 to 4)"]
pub struct CTIMER4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTIMER4 {}
impl CTIMER4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctimer0::RegisterBlock {
        0x4004_9000 as *const _
    }
}
impl Deref for CTIMER4 {
    type Target = ctimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTIMER4::ptr() }
    }
}
#[doc = "LPC5411x Windowed Watchdog Timer (WWDT)"]
pub struct WWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDT {}
impl WWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdt::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for WWDT {
    type Target = wwdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDT::ptr() }
    }
}
#[doc = "LPC5411x Windowed Watchdog Timer (WWDT)"]
pub mod wwdt;
#[doc = "LPC5411x Multi-Rate Timer (MRT)"]
pub struct MRT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MRT0 {}
impl MRT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mrt0::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for MRT0 {
    type Target = mrt0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MRT0::ptr() }
    }
}
#[doc = "LPC5411x Multi-Rate Timer (MRT)"]
pub mod mrt0;
#[doc = "LPC5411x Micro-tick Timer (UTICK)"]
pub struct UTICK0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UTICK0 {}
impl UTICK0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const utick0::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for UTICK0 {
    type Target = utick0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UTICK0::ptr() }
    }
}
#[doc = "LPC5411x Micro-tick Timer (UTICK)"]
pub mod utick0;
#[doc = "LPC54S60x/LPC5460x EEPROM controller"]
pub struct EEPROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EEPROM {}
impl EEPROM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eeprom::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for EEPROM {
    type Target = eeprom::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EEPROM::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x EEPROM controller"]
pub mod eeprom;
#[doc = "This is the description of component otpc It is an eFUSE OTP (One Time Programmable memory) controller with APB bus interface. More details will follow."]
pub struct OTPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTPC {}
impl OTPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otpc::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for OTPC {
    type Target = otpc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTPC::ptr() }
    }
}
#[doc = "This is the description of component otpc It is an eFUSE OTP (One Time Programmable memory) controller with APB bus interface. More details will follow."]
pub mod otpc;
#[doc = "LPC5411x Real-Time Clock (RTC)"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "LPC5411x Real-Time Clock (RTC)"]
pub mod rtc;
#[doc = "LPC54S60x/LPC5460x Repetitive Interrupt Timer(RIT)"]
pub struct RIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RIT {}
impl RIT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rit::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for RIT {
    type Target = rit::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RIT::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x Repetitive Interrupt Timer(RIT)"]
pub mod rit;
#[doc = "LPC54S60x/LPC5460x Flash signature generator"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x Flash signature generator"]
pub mod fmc;
#[doc = "LPC5460x/LPC54S60x Smart Card Interface"]
pub struct SMARTCARD0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMARTCARD0 {}
impl SMARTCARD0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smartcard0::RegisterBlock {
        0x4003_6000 as *const _
    }
}
impl Deref for SMARTCARD0 {
    type Target = smartcard0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMARTCARD0::ptr() }
    }
}
#[doc = "LPC5460x/LPC54S60x Smart Card Interface"]
pub mod smartcard0;
#[doc = "LPC5460x/LPC54S60x Smart Card Interface"]
pub struct SMARTCARD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMARTCARD1 {}
impl SMARTCARD1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smartcard0::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for SMARTCARD1 {
    type Target = smartcard0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMARTCARD1::ptr() }
    }
}
#[doc = "LPC5411x Asynchronous system configuration (ASYNC_SYSCON)"]
pub struct ASYNC_SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ASYNC_SYSCON {}
impl ASYNC_SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const async_syscon::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for ASYNC_SYSCON {
    type Target = async_syscon::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ASYNC_SYSCON::ptr() }
    }
}
#[doc = "LPC5411x Asynchronous system configuration (ASYNC_SYSCON)"]
pub mod async_syscon;
#[doc = "LPC5411x SPI Flash Interface (SPIFI)"]
pub struct SPIFI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIFI0 {}
impl SPIFI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spifi0::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for SPIFI0 {
    type Target = spifi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIFI0::ptr() }
    }
}
#[doc = "LPC5411x SPI Flash Interface (SPIFI)"]
pub mod spifi0;
#[doc = "LPC54S60x/LPC5460x External Memory Controller (EMC)"]
pub struct EMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMC {}
impl EMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emc::RegisterBlock {
        0x4008_1000 as *const _
    }
}
impl Deref for EMC {
    type Target = emc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMC::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x External Memory Controller (EMC)"]
pub mod emc;
#[doc = "LPC5411x DMA controller"]
pub struct DMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA0 {}
impl DMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma0::RegisterBlock {
        0x4008_2000 as *const _
    }
}
impl Deref for DMA0 {
    type Target = dma0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA0::ptr() }
    }
}
#[doc = "LPC5411x DMA controller"]
pub mod dma0;
#[doc = "LPC54S60x/LPC5460x LCD controller"]
pub struct LCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD {}
impl LCD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcd::RegisterBlock {
        0x4008_3000 as *const _
    }
}
impl Deref for LCD {
    type Target = lcd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCD::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x LCD controller"]
pub mod lcd;
#[doc = "LPC5411x USB 2.0 Device Controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "LPC5411x USB 2.0 Device Controller"]
pub mod usb0;
#[doc = "LPC5411x SCTimer/PWM (SCT)"]
pub struct SCT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCT0 {}
impl SCT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sct0::RegisterBlock {
        0x4008_5000 as *const _
    }
}
impl Deref for SCT0 {
    type Target = sct0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCT0::ptr() }
    }
}
#[doc = "LPC5411x SCTimer/PWM (SCT)"]
pub mod sct0;
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM0 {}
impl FLEXCOMM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for FLEXCOMM0 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM0::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub mod flexcomm0;
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM1 {}
impl FLEXCOMM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for FLEXCOMM1 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM1::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM2 {}
impl FLEXCOMM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for FLEXCOMM2 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM2::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM3 {}
impl FLEXCOMM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for FLEXCOMM3 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM3::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM4 {}
impl FLEXCOMM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for FLEXCOMM4 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM4::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM5 {}
impl FLEXCOMM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for FLEXCOMM5 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM5::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM6 {}
impl FLEXCOMM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for FLEXCOMM6 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM6::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM7 {}
impl FLEXCOMM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for FLEXCOMM7 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM7::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM8 {}
impl FLEXCOMM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_9000 as *const _
    }
}
impl Deref for FLEXCOMM8 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM8::ptr() }
    }
}
#[doc = "LPC5411x Flexcomm serial communication"]
pub struct FLEXCOMM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLEXCOMM9 {}
impl FLEXCOMM9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flexcomm0::RegisterBlock {
        0x4009_a000 as *const _
    }
}
impl Deref for FLEXCOMM9 {
    type Target = flexcomm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLEXCOMM9::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub mod i2c0;
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C5 {}
impl I2C5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for I2C5 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C5::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C6 {}
impl I2C6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for I2C6 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C6::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C7 {}
impl I2C7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for I2C7 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C7::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C8 {}
impl I2C8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_9000 as *const _
    }
}
impl Deref for I2C8 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C8::ptr() }
    }
}
#[doc = "LPC5411x I2C-bus interfaces"]
pub struct I2C9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C9 {}
impl I2C9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4009_a000 as *const _
    }
}
impl Deref for I2C9 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C9::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub mod spi0;
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI6::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI7 {}
impl SPI7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for SPI7 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI7::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI8 {}
impl SPI8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_9000 as *const _
    }
}
impl Deref for SPI8 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI8::ptr() }
    }
}
#[doc = "LPC5411x Serial Peripheral Interfaces (SPI)"]
pub struct SPI9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI9 {}
impl SPI9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4009_a000 as *const _
    }
}
impl Deref for SPI9 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI9::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_6000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub mod usart0;
#[doc = "LPC5411x USARTs"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_7000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4008_a000 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART5 {}
impl USART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_6000 as *const _
    }
}
impl Deref for USART5 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART5::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART7 {}
impl USART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for USART7 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART7::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART8 {}
impl USART8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_9000 as *const _
    }
}
impl Deref for USART8 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART8::ptr() }
    }
}
#[doc = "LPC5411x USARTs"]
pub struct USART9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART9 {}
impl USART9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4009_a000 as *const _
    }
}
impl Deref for USART9 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART9::ptr() }
    }
}
#[doc = "LPC5411x General Purpose I/O (GPIO)"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4008_c000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "LPC5411x General Purpose I/O (GPIO)"]
pub mod gpio;
#[doc = "LPC5411x DMIC Subsystem (DMIC))"]
pub struct DMIC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMIC0 {}
impl DMIC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmic0::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for DMIC0 {
    type Target = dmic0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMIC0::ptr() }
    }
}
#[doc = "LPC5411x DMIC Subsystem (DMIC))"]
pub mod dmic0;
#[doc = "LPC54S60x/LPC5460x Ethernet controller"]
pub struct ENET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENET {}
impl ENET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enet::RegisterBlock {
        0x4009_2000 as *const _
    }
}
impl Deref for ENET {
    type Target = enet::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENET::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x Ethernet controller"]
pub mod enet;
#[doc = "LPC54S60x/LPC5460x USB1 High-speed Device Controller"]
pub struct USBHSD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHSD {}
impl USBHSD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhsd::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for USBHSD {
    type Target = usbhsd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHSD::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x USB1 High-speed Device Controller"]
pub mod usbhsd;
#[doc = "LPC5411x CRC engine"]
pub struct CRC_ENGINE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC_ENGINE {}
impl CRC_ENGINE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc_engine::RegisterBlock {
        0x4009_5000 as *const _
    }
}
impl Deref for CRC_ENGINE {
    type Target = crc_engine::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC_ENGINE::ptr() }
    }
}
#[doc = "LPC5411x CRC engine"]
pub mod crc_engine;
#[doc = "LPC5411x I2S interface"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4009_7000 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "LPC5411x I2S interface"]
pub mod i2s0;
#[doc = "LPC5411x I2S interface"]
pub struct I2S1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S1 {}
impl I2S1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for I2S1 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S1::ptr() }
    }
}
#[doc = "SDMMC"]
pub struct SDIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIF {}
impl SDIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdif::RegisterBlock {
        0x4009_b000 as *const _
    }
}
impl Deref for SDIF {
    type Target = sdif::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDIF::ptr() }
    }
}
#[doc = "SDMMC"]
pub mod sdif;
#[doc = "LPC54S60x/LPC5460x Controller Area Network Flexible Data"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4009_d000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x Controller Area Network Flexible Data"]
pub mod can0;
#[doc = "LPC54S60x/LPC5460x Controller Area Network Flexible Data"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4009_e000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "LPC5411x 12-bit ADC controller (ADC)"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "LPC5411x 12-bit ADC controller (ADC)"]
pub mod adc0;
#[doc = "LPC54S60x/LPC5460x USB0 Full-speed Host controller"]
pub struct USBFSH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBFSH {}
impl USBFSH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbfsh::RegisterBlock {
        0x400a_2000 as *const _
    }
}
impl Deref for USBFSH {
    type Target = usbfsh::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBFSH::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x USB0 Full-speed Host controller"]
pub mod usbfsh;
#[doc = "LPC54S60x/LPC5460x USB1 High-speed Host Controller"]
pub struct USBHSH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHSH {}
impl USBHSH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhsh::RegisterBlock {
        0x400a_3000 as *const _
    }
}
impl Deref for USBHSH {
    type Target = usbhsh::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHSH::ptr() }
    }
}
#[doc = "LPC54S60x/LPC5460x USB1 High-speed Host Controller"]
pub mod usbhsh;
#[doc = "SHA"]
pub struct SHA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHA0 {}
impl SHA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sha0::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for SHA0 {
    type Target = sha0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SHA0::ptr() }
    }
}
#[doc = "SHA"]
pub mod sha0;
#[doc = "System Control Block"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Block"]
pub mod system_control;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "Embedded Trace Macrocell Registers"]
pub struct ETM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETM {}
impl ETM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const etm::RegisterBlock {
        0xe004_1000 as *const _
    }
}
impl Deref for ETM {
    type Target = etm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETM::ptr() }
    }
}
#[doc = "Embedded Trace Macrocell Registers"]
pub mod etm;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "IOCON"]
    pub IOCON: IOCON,
    #[doc = "GINT0"]
    pub GINT0: GINT0,
    #[doc = "GINT1"]
    pub GINT1: GINT1,
    #[doc = "PINT"]
    pub PINT: PINT,
    #[doc = "INPUTMUX"]
    pub INPUTMUX: INPUTMUX,
    #[doc = "CTIMER0"]
    pub CTIMER0: CTIMER0,
    #[doc = "CTIMER1"]
    pub CTIMER1: CTIMER1,
    #[doc = "CTIMER2"]
    pub CTIMER2: CTIMER2,
    #[doc = "CTIMER3"]
    pub CTIMER3: CTIMER3,
    #[doc = "CTIMER4"]
    pub CTIMER4: CTIMER4,
    #[doc = "WWDT"]
    pub WWDT: WWDT,
    #[doc = "MRT0"]
    pub MRT0: MRT0,
    #[doc = "UTICK0"]
    pub UTICK0: UTICK0,
    #[doc = "EEPROM"]
    pub EEPROM: EEPROM,
    #[doc = "OTPC"]
    pub OTPC: OTPC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RIT"]
    pub RIT: RIT,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "SMARTCARD0"]
    pub SMARTCARD0: SMARTCARD0,
    #[doc = "SMARTCARD1"]
    pub SMARTCARD1: SMARTCARD1,
    #[doc = "ASYNC_SYSCON"]
    pub ASYNC_SYSCON: ASYNC_SYSCON,
    #[doc = "SPIFI0"]
    pub SPIFI0: SPIFI0,
    #[doc = "EMC"]
    pub EMC: EMC,
    #[doc = "DMA0"]
    pub DMA0: DMA0,
    #[doc = "LCD"]
    pub LCD: LCD,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "SCT0"]
    pub SCT0: SCT0,
    #[doc = "FLEXCOMM0"]
    pub FLEXCOMM0: FLEXCOMM0,
    #[doc = "FLEXCOMM1"]
    pub FLEXCOMM1: FLEXCOMM1,
    #[doc = "FLEXCOMM2"]
    pub FLEXCOMM2: FLEXCOMM2,
    #[doc = "FLEXCOMM3"]
    pub FLEXCOMM3: FLEXCOMM3,
    #[doc = "FLEXCOMM4"]
    pub FLEXCOMM4: FLEXCOMM4,
    #[doc = "FLEXCOMM5"]
    pub FLEXCOMM5: FLEXCOMM5,
    #[doc = "FLEXCOMM6"]
    pub FLEXCOMM6: FLEXCOMM6,
    #[doc = "FLEXCOMM7"]
    pub FLEXCOMM7: FLEXCOMM7,
    #[doc = "FLEXCOMM8"]
    pub FLEXCOMM8: FLEXCOMM8,
    #[doc = "FLEXCOMM9"]
    pub FLEXCOMM9: FLEXCOMM9,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "I2C5"]
    pub I2C5: I2C5,
    #[doc = "I2C6"]
    pub I2C6: I2C6,
    #[doc = "I2C7"]
    pub I2C7: I2C7,
    #[doc = "I2C8"]
    pub I2C8: I2C8,
    #[doc = "I2C9"]
    pub I2C9: I2C9,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "SPI6"]
    pub SPI6: SPI6,
    #[doc = "SPI7"]
    pub SPI7: SPI7,
    #[doc = "SPI8"]
    pub SPI8: SPI8,
    #[doc = "SPI9"]
    pub SPI9: SPI9,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "USART5"]
    pub USART5: USART5,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "USART7"]
    pub USART7: USART7,
    #[doc = "USART8"]
    pub USART8: USART8,
    #[doc = "USART9"]
    pub USART9: USART9,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "DMIC0"]
    pub DMIC0: DMIC0,
    #[doc = "ENET"]
    pub ENET: ENET,
    #[doc = "USBHSD"]
    pub USBHSD: USBHSD,
    #[doc = "CRC_ENGINE"]
    pub CRC_ENGINE: CRC_ENGINE,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "I2S1"]
    pub I2S1: I2S1,
    #[doc = "SDIF"]
    pub SDIF: SDIF,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "USBFSH"]
    pub USBFSH: USBFSH,
    #[doc = "USBHSH"]
    pub USBHSH: USBHSH,
    #[doc = "SHA0"]
    pub SHA0: SHA0,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "ETM"]
    pub ETM: ETM,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            IOCON: IOCON {
                _marker: PhantomData,
            },
            GINT0: GINT0 {
                _marker: PhantomData,
            },
            GINT1: GINT1 {
                _marker: PhantomData,
            },
            PINT: PINT {
                _marker: PhantomData,
            },
            INPUTMUX: INPUTMUX {
                _marker: PhantomData,
            },
            CTIMER0: CTIMER0 {
                _marker: PhantomData,
            },
            CTIMER1: CTIMER1 {
                _marker: PhantomData,
            },
            CTIMER2: CTIMER2 {
                _marker: PhantomData,
            },
            CTIMER3: CTIMER3 {
                _marker: PhantomData,
            },
            CTIMER4: CTIMER4 {
                _marker: PhantomData,
            },
            WWDT: WWDT {
                _marker: PhantomData,
            },
            MRT0: MRT0 {
                _marker: PhantomData,
            },
            UTICK0: UTICK0 {
                _marker: PhantomData,
            },
            EEPROM: EEPROM {
                _marker: PhantomData,
            },
            OTPC: OTPC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RIT: RIT {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            SMARTCARD0: SMARTCARD0 {
                _marker: PhantomData,
            },
            SMARTCARD1: SMARTCARD1 {
                _marker: PhantomData,
            },
            ASYNC_SYSCON: ASYNC_SYSCON {
                _marker: PhantomData,
            },
            SPIFI0: SPIFI0 {
                _marker: PhantomData,
            },
            EMC: EMC {
                _marker: PhantomData,
            },
            DMA0: DMA0 {
                _marker: PhantomData,
            },
            LCD: LCD {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            SCT0: SCT0 {
                _marker: PhantomData,
            },
            FLEXCOMM0: FLEXCOMM0 {
                _marker: PhantomData,
            },
            FLEXCOMM1: FLEXCOMM1 {
                _marker: PhantomData,
            },
            FLEXCOMM2: FLEXCOMM2 {
                _marker: PhantomData,
            },
            FLEXCOMM3: FLEXCOMM3 {
                _marker: PhantomData,
            },
            FLEXCOMM4: FLEXCOMM4 {
                _marker: PhantomData,
            },
            FLEXCOMM5: FLEXCOMM5 {
                _marker: PhantomData,
            },
            FLEXCOMM6: FLEXCOMM6 {
                _marker: PhantomData,
            },
            FLEXCOMM7: FLEXCOMM7 {
                _marker: PhantomData,
            },
            FLEXCOMM8: FLEXCOMM8 {
                _marker: PhantomData,
            },
            FLEXCOMM9: FLEXCOMM9 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            I2C5: I2C5 {
                _marker: PhantomData,
            },
            I2C6: I2C6 {
                _marker: PhantomData,
            },
            I2C7: I2C7 {
                _marker: PhantomData,
            },
            I2C8: I2C8 {
                _marker: PhantomData,
            },
            I2C9: I2C9 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            SPI6: SPI6 {
                _marker: PhantomData,
            },
            SPI7: SPI7 {
                _marker: PhantomData,
            },
            SPI8: SPI8 {
                _marker: PhantomData,
            },
            SPI9: SPI9 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            USART4: USART4 {
                _marker: PhantomData,
            },
            USART5: USART5 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            USART7: USART7 {
                _marker: PhantomData,
            },
            USART8: USART8 {
                _marker: PhantomData,
            },
            USART9: USART9 {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            DMIC0: DMIC0 {
                _marker: PhantomData,
            },
            ENET: ENET {
                _marker: PhantomData,
            },
            USBHSD: USBHSD {
                _marker: PhantomData,
            },
            CRC_ENGINE: CRC_ENGINE {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            I2S1: I2S1 {
                _marker: PhantomData,
            },
            SDIF: SDIF {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            USBFSH: USBFSH {
                _marker: PhantomData,
            },
            USBHSH: USBHSH {
                _marker: PhantomData,
            },
            SHA0: SHA0 {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            ETM: ETM {
                _marker: PhantomData,
            },
        }
    }
}
