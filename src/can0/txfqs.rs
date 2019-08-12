#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXFQS {
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
pub type TFGI_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TFGIW<'a> {
    w: &'a mut W,
}
impl<'a> _TFGIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TFQPI_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TFQPIW<'a> {
    w: &'a mut W,
}
impl<'a> _TFQPIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TFQF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TFQFW<'a> {
    w: &'a mut W,
}
impl<'a> _TFQFW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits() >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:12 - Tx FIFO get index."]
    #[inline(always)]
    pub fn tfgi(&mut self) -> _TFGIW {
        _TFGIW { w: self }
    }
    #[doc = "Bits 16:20 - Tx FIFO/queue put index."]
    #[inline(always)]
    pub fn tfqpi(&mut self) -> _TFQPIW {
        _TFQPIW { w: self }
    }
    #[doc = "Bit 21 - Tx FIFO/queue full."]
    #[inline(always)]
    pub fn tfqf(&mut self) -> _TFQFW {
        _TFQFW { w: self }
    }
}
