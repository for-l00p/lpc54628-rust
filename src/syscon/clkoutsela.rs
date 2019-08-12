#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTSELA {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0x07
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "Main clock (main_clk)"]
    MAIN_CLOCK,
    #[doc = "CLKIN (clk_in)"]
    CLKIN,
    #[doc = "Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR,
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF,
    #[doc = "PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT,
    #[doc = "USB PLL clock (usb_pll_clk)"]
    USB_PLL_CLOCK,
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT,
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::MAIN_CLOCK => 0,
            SELR::CLKIN => 1,
            SELR::WATCHDOG_OSCILLATOR => 2,
            SELR::FRO_HF => 3,
            SELR::SYSTEM_PLL_OUTPUT => 4,
            SELR::USB_PLL_CLOCK => 5,
            SELR::AUDIO_PLL_OUTPUT => 6,
            SELR::RTC_OSC_OUTPUT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<u8, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SELR::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `CLKIN`"]
    #[inline(always)]
    pub fn is_clkin(&self) -> bool {
        *self == SELR::CLKIN
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SELR::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        *self == SELR::FRO_HF
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        *self == SELR::SYSTEM_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `USB_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_usb_pll_clock(&self) -> bool {
        *self == SELR::USB_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_audio_pll_output(&self) -> bool {
        *self == SELR::AUDIO_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `RTC_OSC_OUTPUT`"]
    #[inline(always)]
    pub fn is_rtc_osc_output(&self) -> bool {
        *self == SELR::RTC_OSC_OUTPUT
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "Main clock (main_clk)"]
    MAIN_CLOCK,
    #[doc = "CLKIN (clk_in)"]
    CLKIN,
    #[doc = "Watchdog oscillator (wdt_clk)"]
    WATCHDOG_OSCILLATOR,
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF,
    #[doc = "PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT,
    #[doc = "USB PLL clock (usb_pll_clk)"]
    USB_PLL_CLOCK,
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT,
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    RTC_OSC_OUTPUT,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::MAIN_CLOCK => 0,
            SELW::CLKIN => 1,
            SELW::WATCHDOG_OSCILLATOR => 2,
            SELW::FRO_HF => 3,
            SELW::SYSTEM_PLL_OUTPUT => 4,
            SELW::USB_PLL_CLOCK => 5,
            SELW::AUDIO_PLL_OUTPUT => 6,
            SELW::RTC_OSC_OUTPUT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main clock (main_clk)"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SELW::MAIN_CLOCK)
    }
    #[doc = "CLKIN (clk_in)"]
    #[inline(always)]
    pub fn clkin(self) -> &'a mut W {
        self.variant(SELW::CLKIN)
    }
    #[doc = "Watchdog oscillator (wdt_clk)"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SELW::WATCHDOG_OSCILLATOR)
    }
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SELW::FRO_HF)
    }
    #[doc = "PLL output (pll_clk)"]
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SELW::SYSTEM_PLL_OUTPUT)
    }
    #[doc = "USB PLL clock (usb_pll_clk)"]
    #[inline(always)]
    pub fn usb_pll_clock(self) -> &'a mut W {
        self.variant(SELW::USB_PLL_CLOCK)
    }
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    #[inline(always)]
    pub fn audio_pll_output(self) -> &'a mut W {
        self.variant(SELW::AUDIO_PLL_OUTPUT)
    }
    #[doc = "RTC oscillator 32 kHz output (32k_clk)"]
    #[inline(always)]
    pub fn rtc_osc_output(self) -> &'a mut W {
        self.variant(SELW::RTC_OSC_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - CLKOUT clock source selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits() & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - CLKOUT clock source selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
