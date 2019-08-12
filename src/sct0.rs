#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit event select register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt event select register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop event select register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start event select register"]
    pub start: START,
    _reserved6: [u8; 40usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    #[doc = "0x5c - SCT DMA request 0 register"]
    pub dma0request: DMA0REQUEST,
    #[doc = "0x60 - SCT DMA request 1 register"]
    pub dma1request: DMA1REQUEST,
    _reserved15: [u8; 140usize],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    _reserved_19_sctcap0: [u8; 4usize],
    _reserved_20_sctcap1: [u8; 4usize],
    _reserved_21_sctcap2: [u8; 4usize],
    _reserved_22_sctcap3: [u8; 4usize],
    _reserved_23_sctcap4: [u8; 4usize],
    _reserved_24_sctcap5: [u8; 4usize],
    _reserved_25_sctcap6: [u8; 4usize],
    _reserved_26_sctcap7: [u8; 4usize],
    _reserved_27_sctcap8: [u8; 4usize],
    _reserved_28_sctcap9: [u8; 4usize],
    _reserved29: [u8; 216usize],
    _reserved_29_sctcapctrl0: [u8; 4usize],
    _reserved_30_sctcapctrl1: [u8; 4usize],
    _reserved_31_sctcapctrl2: [u8; 4usize],
    _reserved_32_sctcapctrl3: [u8; 4usize],
    _reserved_33_sctcapctrl4: [u8; 4usize],
    _reserved_34_sctcapctrl5: [u8; 4usize],
    _reserved_35_sctcapctrl6: [u8; 4usize],
    _reserved_36_sctcapctrl7: [u8; 4usize],
    _reserved_37_sctcapctrl8: [u8; 4usize],
    _reserved_38_sctcapctrl9: [u8; 4usize],
    _reserved39: [u8; 216usize],
    #[doc = "0x300 - no description available"]
    pub event: [EVENT; 10],
    _reserved40: [u8; 432usize],
    #[doc = "0x500 - no description available"]
    pub out: [OUT; 10],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch0(&self) -> &SCTMATCH0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const SCTMATCH0) }
    }
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch0_mut(&self) -> &mut SCTMATCH0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut SCTMATCH0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap0(&self) -> &SCTCAP0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const SCTCAP0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap0_mut(&self) -> &mut SCTCAP0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut SCTCAP0) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch1(&self) -> &SCTMATCH1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const SCTMATCH1) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch1_mut(&self) -> &mut SCTMATCH1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut SCTMATCH1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap1(&self) -> &SCTCAP1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const SCTCAP1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap1_mut(&self) -> &mut SCTCAP1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut SCTCAP1) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch2(&self) -> &SCTMATCH2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const SCTMATCH2) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch2_mut(&self) -> &mut SCTMATCH2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut SCTMATCH2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap2(&self) -> &SCTCAP2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const SCTCAP2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap2_mut(&self) -> &mut SCTCAP2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut SCTCAP2) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch3(&self) -> &SCTMATCH3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const SCTMATCH3) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch3_mut(&self) -> &mut SCTMATCH3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut SCTMATCH3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap3(&self) -> &SCTCAP3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const SCTCAP3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap3_mut(&self) -> &mut SCTCAP3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut SCTCAP3) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch4(&self) -> &SCTMATCH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const SCTMATCH4) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch4_mut(&self) -> &mut SCTMATCH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut SCTMATCH4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap4(&self) -> &SCTCAP4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const SCTCAP4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap4_mut(&self) -> &mut SCTCAP4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut SCTCAP4) }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch5(&self) -> &SCTMATCH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const SCTMATCH5) }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch5_mut(&self) -> &mut SCTMATCH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut SCTMATCH5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap5(&self) -> &SCTCAP5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const SCTCAP5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap5_mut(&self) -> &mut SCTCAP5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut SCTCAP5) }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch6(&self) -> &SCTMATCH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const SCTMATCH6) }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch6_mut(&self) -> &mut SCTMATCH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut SCTMATCH6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap6(&self) -> &SCTCAP6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const SCTCAP6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap6_mut(&self) -> &mut SCTCAP6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut SCTCAP6) }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch7(&self) -> &SCTMATCH7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const SCTMATCH7) }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch7_mut(&self) -> &mut SCTMATCH7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut SCTMATCH7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap7(&self) -> &SCTCAP7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const SCTCAP7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap7_mut(&self) -> &mut SCTCAP7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut SCTCAP7) }
    }
    #[doc = "0x120 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch8(&self) -> &SCTMATCH8 {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const SCTMATCH8) }
    }
    #[doc = "0x120 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch8_mut(&self) -> &mut SCTMATCH8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(288usize) as *mut SCTMATCH8) }
    }
    #[doc = "0x120 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap8(&self) -> &SCTCAP8 {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const SCTCAP8) }
    }
    #[doc = "0x120 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap8_mut(&self) -> &mut SCTCAP8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(288usize) as *mut SCTCAP8) }
    }
    #[doc = "0x124 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch9(&self) -> &SCTMATCH9 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const SCTMATCH9) }
    }
    #[doc = "0x124 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch9_mut(&self) -> &mut SCTMATCH9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(292usize) as *mut SCTMATCH9) }
    }
    #[doc = "0x124 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap9(&self) -> &SCTCAP9 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const SCTCAP9) }
    }
    #[doc = "0x124 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap9_mut(&self) -> &mut SCTCAP9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(292usize) as *mut SCTCAP9) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel0(&self) -> &SCTMATCHREL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const SCTMATCHREL0) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel0_mut(&self) -> &mut SCTMATCHREL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut SCTMATCHREL0) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl0(&self) -> &SCTCAPCTRL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const SCTCAPCTRL0) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl0_mut(&self) -> &mut SCTCAPCTRL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut SCTCAPCTRL0) }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel1(&self) -> &SCTMATCHREL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const SCTMATCHREL1) }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel1_mut(&self) -> &mut SCTMATCHREL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut SCTMATCHREL1) }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl1(&self) -> &SCTCAPCTRL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const SCTCAPCTRL1) }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl1_mut(&self) -> &mut SCTCAPCTRL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut SCTCAPCTRL1) }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel2(&self) -> &SCTMATCHREL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const SCTMATCHREL2) }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel2_mut(&self) -> &mut SCTMATCHREL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(520usize) as *mut SCTMATCHREL2) }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl2(&self) -> &SCTCAPCTRL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const SCTCAPCTRL2) }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl2_mut(&self) -> &mut SCTCAPCTRL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(520usize) as *mut SCTCAPCTRL2) }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel3(&self) -> &SCTMATCHREL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const SCTMATCHREL3) }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel3_mut(&self) -> &mut SCTMATCHREL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(524usize) as *mut SCTMATCHREL3) }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl3(&self) -> &SCTCAPCTRL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const SCTCAPCTRL3) }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl3_mut(&self) -> &mut SCTCAPCTRL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(524usize) as *mut SCTCAPCTRL3) }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel4(&self) -> &SCTMATCHREL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const SCTMATCHREL4) }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel4_mut(&self) -> &mut SCTMATCHREL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut SCTMATCHREL4) }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl4(&self) -> &SCTCAPCTRL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const SCTCAPCTRL4) }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl4_mut(&self) -> &mut SCTCAPCTRL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut SCTCAPCTRL4) }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel5(&self) -> &SCTMATCHREL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const SCTMATCHREL5) }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel5_mut(&self) -> &mut SCTMATCHREL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(532usize) as *mut SCTMATCHREL5) }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl5(&self) -> &SCTCAPCTRL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const SCTCAPCTRL5) }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl5_mut(&self) -> &mut SCTCAPCTRL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(532usize) as *mut SCTCAPCTRL5) }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel6(&self) -> &SCTMATCHREL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const SCTMATCHREL6) }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel6_mut(&self) -> &mut SCTMATCHREL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut SCTMATCHREL6) }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl6(&self) -> &SCTCAPCTRL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const SCTCAPCTRL6) }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl6_mut(&self) -> &mut SCTCAPCTRL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut SCTCAPCTRL6) }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel7(&self) -> &SCTMATCHREL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const SCTMATCHREL7) }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel7_mut(&self) -> &mut SCTMATCHREL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(540usize) as *mut SCTMATCHREL7) }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl7(&self) -> &SCTCAPCTRL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const SCTCAPCTRL7) }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl7_mut(&self) -> &mut SCTCAPCTRL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(540usize) as *mut SCTCAPCTRL7) }
    }
    #[doc = "0x220 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel8(&self) -> &SCTMATCHREL8 {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const SCTMATCHREL8) }
    }
    #[doc = "0x220 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel8_mut(&self) -> &mut SCTMATCHREL8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(544usize) as *mut SCTMATCHREL8) }
    }
    #[doc = "0x220 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl8(&self) -> &SCTCAPCTRL8 {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const SCTCAPCTRL8) }
    }
    #[doc = "0x220 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl8_mut(&self) -> &mut SCTCAPCTRL8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(544usize) as *mut SCTCAPCTRL8) }
    }
    #[doc = "0x224 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel9(&self) -> &SCTMATCHREL9 {
        unsafe { &*(((self as *const Self) as *const u8).add(548usize) as *const SCTMATCHREL9) }
    }
    #[doc = "0x224 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel9_mut(&self) -> &mut SCTMATCHREL9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(548usize) as *mut SCTMATCHREL9) }
    }
    #[doc = "0x224 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl9(&self) -> &SCTCAPCTRL9 {
        unsafe { &*(((self as *const Self) as *const u8).add(548usize) as *const SCTCAPCTRL9) }
    }
    #[doc = "0x224 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl9_mut(&self) -> &mut SCTCAPCTRL9 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(548usize) as *mut SCTCAPCTRL9) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENT {
    #[doc = "0x00 - SCT event state register 0"]
    pub state: self::event::STATE,
    #[doc = "0x04 - SCT event control register 0"]
    pub ctrl: self::event::CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod event;
