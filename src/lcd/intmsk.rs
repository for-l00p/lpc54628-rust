#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTMSK {
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
pub type FUFIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FUFIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FUFIMW<'a> {
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
pub type LNBUIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LNBUIMW<'a> {
    w: &'a mut W,
}
impl<'a> _LNBUIMW<'a> {
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
#[doc = r"Reader of the field"]
pub type VCOMPIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VCOMPIMW<'a> {
    w: &'a mut W,
}
impl<'a> _VCOMPIMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BERIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BERIMW<'a> {
    w: &'a mut W,
}
impl<'a> _BERIMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - FIFO underflow interrupt enable."]
    #[inline(always)]
    pub fn fufim(&self) -> FUFIM_R {
        FUFIM_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable."]
    #[inline(always)]
    pub fn lnbuim(&self) -> LNBUIM_R {
        LNBUIM_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable."]
    #[inline(always)]
    pub fn vcompim(&self) -> VCOMPIM_R {
        VCOMPIM_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable."]
    #[inline(always)]
    pub fn berim(&self) -> BERIM_R {
        BERIM_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - FIFO underflow interrupt enable."]
    #[inline(always)]
    pub fn fufim(&mut self) -> _FUFIMW {
        _FUFIMW { w: self }
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable."]
    #[inline(always)]
    pub fn lnbuim(&mut self) -> _LNBUIMW {
        _LNBUIMW { w: self }
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable."]
    #[inline(always)]
    pub fn vcompim(&mut self) -> _VCOMPIMW {
        _VCOMPIMW { w: self }
    }
    #[doc = "Bit 4 - AHB master error interrupt enable."]
    #[inline(always)]
    pub fn berim(&mut self) -> _BERIMW {
        _BERIMW { w: self }
    }
}
