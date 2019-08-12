#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUTOCGOR {
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
pub type RAM0X_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RAM0XW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0XW<'a> {
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
pub type RAM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1W<'a> {
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
pub type RAM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RAM2W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM2W<'a> {
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
pub type RAM3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RAM3W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM3W<'a> {
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
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&self) -> RAM0X_R {
        RAM0X_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&mut self) -> _RAM0XW {
        _RAM0XW { w: self }
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&mut self) -> _RAM1W {
        _RAM1W { w: self }
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&mut self) -> _RAM2W {
        _RAM2W { w: self }
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&mut self) -> _RAM3W {
        _RAM3W { w: self }
    }
}
