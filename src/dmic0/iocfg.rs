#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCFG {
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
#[doc = r"Reader of the field"]
pub type CLK_BYPASS0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLK_BYPASS0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_BYPASS0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLK_BYPASS1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLK_BYPASS1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_BYPASS1W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type STEREO_DATA0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STEREO_DATA0W<'a> {
    w: &'a mut W,
}
impl<'a> _STEREO_DATA0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass0(&self) -> CLK_BYPASS0_R {
        CLK_BYPASS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass1(&self) -> CLK_BYPASS1_R {
        CLK_BYPASS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
    #[inline(always)]
    pub fn stereo_data0(&self) -> STEREO_DATA0_R {
        STEREO_DATA0_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass0(&mut self) -> _CLK_BYPASS0W {
        _CLK_BYPASS0W { w: self }
    }
    #[doc = "Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass1(&mut self) -> _CLK_BYPASS1W {
        _CLK_BYPASS1W { w: self }
    }
    #[doc = "Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
    #[inline(always)]
    pub fn stereo_data0(&mut self) -> _STEREO_DATA0W {
        _STEREO_DATA0W { w: self }
    }
}
