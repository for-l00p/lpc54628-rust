#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUDPLLFRAC {
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
pub type CTRL_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type REQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _REQW<'a> {
    w: &'a mut W,
}
impl<'a> _REQW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SEL_EXT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEL_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_EXTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits() & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&self) -> SEL_EXT_R {
        SEL_EXT_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:21 - PLL fractional divider control word"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> _CTRLW {
        _CTRLW { w: self }
    }
    #[doc = "Bit 22 - Writing 1 to REQ signal loads CTRL value into fractional wrapper modulator."]
    #[inline(always)]
    pub fn req(&mut self) -> _REQW {
        _REQW { w: self }
    }
    #[doc = "Bit 23 - Select fractional divider."]
    #[inline(always)]
    pub fn sel_ext(&mut self) -> _SEL_EXTW {
        _SEL_EXTW { w: self }
    }
}
