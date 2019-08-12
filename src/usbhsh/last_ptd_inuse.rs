#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LAST_PTD_INUSE {
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
pub type ATL_LAST_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ATL_LASTW<'a> {
    w: &'a mut W,
}
impl<'a> _ATL_LASTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ISO_LAST_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ISO_LASTW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_LASTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type INT_LAST_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INT_LASTW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_LASTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&self) -> ATL_LAST_R {
        ATL_LAST_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&self) -> ISO_LAST_R {
        ISO_LAST_R::new(((self.bits() >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&self) -> INT_LAST_R {
        INT_LAST_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&mut self) -> _ATL_LASTW {
        _ATL_LASTW { w: self }
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&mut self) -> _ISO_LASTW {
        _ISO_LASTW { w: self }
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&mut self) -> _INT_LASTW {
        _INT_LASTW { w: self }
    }
}
