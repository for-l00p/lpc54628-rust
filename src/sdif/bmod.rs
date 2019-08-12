#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BMOD {
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
pub type SWR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
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
pub type FB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FBW<'a> {
    w: &'a mut W,
}
impl<'a> _FBW<'a> {
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
pub type DSL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DSLW<'a> {
    w: &'a mut W,
}
impl<'a> _DSLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PBL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PBLW<'a> {
    w: &'a mut W,
}
impl<'a> _PBLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset."]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fixed Burst."]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length."]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits() >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - SD/MMC DMA Enable."]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length."]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset."]
    #[inline(always)]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bit 1 - Fixed Burst."]
    #[inline(always)]
    pub fn fb(&mut self) -> _FBW {
        _FBW { w: self }
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length."]
    #[inline(always)]
    pub fn dsl(&mut self) -> _DSLW {
        _DSLW { w: self }
    }
    #[doc = "Bit 7 - SD/MMC DMA Enable."]
    #[inline(always)]
    pub fn de(&mut self) -> _DEW {
        _DEW { w: self }
    }
    #[doc = "Bits 8:10 - Programmable Burst Length."]
    #[inline(always)]
    pub fn pbl(&mut self) -> _PBLW {
        _PBLW { w: self }
    }
}