#[doc = r"Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - SCT output 0 set register"]
    pub set: self::out::SET,
    #[doc = "0x04 - SCT output 0 clear register"]
    pub clr: self::out::CLR,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod out;
#[doc = "SCT configuration register"]
pub struct CONFIG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "SCT control register"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "SCT limit event select register"]
pub struct LIMIT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "SCT halt event select register"]
pub struct HALT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "SCT stop event select register"]
pub struct STOP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "SCT start event select register"]
pub struct START {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "SCT counter register"]
pub struct COUNT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT counter register"]
pub mod count;
#[doc = "SCT state register"]
pub struct STATE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT state register"]
pub mod state;
#[doc = "SCT input register"]
pub struct INPUT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT input register"]
pub mod input;
#[doc = "SCT match/capture mode register"]
pub struct REGMODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "SCT output register"]
pub struct OUTPUT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT output register"]
pub mod output;
#[doc = "SCT output counter direction control register"]
pub struct OUTPUTDIRCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "SCT conflict resolution register"]
pub struct RES {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "SCT DMA request 0 register"]
pub struct DMA0REQUEST {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT DMA request 0 register"]
pub mod dma0request;
#[doc = "SCT DMA request 1 register"]
pub struct DMA1REQUEST {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT DMA request 1 register"]
pub mod dma1request;
#[doc = "SCT event interrupt enable register"]
pub struct EVEN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "SCT event flag register"]
pub struct EVFLAG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "SCT conflict interrupt enable register"]
pub struct CONEN {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "SCT conflict flag register"]
pub struct CONFLAG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap0;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch0;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap1;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch1;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap2;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch2;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap3;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch3;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap4;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch4;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap5;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch5;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap6;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch6;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap7;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch7;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP8 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap8;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH8 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch8;
#[doc = "SCT capture register of capture channel"]
pub struct SCTCAP9 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap9;
#[doc = "SCT match value register of match channels"]
pub struct SCTMATCH9 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch9;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl0;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel0;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl1;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel1;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl2;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel2;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl3;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel3;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl4;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL4 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel4;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl5;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL5 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel5;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl6;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL6 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel6;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl7;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL7 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel7;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL8 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl8;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL8 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel8;
#[doc = "SCT capture control register"]
pub struct SCTCAPCTRL9 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT capture control register"]
pub mod sctcapctrl9;
#[doc = "SCT match reload value register"]
pub struct SCTMATCHREL9 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel9;
