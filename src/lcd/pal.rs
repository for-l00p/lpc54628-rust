#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PAL {
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
pub type R04_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _R04_0W<'a> {
    w: &'a mut W,
}
impl<'a> _R04_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type G04_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _G04_0W<'a> {
    w: &'a mut W,
}
impl<'a> _G04_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type B04_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _B04_0W<'a> {
    w: &'a mut W,
}
impl<'a> _B04_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type I0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _I0W<'a> {
    w: &'a mut W,
}
impl<'a> _I0W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type R14_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _R14_0W<'a> {
    w: &'a mut W,
}
impl<'a> _R14_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type G14_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _G14_0W<'a> {
    w: &'a mut W,
}
impl<'a> _G14_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type B14_0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _B14_0W<'a> {
    w: &'a mut W,
}
impl<'a> _B14_0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type I1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _I1W<'a> {
    w: &'a mut W,
}
impl<'a> _I1W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&self) -> R04_0_R {
        R04_0_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&self) -> G04_0_R {
        G04_0_R::new(((self.bits() >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&self) -> B04_0_R {
        B04_0_R::new(((self.bits() >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&self) -> I0_R {
        I0_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&self) -> R14_0_R {
        R14_0_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&self) -> G14_0_R {
        G14_0_R::new(((self.bits() >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&self) -> B14_0_R {
        B14_0_R::new(((self.bits() >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&self) -> I1_R {
        I1_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Red palette data."]
    #[inline(always)]
    pub fn r04_0(&mut self) -> _R04_0W {
        _R04_0W { w: self }
    }
    #[doc = "Bits 5:9 - Green palette data."]
    #[inline(always)]
    pub fn g04_0(&mut self) -> _G04_0W {
        _G04_0W { w: self }
    }
    #[doc = "Bits 10:14 - Blue palette data."]
    #[inline(always)]
    pub fn b04_0(&mut self) -> _B04_0W {
        _B04_0W { w: self }
    }
    #[doc = "Bit 15 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i0(&mut self) -> _I0W {
        _I0W { w: self }
    }
    #[doc = "Bits 16:20 - Red palette data."]
    #[inline(always)]
    pub fn r14_0(&mut self) -> _R14_0W {
        _R14_0W { w: self }
    }
    #[doc = "Bits 21:25 - Green palette data."]
    #[inline(always)]
    pub fn g14_0(&mut self) -> _G14_0W {
        _G14_0W { w: self }
    }
    #[doc = "Bits 26:30 - Blue palette data."]
    #[inline(always)]
    pub fn b14_0(&mut self) -> _B14_0W {
        _B14_0W { w: self }
    }
    #[doc = "Bit 31 - Intensity / unused bit."]
    #[inline(always)]
    pub fn i1(&mut self) -> _I1W {
        _I1W { w: self }
    }
}
