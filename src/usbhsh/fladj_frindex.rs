#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLADJ_FRINDEX {
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
        0x20
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type FLADJ_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FLADJW<'a> {
    w: &'a mut W,
}
impl<'a> _FLADJW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FRINDEX_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _FRINDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _FRINDEXW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&self) -> FLADJ_R {
        FLADJ_R::new((self.bits() & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new(((self.bits() >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&mut self) -> _FLADJW {
        _FLADJW { w: self }
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&mut self) -> _FRINDEXW {
        _FRINDEXW { w: self }
    }
}
