#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMH {
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
pub type PPL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PPLW<'a> {
    w: &'a mut W,
}
impl<'a> _PPLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HSW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HSWW<'a> {
    w: &'a mut W,
}
impl<'a> _HSWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HFP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HFPW<'a> {
    w: &'a mut W,
}
impl<'a> _HFPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HBP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HBPW<'a> {
    w: &'a mut W,
}
impl<'a> _HBPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&self) -> PPL_R {
        PPL_R::new(((self.bits() >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&self) -> HFP_R {
        HFP_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&mut self) -> _PPLW {
        _PPLW { w: self }
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&mut self) -> _HSWW {
        _HSWW { w: self }
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&mut self) -> _HFPW {
        _HFPW { w: self }
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&mut self) -> _HBPW {
        _HBPW { w: self }
    }
}
