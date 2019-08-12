#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISO_PTD_BASE_ADDR {
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
pub type ISO_FIRST_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ISO_FIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_FIRSTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ISO_BASE_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _ISO_BASEW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_BASEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&self) -> ISO_FIRST_R {
        ISO_FIRST_R::new(((self.bits() >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&self) -> ISO_BASE_R {
        ISO_BASE_R::new(((self.bits() >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 5:9 - This indicates the first PTD that is used by the hardware when it is processing the ISO list."]
    #[inline(always)]
    pub fn iso_first(&mut self) -> _ISO_FIRSTW {
        _ISO_FIRSTW { w: self }
    }
    #[doc = "Bits 10:31 - Base address to be used by the hardware to find the start of the ISO list."]
    #[inline(always)]
    pub fn iso_base(&mut self) -> _ISO_BASEW {
        _ISO_BASEW { w: self }
    }
}
