#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXESC {
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
pub type F0DS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F0DSW<'a> {
    w: &'a mut W,
}
impl<'a> _F0DSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F1DS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F1DSW<'a> {
    w: &'a mut W,
}
impl<'a> _F1DSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RBDS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RBDSW<'a> {
    w: &'a mut W,
}
impl<'a> _RBDSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Rx FIFO 0 data field size."]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 data field size."]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - ."]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Rx FIFO 0 data field size."]
    #[inline(always)]
    pub fn f0ds(&mut self) -> _F0DSW {
        _F0DSW { w: self }
    }
    #[doc = "Bits 4:6 - Rx FIFO 1 data field size."]
    #[inline(always)]
    pub fn f1ds(&mut self) -> _F1DSW {
        _F1DSW { w: self }
    }
    #[doc = "Bits 8:10 - ."]
    #[inline(always)]
    pub fn rbds(&mut self) -> _RBDSW {
        _RBDSW { w: self }
    }
}
