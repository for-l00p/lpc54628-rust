#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMV {
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
pub type LPP_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _LPPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type VSW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _VSWW<'a> {
    w: &'a mut W,
}
impl<'a> _VSWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type VFP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _VFPW<'a> {
    w: &'a mut W,
}
impl<'a> _VFPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type VBP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _VBPW<'a> {
    w: &'a mut W,
}
impl<'a> _VBPW<'a> {
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
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&self) -> LPP_R {
        LPP_R::new((self.bits() & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&self) -> VSW_R {
        VSW_R::new(((self.bits() >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&mut self) -> _LPPW {
        _LPPW { w: self }
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&mut self) -> _VSWW {
        _VSWW { w: self }
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&mut self) -> _VFPW {
        _VFPW { w: self }
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&mut self) -> _VBPW {
        _VBPW { w: self }
    }
}
