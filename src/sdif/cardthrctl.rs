#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CARDTHRCTL {
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
pub type CARDRDTHREN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CARDRDTHRENW<'a> {
    w: &'a mut W,
}
impl<'a> _CARDRDTHRENW<'a> {
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
pub type BSYCLRINTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BSYCLRINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BSYCLRINTENW<'a> {
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
pub type CARDTHRESHOLD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CARDTHRESHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _CARDTHRESHOLDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&self) -> BSYCLRINTEN_R {
        BSYCLRINTEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&self) -> CARDTHRESHOLD_R {
        CARDTHRESHOLD_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&mut self) -> _CARDRDTHRENW {
        _CARDRDTHRENW { w: self }
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&mut self) -> _BSYCLRINTENW {
        _BSYCLRINTENW { w: self }
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&mut self) -> _CARDTHRESHOLDW {
        _CARDTHRESHOLDW { w: self }
    }
}
