#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
pub type CAPEN0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPEN0W<'a> {
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
pub type CAPEN1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPEN1W<'a> {
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
pub type CAPEN2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPEN2W<'a> {
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
pub type CAPEN3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPEN3W<'a> {
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
pub type CAPPOL0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPPOL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPPOL0W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CAPPOL1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPPOL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPPOL1W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CAPPOL2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPPOL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPPOL2W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CAPPOL3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CAPPOL3W<'a> {
    w: &'a mut W,
}
impl<'a> _CAPPOL3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&self) -> CAPEN0_R {
        CAPEN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&self) -> CAPEN1_R {
        CAPEN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&self) -> CAPEN2_R {
        CAPEN2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&self) -> CAPEN3_R {
        CAPEN3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&self) -> CAPPOL0_R {
        CAPPOL0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&self) -> CAPPOL1_R {
        CAPPOL1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&self) -> CAPPOL2_R {
        CAPPOL2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&self) -> CAPPOL3_R {
        CAPPOL3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&mut self) -> _CAPEN0W {
        _CAPEN0W { w: self }
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&mut self) -> _CAPEN1W {
        _CAPEN1W { w: self }
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&mut self) -> _CAPEN2W {
        _CAPEN2W { w: self }
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&mut self) -> _CAPEN3W {
        _CAPEN3W { w: self }
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&mut self) -> _CAPPOL0W {
        _CAPPOL0W { w: self }
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&mut self) -> _CAPPOL1W {
        _CAPPOL1W { w: self }
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&mut self) -> _CAPPOL2W {
        _CAPPOL2W { w: self }
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&mut self) -> _CAPPOL3W {
        _CAPPOL3W { w: self }
    }
}
