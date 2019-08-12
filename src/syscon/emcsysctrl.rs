#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMCSYSCTRL {
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
        0x01
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type EMCSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMCSCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCSCW<'a> {
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
pub type EMCRD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMCRDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCRDW<'a> {
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
pub type EMCBC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMCBCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCBCW<'a> {
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
pub type EMCFBCLKINSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMCFBCLKINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCFBCLKINSELW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&self) -> EMCSC_R {
        EMCSC_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&self) -> EMCRD_R {
        EMCRD_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&self) -> EMCBC_R {
        EMCBC_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&self) -> EMCFBCLKINSEL_R {
        EMCFBCLKINSEL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - EMC Shift Control."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> _EMCSCW {
        _EMCSCW { w: self }
    }
    #[doc = "Bit 1 - EMC Reset Disable."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> _EMCRDW {
        _EMCRDW { w: self }
    }
    #[doc = "Bit 2 - External Memory Controller burst control."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> _EMCBCW {
        _EMCBCW { w: self }
    }
    #[doc = "Bit 3 - External Memory Controller clock select."]
    #[inline(always)]
    pub fn emcfbclkinsel(&mut self) -> _EMCFBCLKINSELW {
        _EMCFBCLKINSELW { w: self }
    }
}
