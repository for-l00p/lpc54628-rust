#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ASYNCAPBCLKCTRL {
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
pub type CTIMER3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CTIMER4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 13 - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 13 - Controls the clock for CTIMER3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> _CTIMER3W {
        _CTIMER3W { w: self }
    }
    #[doc = "Bit 14 - Controls the clock for CTIMER4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> _CTIMER4W {
        _CTIMER4W { w: self }
    }
}
