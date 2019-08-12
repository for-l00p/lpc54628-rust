#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCLKSEL {
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
    #[doc = "FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ,
    #[doc = "FRO HF DIV (fro_hf_div)"]
    FRO_HF_DIV,
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT,
    #[doc = "MCLK pin input, when selected in IOCON (mclk_in)"]
    MCLK_INPUT,
    #[doc = "FRG clock, the output of the fractional rate generator (frg_clk)"]
    FRG_CLOCK_OUTPUT,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::FRO_12_MHZ => 0,
            SELR::FRO_HF_DIV => 1,
            SELR::AUDIO_PLL_OUTPUT => 2,
            SELR::MCLK_INPUT => 3,
            SELR::FRG_CLOCK_OUTPUT => 4,
            SELR::NONE => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<u8, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        *self == SELR::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `FRO_HF_DIV`"]
    #[inline(always)]
    pub fn is_fro_hf_div(&self) -> bool {
        *self == SELR::FRO_HF_DIV
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_audio_pll_output(&self) -> bool {
        *self == SELR::AUDIO_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `MCLK_INPUT`"]
    #[inline(always)]
    pub fn is_mclk_input(&self) -> bool {
        *self == SELR::MCLK_INPUT
    }
    #[doc = "Checks if the value of the field is `FRG_CLOCK_OUTPUT`"]
    #[inline(always)]
    pub fn is_frg_clock_output(&self) -> bool {
        *self == SELR::FRG_CLOCK_OUTPUT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SELR::NONE
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ,
    #[doc = "FRO HF DIV (fro_hf_div)"]
    FRO_HF_DIV,
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_OUTPUT,
    #[doc = "MCLK pin input, when selected in IOCON (mclk_in)"]
    MCLK_INPUT,
    #[doc = "FRG clock, the output of the fractional rate generator (frg_clk)"]
    FRG_CLOCK_OUTPUT,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::FRO_12_MHZ => 0,
            SELW::FRO_HF_DIV => 1,
            SELW::AUDIO_PLL_OUTPUT => 2,
            SELW::MCLK_INPUT => 3,
            SELW::FRG_CLOCK_OUTPUT => 4,
            SELW::NONE => 7,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SELW::FRO_12_MHZ)
    }
    #[doc = "FRO HF DIV (fro_hf_div)"]
    #[inline(always)]
    pub fn fro_hf_div(self) -> &'a mut W {
        self.variant(SELW::FRO_HF_DIV)
    }
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    #[inline(always)]
    pub fn audio_pll_output(self) -> &'a mut W {
        self.variant(SELW::AUDIO_PLL_OUTPUT)
    }
    #[doc = "MCLK pin input, when selected in IOCON (mclk_in)"]
    #[inline(always)]
    pub fn mclk_input(self) -> &'a mut W {
        self.variant(SELW::MCLK_INPUT)
    }
    #[doc = "FRG clock, the output of the fractional rate generator (frg_clk)"]
    #[inline(always)]
    pub fn frg_clock_output(self) -> &'a mut W {
        self.variant(SELW::FRG_CLOCK_OUTPUT)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SELW::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:2 - Flexcomm clock source selection. One per Flexcomm."]
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
    #[doc = "Bits 0:2 - Flexcomm clock source selection. One per Flexcomm."]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
