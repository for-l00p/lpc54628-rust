#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - AHB multilayer matrix priority control"]
    pub ahbmatprio: AHBMATPRIO,
    _reserved1: [u8; 44usize],
    #[doc = "0x40 - System tick counter calibration"]
    pub systckcal: SYSTCKCAL,
    _reserved2: [u8; 4usize],
    #[doc = "0x48 - NMI Source Select"]
    pub nmisrc: NMISRC,
    #[doc = "0x4c - Asynchronous APB Control"]
    pub asyncapbctrl: ASYNCAPBCTRL,
    _reserved4: [u8; 112usize],
    #[doc = "0xc0 - POR captured value of port n"]
    pub pioporcap: [PIOPORCAP; 2],
    _reserved5: [u8; 8usize],
    #[doc = "0xd0 - Reset captured value of port n"]
    pub piorescap: [PIORESCAP; 2],
    _reserved6: [u8; 40usize],
    #[doc = "0x100 - Peripheral reset control n"]
    pub presetctrl0: PRESETCTRL0,
    #[doc = "0x104 - Peripheral reset control n"]
    pub presetctrl1: PRESETCTRL1,
    #[doc = "0x108 - Peripheral reset control n"]
    pub presetctrl2: PRESETCTRL2,
    _reserved9: [u8; 20usize],
    #[doc = "0x120 - Set bits in PRESETCTRLn"]
    pub presetctrlset: [PRESETCTRLSET; 3],
    _reserved10: [u8; 20usize],
    #[doc = "0x140 - Clear bits in PRESETCTRLn"]
    pub presetctrlclr: [PRESETCTRLCLR; 3],
    _reserved11: [u8; 164usize],
    #[doc = "0x1f0 - System reset status register"]
    pub sysrststat: SYSRSTSTAT,
    _reserved12: [u8; 12usize],
    #[doc = "0x200 - AHB Clock control n"]
    pub ahbclkctrl0: AHBCLKCTRL0,
    #[doc = "0x204 - AHB Clock control n"]
    pub ahbclkctrl1: AHBCLKCTRL1,
    #[doc = "0x208 - AHB Clock control n"]
    pub ahbclkctrl2: AHBCLKCTRL2,
    _reserved15: [u8; 20usize],
    #[doc = "0x220 - Set bits in AHBCLKCTRLn"]
    pub ahbclkctrlset: [AHBCLKCTRLSET; 3],
    _reserved16: [u8; 20usize],
    #[doc = "0x240 - Clear bits in AHBCLKCTRLn"]
    pub ahbclkctrlclr: [AHBCLKCTRLCLR; 3],
    _reserved17: [u8; 52usize],
    #[doc = "0x280 - Main clock source select A"]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x284 - Main clock source select B"]
    pub mainclkselb: MAINCLKSELB,
    #[doc = "0x288 - CLKOUT clock source select A"]
    pub clkoutsela: CLKOUTSELA,
    _reserved20: [u8; 4usize],
    #[doc = "0x290 - PLL clock source select"]
    pub syspllclksel: SYSPLLCLKSEL,
    _reserved21: [u8; 4usize],
    #[doc = "0x298 - Audio PLL clock source select"]
    pub audpllclksel: AUDPLLCLKSEL,
    _reserved22: [u8; 4usize],
    #[doc = "0x2a0 - SPIFI clock source select"]
    pub spificlksel: SPIFICLKSEL,
    #[doc = "0x2a4 - ADC clock source select"]
    pub adcclksel: ADCCLKSEL,
    #[doc = "0x2a8 - USB0 clock source select"]
    pub usb0clksel: USB0CLKSEL,
    #[doc = "0x2ac - USB1 clock source select"]
    pub usb1clksel: USB1CLKSEL,
    #[doc = "0x2b0 - Flexcomm 0 clock source select"]
    pub fclksel: [FCLKSEL; 10],
    _reserved27: [u8; 8usize],
    #[doc = "0x2e0 - MCLK clock source select"]
    pub mclkclksel: MCLKCLKSEL,
    _reserved28: [u8; 4usize],
    #[doc = "0x2e8 - Fractional Rate Generator clock source select"]
    pub frgclksel: FRGCLKSEL,
    #[doc = "0x2ec - Digital microphone (DMIC) subsystem clock select"]
    pub dmicclksel: DMICCLKSEL,
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    pub sctclksel: SCTCLKSEL,
    #[doc = "0x2f4 - LCD clock source select"]
    pub lcdclksel: LCDCLKSEL,
    #[doc = "0x2f8 - SDIO clock source select"]
    pub sdioclksel: SDIOCLKSEL,
    _reserved33: [u8; 4usize],
    #[doc = "0x300 - SYSTICK clock divider"]
    pub systickclkdiv: SYSTICKCLKDIV,
    #[doc = "0x304 - ARM Trace clock divider"]
    pub armtraceclkdiv: ARMTRACECLKDIV,
    #[doc = "0x308 - MCAN0 clock divider"]
    pub can0clkdiv: CAN0CLKDIV,
    #[doc = "0x30c - MCAN1 clock divider"]
    pub can1clkdiv: CAN1CLKDIV,
    #[doc = "0x310 - Smartcard0 clock divider"]
    pub sc0clkdiv: SC0CLKDIV,
    #[doc = "0x314 - Smartcard1 clock divider"]
    pub sc1clkdiv: SC1CLKDIV,
    _reserved39: [u8; 104usize],
    #[doc = "0x380 - AHB clock divider"]
    pub ahbclkdiv: AHBCLKDIV,
    #[doc = "0x384 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    #[doc = "0x388 - FROHF clock divider"]
    pub frohfclkdiv: FROHFCLKDIV,
    _reserved42: [u8; 4usize],
    #[doc = "0x390 - SPIFI clock divider"]
    pub spificlkdiv: SPIFICLKDIV,
    #[doc = "0x394 - ADC clock divider"]
    pub adcclkdiv: ADCCLKDIV,
    #[doc = "0x398 - USB0 clock divider"]
    pub usb0clkdiv: USB0CLKDIV,
    #[doc = "0x39c - USB1 clock divider"]
    pub usb1clkdiv: USB1CLKDIV,
    #[doc = "0x3a0 - Fractional rate divider"]
    pub frgctrl: FRGCTRL,
    _reserved47: [u8; 4usize],
    #[doc = "0x3a8 - DMIC clock divider"]
    pub dmicclkdiv: DMICCLKDIV,
    #[doc = "0x3ac - I2S MCLK clock divider"]
    pub mclkdiv: MCLKDIV,
    #[doc = "0x3b0 - LCD clock divider"]
    pub lcdclkdiv: LCDCLKDIV,
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    pub sctclkdiv: SCTCLKDIV,
    #[doc = "0x3b8 - EMC clock divider"]
    pub emcclkdiv: EMCCLKDIV,
    #[doc = "0x3bc - SDIO clock divider"]
    pub sdioclkdiv: SDIOCLKDIV,
    _reserved53: [u8; 64usize],
    #[doc = "0x400 - Flash wait states configuration"]
    pub flashcfg: FLASHCFG,
    _reserved54: [u8; 8usize],
    #[doc = "0x40c - USB0 clock control"]
    pub usb0clkctrl: USB0CLKCTRL,
    #[doc = "0x410 - USB0 clock status"]
    pub usb0clkstat: USB0CLKSTAT,
    _reserved56: [u8; 4usize],
    #[doc = "0x418 - Frequency measure register"]
    pub freqmectrl: FREQMECTRL,
    _reserved57: [u8; 4usize],
    #[doc = "0x420 - MCLK input/output control"]
    pub mclkio: MCLKIO,
    #[doc = "0x424 - USB1 clock control"]
    pub usb1clkctrl: USB1CLKCTRL,
    #[doc = "0x428 - USB1 clock status"]
    pub usb1clkstat: USB1CLKSTAT,
    _reserved60: [u8; 24usize],
    #[doc = "0x444 - EMC system control"]
    pub emcsysctrl: EMCSYSCTRL,
    #[doc = "0x448 - EMC clock delay control"]
    pub emcdlyctrl: EMCDLYCTRL,
    #[doc = "0x44c - EMC delay chain calibration control"]
    pub emcdlycal: EMCDLYCAL,
    #[doc = "0x450 - Ethernet PHY Selection"]
    pub ethphysel: ETHPHYSEL,
    #[doc = "0x454 - Ethernet SBD flow control"]
    pub ethsbdctrl: ETHSBDCTRL,
    _reserved65: [u8; 8usize],
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    pub sdioclkctrl: SDIOCLKCTRL,
    _reserved66: [u8; 156usize],
    #[doc = "0x500 - FRO oscillator control"]
    pub froctrl: FROCTRL,
    #[doc = "0x504 - System oscillator control"]
    pub sysoscctrl: SYSOSCCTRL,
    #[doc = "0x508 - Watchdog oscillator control"]
    pub wdtoscctrl: WDTOSCCTRL,
    #[doc = "0x50c - RTC oscillator 32 kHz output control"]
    pub rtcoscctrl: RTCOSCCTRL,
    _reserved70: [u8; 12usize],
    #[doc = "0x51c - USB PLL control"]
    pub usbpllctrl: USBPLLCTRL,
    #[doc = "0x520 - USB PLL status"]
    pub usbpllstat: USBPLLSTAT,
    _reserved72: [u8; 92usize],
    #[doc = "0x580 - System PLL control"]
    pub syspllctrl: SYSPLLCTRL,
    #[doc = "0x584 - PLL status"]
    pub syspllstat: SYSPLLSTAT,
    #[doc = "0x588 - PLL N divider"]
    pub syspllndec: SYSPLLNDEC,
    #[doc = "0x58c - PLL P divider"]
    pub syspllpdec: SYSPLLPDEC,
    #[doc = "0x590 - System PLL M divider"]
    pub syspllmdec: SYSPLLMDEC,
    _reserved77: [u8; 12usize],
    #[doc = "0x5a0 - Audio PLL control"]
    pub audpllctrl: AUDPLLCTRL,
    #[doc = "0x5a4 - Audio PLL status"]
    pub audpllstat: AUDPLLSTAT,
    #[doc = "0x5a8 - Audio PLL N divider"]
    pub audpllndec: AUDPLLNDEC,
    #[doc = "0x5ac - Audio PLL P divider"]
    pub audpllpdec: AUDPLLPDEC,
    #[doc = "0x5b0 - Audio PLL M divider"]
    pub audpllmdec: AUDPLLMDEC,
    #[doc = "0x5b4 - Audio PLL fractional divider control"]
    pub audpllfrac: AUDPLLFRAC,
    _reserved83: [u8; 72usize],
    #[doc = "0x600 - Sleep configuration register"]
    pub pdsleepcfg0: PDSLEEPCFG0,
    #[doc = "0x604 - Sleep configuration register"]
    pub pdsleepcfg1: PDSLEEPCFG1,
    _reserved85: [u8; 8usize],
    #[doc = "0x610 - Power configuration register"]
    pub pdruncfg0: PDRUNCFG0,
    #[doc = "0x614 - Power configuration register"]
    pub pdruncfg1: PDRUNCFG1,
    _reserved87: [u8; 8usize],
    #[doc = "0x620 - Power configuration set register"]
    pub pdruncfgset0: PDRUNCFGSET0,
    #[doc = "0x624 - Power configuration set register"]
    pub pdruncfgset1: PDRUNCFGSET1,
    _reserved89: [u8; 8usize],
    #[doc = "0x630 - Power configuration clear register"]
    pub pdruncfgclr0: PDRUNCFGCLR0,
    #[doc = "0x634 - Power configuration clear register"]
    pub pdruncfgclr1: PDRUNCFGCLR1,
    _reserved91: [u8; 72usize],
    #[doc = "0x680 - Start logic 0 wake-up enable register"]
    pub starter0: STARTER0,
    #[doc = "0x684 - Start logic 0 wake-up enable register"]
    pub starter1: STARTER1,
    _reserved93: [u8; 24usize],
    #[doc = "0x6a0 - Set bits in STARTER"]
    pub starterset: [STARTERSET; 2],
    _reserved94: [u8; 24usize],
    #[doc = "0x6c0 - Clear bits in STARTER0"]
    pub starterclr: [STARTERCLR; 2],
    _reserved95: [u8; 184usize],
    #[doc = "0x780 - Configures special cases of hardware wake-up"]
    pub hwwake: HWWAKE,
    _reserved96: [u8; 1664usize],
    #[doc = "0xe04 - Auto Clock-Gate Override Register"]
    pub autocgor: AUTOCGOR,
    _reserved97: [u8; 492usize],
    #[doc = "0xff4 - JTAG ID code register"]
    pub jtagidcode: JTAGIDCODE,
    #[doc = "0xff8 - Part ID register"]
    pub device_id0: DEVICE_ID0,
    #[doc = "0xffc - Boot ROM and die revision register"]
    pub device_id1: DEVICE_ID1,
    _reserved100: [u8; 127044usize],
    #[doc = "0x20044 - Brown-Out Detect control"]
    pub bodctrl: BODCTRL,
}
#[doc = "AHB multilayer matrix priority control"]
pub struct AHBMATPRIO {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AHB multilayer matrix priority control"]
pub mod ahbmatprio;
#[doc = "System tick counter calibration"]
pub struct SYSTCKCAL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System tick counter calibration"]
pub mod systckcal;
#[doc = "NMI Source Select"]
pub struct NMISRC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "Asynchronous APB Control"]
pub struct ASYNCAPBCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Asynchronous APB Control"]
pub mod asyncapbctrl;
#[doc = "POR captured value of port n"]
pub struct PIOPORCAP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "POR captured value of port n"]
pub mod pioporcap;
#[doc = "Reset captured value of port n"]
pub struct PIORESCAP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Reset captured value of port n"]
pub mod piorescap;
#[doc = "Peripheral reset control n"]
pub struct PRESETCTRL0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control n"]
pub mod presetctrl0;
#[doc = "Peripheral reset control n"]
pub struct PRESETCTRL1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control n"]
pub mod presetctrl1;
#[doc = "Peripheral reset control n"]
pub struct PRESETCTRL2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Peripheral reset control n"]
pub mod presetctrl2;
#[doc = "Set bits in PRESETCTRLn"]
pub struct PRESETCTRLSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set bits in PRESETCTRLn"]
pub mod presetctrlset;
#[doc = "Clear bits in PRESETCTRLn"]
pub struct PRESETCTRLCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in PRESETCTRLn"]
pub mod presetctrlclr;
#[doc = "System reset status register"]
pub struct SYSRSTSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System reset status register"]
pub mod sysrststat;
#[doc = "AHB Clock control n"]
pub struct AHBCLKCTRL0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AHB Clock control n"]
pub mod ahbclkctrl0;
#[doc = "AHB Clock control n"]
pub struct AHBCLKCTRL1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AHB Clock control n"]
pub mod ahbclkctrl1;
#[doc = "AHB Clock control n"]
pub struct AHBCLKCTRL2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AHB Clock control n"]
pub mod ahbclkctrl2;
#[doc = "Set bits in AHBCLKCTRLn"]
pub struct AHBCLKCTRLSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set bits in AHBCLKCTRLn"]
pub mod ahbclkctrlset;
#[doc = "Clear bits in AHBCLKCTRLn"]
pub struct AHBCLKCTRLCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in AHBCLKCTRLn"]
pub mod ahbclkctrlclr;
#[doc = "Main clock source select A"]
pub struct MAINCLKSELA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select A"]
pub mod mainclksela;
#[doc = "Main clock source select B"]
pub struct MAINCLKSELB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Main clock source select B"]
pub mod mainclkselb;
#[doc = "CLKOUT clock source select A"]
pub struct CLKOUTSELA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock source select A"]
pub mod clkoutsela;
#[doc = "PLL clock source select"]
pub struct SYSPLLCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "PLL clock source select"]
pub mod syspllclksel;
#[doc = "Audio PLL clock source select"]
pub struct AUDPLLCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL clock source select"]
pub mod audpllclksel;
#[doc = "SPIFI clock source select"]
pub struct SPIFICLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SPIFI clock source select"]
pub mod spificlksel;
#[doc = "ADC clock source select"]
pub struct ADCCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "USB0 clock source select"]
pub struct USB0CLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB0 clock source select"]
pub mod usb0clksel;
#[doc = "USB1 clock source select"]
pub struct USB1CLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB1 clock source select"]
pub mod usb1clksel;
#[doc = "Flexcomm 0 clock source select"]
pub struct FCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Flexcomm 0 clock source select"]
pub mod fclksel;
#[doc = "MCLK clock source select"]
pub struct MCLKCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "Fractional Rate Generator clock source select"]
pub struct FRGCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fractional Rate Generator clock source select"]
pub mod frgclksel;
#[doc = "Digital microphone (DMIC) subsystem clock select"]
pub struct DMICCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Digital microphone (DMIC) subsystem clock select"]
pub mod dmicclksel;
#[doc = "SCTimer/PWM clock source select"]
pub struct SCTCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "LCD clock source select"]
pub struct LCDCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LCD clock source select"]
pub mod lcdclksel;
#[doc = "SDIO clock source select"]
pub struct SDIOCLKSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "SYSTICK clock divider"]
pub struct SYSTICKCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SYSTICK clock divider"]
pub mod systickclkdiv;
#[doc = "ARM Trace clock divider"]
pub struct ARMTRACECLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ARM Trace clock divider"]
pub mod armtraceclkdiv;
#[doc = "MCAN0 clock divider"]
pub struct CAN0CLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MCAN0 clock divider"]
pub mod can0clkdiv;
#[doc = "MCAN1 clock divider"]
pub struct CAN1CLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MCAN1 clock divider"]
pub mod can1clkdiv;
#[doc = "Smartcard0 clock divider"]
pub struct SC0CLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Smartcard0 clock divider"]
pub mod sc0clkdiv;
#[doc = "Smartcard1 clock divider"]
pub struct SC1CLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Smartcard1 clock divider"]
pub mod sc1clkdiv;
#[doc = "AHB clock divider"]
pub struct AHBCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AHB clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUT clock divider"]
pub struct CLKOUTDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FROHF clock divider"]
pub struct FROHFCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FROHF clock divider"]
pub mod frohfclkdiv;
#[doc = "SPIFI clock divider"]
pub struct SPIFICLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SPIFI clock divider"]
pub mod spificlkdiv;
#[doc = "ADC clock divider"]
pub struct ADCCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0 clock divider"]
pub struct USB0CLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB0 clock divider"]
pub mod usb0clkdiv;
#[doc = "USB1 clock divider"]
pub struct USB1CLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB1 clock divider"]
pub mod usb1clkdiv;
#[doc = "Fractional rate divider"]
pub struct FRGCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Fractional rate divider"]
pub mod frgctrl;
#[doc = "DMIC clock divider"]
pub struct DMICCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "DMIC clock divider"]
pub mod dmicclkdiv;
#[doc = "I2S MCLK clock divider"]
pub struct MCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "LCD clock divider"]
pub struct LCDCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "LCD clock divider"]
pub mod lcdclkdiv;
#[doc = "SCT/PWM clock divider"]
pub struct SCTCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "EMC clock divider"]
pub struct EMCCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "EMC clock divider"]
pub mod emcclkdiv;
#[doc = "SDIO clock divider"]
pub struct SDIOCLKDIV {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "Flash wait states configuration"]
pub struct FLASHCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Flash wait states configuration"]
pub mod flashcfg;
#[doc = "USB0 clock control"]
pub struct USB0CLKCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB0 clock control"]
pub mod usb0clkctrl;
#[doc = "USB0 clock status"]
pub struct USB0CLKSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB0 clock status"]
pub mod usb0clkstat;
#[doc = "Frequency measure register"]
pub struct FREQMECTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Frequency measure register"]
pub mod freqmectrl;
#[doc = "MCLK input/output control"]
pub struct MCLKIO {
    register: vcell::VolatileCell<u32>,
}
#[doc = "MCLK input/output control"]
pub mod mclkio;
#[doc = "USB1 clock control"]
pub struct USB1CLKCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB1 clock control"]
pub mod usb1clkctrl;
#[doc = "USB1 clock status"]
pub struct USB1CLKSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB1 clock status"]
pub mod usb1clkstat;
#[doc = "EMC system control"]
pub struct EMCSYSCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "EMC system control"]
pub mod emcsysctrl;
#[doc = "EMC clock delay control"]
pub struct EMCDLYCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "EMC clock delay control"]
pub mod emcdlyctrl;
#[doc = "EMC delay chain calibration control"]
pub struct EMCDLYCAL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "EMC delay chain calibration control"]
pub mod emcdlycal;
#[doc = "Ethernet PHY Selection"]
pub struct ETHPHYSEL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Ethernet PHY Selection"]
pub mod ethphysel;
#[doc = "Ethernet SBD flow control"]
pub struct ETHSBDCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Ethernet SBD flow control"]
pub mod ethsbdctrl;
#[doc = "SDIO CCLKIN phase and delay control"]
pub struct SDIOCLKCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "FRO oscillator control"]
pub struct FROCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "FRO oscillator control"]
pub mod froctrl;
#[doc = "System oscillator control"]
pub struct SYSOSCCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System oscillator control"]
pub mod sysoscctrl;
#[doc = "Watchdog oscillator control"]
pub struct WDTOSCCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Watchdog oscillator control"]
pub mod wdtoscctrl;
#[doc = "RTC oscillator 32 kHz output control"]
pub struct RTCOSCCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RTC oscillator 32 kHz output control"]
pub mod rtcoscctrl;
#[doc = "USB PLL control"]
pub struct USBPLLCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB PLL control"]
pub mod usbpllctrl;
#[doc = "USB PLL status"]
pub struct USBPLLSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "USB PLL status"]
pub mod usbpllstat;
#[doc = "System PLL control"]
pub struct SYSPLLCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System PLL control"]
pub mod syspllctrl;
#[doc = "PLL status"]
pub struct SYSPLLSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "PLL status"]
pub mod syspllstat;
#[doc = "PLL N divider"]
pub struct SYSPLLNDEC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "PLL N divider"]
pub mod syspllndec;
#[doc = "PLL P divider"]
pub struct SYSPLLPDEC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "PLL P divider"]
pub mod syspllpdec;
#[doc = "System PLL M divider"]
pub struct SYSPLLMDEC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "System PLL M divider"]
pub mod syspllmdec;
#[doc = "Audio PLL control"]
pub struct AUDPLLCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL control"]
pub mod audpllctrl;
#[doc = "Audio PLL status"]
pub struct AUDPLLSTAT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL status"]
pub mod audpllstat;
#[doc = "Audio PLL N divider"]
pub struct AUDPLLNDEC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL N divider"]
pub mod audpllndec;
#[doc = "Audio PLL P divider"]
pub struct AUDPLLPDEC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL P divider"]
pub mod audpllpdec;
#[doc = "Audio PLL M divider"]
pub struct AUDPLLMDEC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL M divider"]
pub mod audpllmdec;
#[doc = "Audio PLL fractional divider control"]
pub struct AUDPLLFRAC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Audio PLL fractional divider control"]
pub mod audpllfrac;
#[doc = "Sleep configuration register"]
pub struct PDSLEEPCFG0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Sleep configuration register"]
pub mod pdsleepcfg0;
#[doc = "Sleep configuration register"]
pub struct PDSLEEPCFG1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Sleep configuration register"]
pub mod pdsleepcfg1;
#[doc = "Power configuration register"]
pub struct PDRUNCFG0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Power configuration register"]
pub mod pdruncfg0;
#[doc = "Power configuration register"]
pub struct PDRUNCFG1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Power configuration register"]
pub mod pdruncfg1;
#[doc = "Power configuration set register"]
pub struct PDRUNCFGSET0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Power configuration set register"]
pub mod pdruncfgset0;
#[doc = "Power configuration set register"]
pub struct PDRUNCFGSET1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Power configuration set register"]
pub mod pdruncfgset1;
#[doc = "Power configuration clear register"]
pub struct PDRUNCFGCLR0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Power configuration clear register"]
pub mod pdruncfgclr0;
#[doc = "Power configuration clear register"]
pub struct PDRUNCFGCLR1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Power configuration clear register"]
pub mod pdruncfgclr1;
#[doc = "Start logic 0 wake-up enable register"]
pub struct STARTER0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Start logic 0 wake-up enable register"]
pub mod starter0;
#[doc = "Start logic 0 wake-up enable register"]
pub struct STARTER1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Start logic 0 wake-up enable register"]
pub mod starter1;
#[doc = "Set bits in STARTER"]
pub struct STARTERSET {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Set bits in STARTER"]
pub mod starterset;
#[doc = "Clear bits in STARTER0"]
pub struct STARTERCLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Clear bits in STARTER0"]
pub mod starterclr;
#[doc = "Configures special cases of hardware wake-up"]
pub struct HWWAKE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Configures special cases of hardware wake-up"]
pub mod hwwake;
#[doc = "Auto Clock-Gate Override Register"]
pub struct AUTOCGOR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Auto Clock-Gate Override Register"]
pub mod autocgor;
#[doc = "JTAG ID code register"]
pub struct JTAGIDCODE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "JTAG ID code register"]
pub mod jtagidcode;
#[doc = "Part ID register"]
pub struct DEVICE_ID0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Part ID register"]
pub mod device_id0;
#[doc = "Boot ROM and die revision register"]
pub struct DEVICE_ID1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Boot ROM and die revision register"]
pub mod device_id1;
#[doc = "Brown-Out Detect control"]
pub struct BODCTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Brown-Out Detect control"]
pub mod bodctrl;
