#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_INTR_STAT {
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
pub type DC0IS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DC0ISW<'a> {
    w: &'a mut W,
}
impl<'a> _DC0ISW<'a> {
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
pub type DC1IS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DC1ISW<'a> {
    w: &'a mut W,
}
impl<'a> _DC1ISW<'a> {
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
pub type MTLIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MACIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status This bit indicates an interrupt event in the MTL."]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status This bit indicates an interrupt event in the MAC."]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DMA Channel 0 Interrupt Status This bit indicates an interrupt event in DMA Channel 0."]
    #[inline(always)]
    pub fn dc0is(&mut self) -> _DC0ISW {
        _DC0ISW { w: self }
    }
    #[doc = "Bit 1 - DMA Channel 1 Interrupt Status This bit indicates an interrupt event in DMA Channel 1."]
    #[inline(always)]
    pub fn dc1is(&mut self) -> _DC1ISW {
        _DC1ISW { w: self }
    }
}
