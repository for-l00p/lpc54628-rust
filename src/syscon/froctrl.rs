#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FROCTRL {
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
        0x4000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type TRIM_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
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
#[doc = r"Reader of the field"]
pub type FREQTRIM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FREQTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQTRIMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USBCLKADJ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USBCLKADJW<'a> {
    w: &'a mut W,
}
impl<'a> _USBCLKADJW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USBMODCHG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USBMODCHGW<'a> {
    w: &'a mut W,
}
impl<'a> _USBMODCHGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HSPDCLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HSPDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _HSPDCLKW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WRTRIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WRTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _WRTRIMW<'a> {
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
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits() & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&self) -> FREQTRIM_R {
        FREQTRIM_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&self) -> HSPDCLK_R {
        HSPDCLK_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&self) -> WRTRIM_R {
        WRTRIM_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&mut self) -> _TRIMW {
        _TRIMW { w: self }
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&mut self) -> _FREQTRIMW {
        _FREQTRIMW { w: self }
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> _USBCLKADJW {
        _USBCLKADJW { w: self }
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&mut self) -> _USBMODCHGW {
        _USBMODCHGW { w: self }
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&mut self) -> _HSPDCLKW {
        _HSPDCLKW { w: self }
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> _WRTRIMW {
        _WRTRIMW { w: self }
    }
}
