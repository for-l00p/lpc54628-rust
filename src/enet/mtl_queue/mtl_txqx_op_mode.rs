#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_TXQX_OP_MODE {
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
pub type FTQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FTQW<'a> {
    w: &'a mut W,
}
impl<'a> _FTQW<'a> {
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
pub type TSF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSFW<'a> {
    w: &'a mut W,
}
impl<'a> _TSFW<'a> {
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
pub type TXQEN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TXQENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXQENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TTC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TTCW<'a> {
    w: &'a mut W,
}
impl<'a> _TTCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TQS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TQSW<'a> {
    w: &'a mut W,
}
impl<'a> _TQSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Flush Transmit Queue When this bit is set, the Tx queue controller logic is reset to its default values."]
    #[inline(always)]
    pub fn ftq(&mut self) -> _FTQW {
        _FTQW { w: self }
    }
    #[doc = "Bit 1 - Transmit Store and Forward When this bit is set, the transmission starts when a full packet resides in the MTL Tx queue."]
    #[inline(always)]
    pub fn tsf(&mut self) -> _TSFW {
        _TSFW { w: self }
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable This field is used to enable/disable the transmit queue 0."]
    #[inline(always)]
    pub fn txqen(&mut self) -> _TXQENW {
        _TXQENW { w: self }
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control These bits control the threshold level of the MTL Tx Queue."]
    #[inline(always)]
    pub fn ttc(&mut self) -> _TTCW {
        _TTCW { w: self }
    }
    #[doc = "Bits 16:18 - Transmit Queue Size This field indicates the size of the allocated Transmit queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn tqs(&mut self) -> _TQSW {
        _TQSW { w: self }
    }
}
