#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXBC {
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
pub type TBSA_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TBSAW<'a> {
    w: &'a mut W,
}
impl<'a> _TBSAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NDTB_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NDTBW<'a> {
    w: &'a mut W,
}
impl<'a> _NDTBW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TFQS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TFQSW<'a> {
    w: &'a mut W,
}
impl<'a> _TFQSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TFQM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TFQMW<'a> {
    w: &'a mut W,
}
impl<'a> _TFQMW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 2:15 - Tx buffers start address."]
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits() >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits() >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Tx FIFO/queue mode."]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:15 - Tx buffers start address."]
    #[inline(always)]
    pub fn tbsa(&mut self) -> _TBSAW {
        _TBSAW { w: self }
    }
    #[doc = "Bits 16:21 - Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[inline(always)]
    pub fn ndtb(&mut self) -> _NDTBW {
        _NDTBW { w: self }
    }
    #[doc = "Bits 24:29 - Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[inline(always)]
    pub fn tfqs(&mut self) -> _TFQSW {
        _TFQSW { w: self }
    }
    #[doc = "Bit 30 - Tx FIFO/queue mode."]
    #[inline(always)]
    pub fn tfqm(&mut self) -> _TFQMW {
        _TFQMW { w: self }
    }
}
