#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCICTRL {
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
pub type SCIEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCIENW<'a> {
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
pub type NACKDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NACKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKDISW<'a> {
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
pub type PROTSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PROTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTSELW<'a> {
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
pub type TXRETRY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TXRETRYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRETRYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type GUARDTIME_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _GUARDTIMEW<'a> {
    w: &'a mut W,
}
impl<'a> _GUARDTIMEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits() >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&self) -> GUARDTIME_R {
        GUARDTIME_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> _SCIENW {
        _SCIENW { w: self }
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> _NACKDISW {
        _NACKDISW { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> _PROTSELW {
        _PROTSELW { w: self }
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&mut self) -> _TXRETRYW {
        _TXRETRYW { w: self }
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&mut self) -> _GUARDTIMEW {
        _GUARDTIMEW { w: self }
    }
}
