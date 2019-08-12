#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NBTP {
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
        0x0600_0a03
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type NTSEG2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NTSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _NTSEG2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NTSEG1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NTSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _NTSEG1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NBRP_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _NBRPW<'a> {
    w: &'a mut W,
}
impl<'a> _NBRPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NSJW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NSJWW<'a> {
    w: &'a mut W,
}
impl<'a> _NSJWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Nominal time segment after sample point."]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal time segment before sample point."]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal bit rate prescaler."]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits() >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (re)synchronization jump width."]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits() >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Nominal time segment after sample point."]
    #[inline(always)]
    pub fn ntseg2(&mut self) -> _NTSEG2W {
        _NTSEG2W { w: self }
    }
    #[doc = "Bits 8:15 - Nominal time segment before sample point."]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> _NTSEG1W {
        _NTSEG1W { w: self }
    }
    #[doc = "Bits 16:24 - Nominal bit rate prescaler."]
    #[inline(always)]
    pub fn nbrp(&mut self) -> _NBRPW {
        _NBRPW { w: self }
    }
    #[doc = "Bits 25:31 - Nominal (re)synchronization jump width."]
    #[inline(always)]
    pub fn nsjw(&mut self) -> _NSJWW {
        _NSJWW { w: self }
    }
}
