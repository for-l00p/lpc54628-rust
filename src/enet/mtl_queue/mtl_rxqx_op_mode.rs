#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_RXQX_OP_MODE {
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
pub type RTC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FUP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FUPW<'a> {
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
pub type FEP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FEPW<'a> {
    w: &'a mut W,
}
impl<'a> _FEPW<'a> {
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
#[doc = r"Reader of the field"]
pub type RSF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RSFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSFW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DIS_TCP_EF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DIS_TCP_EFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_TCP_EFW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RQS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RQSW<'a> {
    w: &'a mut W,
}
impl<'a> _RQSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Receive Queue Threshold Control These bits control the threshold level of the MTL Rx queue (in bytes): 00: 64 01: 32 10: 96 11: 128 The packet received is transferred to the application or DMA when the packet size within the MTL Rx queue is larger than the threshold."]
    #[inline(always)]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 3 - Forward Undersized Good Packets When this bit is set, the Rx queue forwards the undersized good packets (packets with no error and length less than 64 bytes), including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fup(&mut self) -> _FUPW {
        _FUPW { w: self }
    }
    #[doc = "Bit 4 - Forward Error Packets When this bit is reset, the Rx queue drops packets with error status (CRC error, Mll_ER, watchdog timeout, or overflow)."]
    #[inline(always)]
    pub fn fep(&mut self) -> _FEPW {
        _FEPW { w: self }
    }
    #[doc = "Bit 5 - Receive Queue Store and Forward When this bit is set, the ethernet block on this chip reads a packet from the Rx queue only after the complete packet has been written to it, ignoring the RTC field of this register."]
    #[inline(always)]
    pub fn rsf(&mut self) -> _RSFW {
        _RSFW { w: self }
    }
    #[doc = "Bit 6 - Disable Dropping of TCP/IP Checksum Error Packets When this bit is set, the MAC does not drop the packets which only have the errors detected by the Receive Checksum Offload engine."]
    #[inline(always)]
    pub fn dis_tcp_ef(&mut self) -> _DIS_TCP_EFW {
        _DIS_TCP_EFW { w: self }
    }
    #[doc = "Bits 20:22 - This field indicates the size of the allocated Receive queues in blocks of 256 bytes."]
    #[inline(always)]
    pub fn rqs(&mut self) -> _RQSW {
        _RQSW { w: self }
    }
}
