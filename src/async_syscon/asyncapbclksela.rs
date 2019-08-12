#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ASYNCAPBCLKSELA {
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
        0
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
    #[doc = "FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ,
    #[doc = "Audio PLL clock.(AUDPLL_BYPASS)"]
    AUDIO_PLL_CLOCK,
    #[doc = "fc6 fclk (fc6_fclk)"]
    FC6_FCLK,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::MAIN_CLOCK => 0,
            SELR::FRO_12_MHZ => 1,
            SELR::AUDIO_PLL_CLOCK => 2,
            SELR::FC6_FCLK => 3,
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
    #[doc = "Checks if the value of the field is `FRO_12_MHZ`"]
    #[inline(always)]
    pub fn is_fro_12_mhz(&self) -> bool {
        *self == SELR::FRO_12_MHZ
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_audio_pll_clock(&self) -> bool {
        *self == SELR::AUDIO_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `FC6_FCLK`"]
    #[inline(always)]
    pub fn is_fc6_fclk(&self) -> bool {
        *self == SELR::FC6_FCLK
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "Main clock (main_clk)"]
    MAIN_CLOCK,
    #[doc = "FRO 12 MHz (fro_12m)"]
    FRO_12_MHZ,
    #[doc = "Audio PLL clock.(AUDPLL_BYPASS)"]
    AUDIO_PLL_CLOCK,
    #[doc = "fc6 fclk (fc6_fclk)"]
    FC6_FCLK,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::MAIN_CLOCK => 0,
            SELW::FRO_12_MHZ => 1,
            SELW::AUDIO_PLL_CLOCK => 2,
            SELW::FC6_FCLK => 3,
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
    #[doc = "FRO 12 MHz (fro_12m)"]
    #[inline(always)]
    pub fn fro_12_mhz(self) -> &'a mut W {
        self.variant(SELW::FRO_12_MHZ)
    }
    #[doc = "Audio PLL clock.(AUDPLL_BYPASS)"]
    #[inline(always)]
    pub fn audio_pll_clock(self) -> &'a mut W {
        self.variant(SELW::AUDIO_PLL_CLOCK)
    }
    #[doc = "fc6 fclk (fc6_fclk)"]
    #[inline(always)]
    pub fn fc6_fclk(self) -> &'a mut W {
        self.variant(SELW::FC6_FCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock source for asynchronous clock source selector A"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Clock source for asynchronous clock source selector A"]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
