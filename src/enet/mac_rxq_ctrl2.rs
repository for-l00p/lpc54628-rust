#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_RXQ_CTRL2 {
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
pub type PSRQ0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSRQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _PSRQ0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PSRQ1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSRQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _PSRQ1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PSRQ2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSRQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _PSRQ2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PSRQ3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSRQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _PSRQ3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priorities Selected in the Receive Queue 2."]
    #[inline(always)]
    pub fn psrq2(&self) -> PSRQ2_R {
        PSRQ2_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priorities Selected in the Receive Queue 3."]
    #[inline(always)]
    pub fn psrq3(&self) -> PSRQ3_R {
        PSRQ3_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub fn psrq0(&mut self) -> _PSRQ0W {
        _PSRQ0W { w: self }
    }
    #[doc = "Bits 8:15 - Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub fn psrq1(&mut self) -> _PSRQ1W {
        _PSRQ1W { w: self }
    }
    #[doc = "Bits 16:23 - Priorities Selected in the Receive Queue 2."]
    #[inline(always)]
    pub fn psrq2(&mut self) -> _PSRQ2W {
        _PSRQ2W { w: self }
    }
    #[doc = "Bits 24:31 - Priorities Selected in the Receive Queue 3."]
    #[inline(always)]
    pub fn psrq3(&mut self) -> _PSRQ3W {
        _PSRQ3W { w: self }
    }
}
