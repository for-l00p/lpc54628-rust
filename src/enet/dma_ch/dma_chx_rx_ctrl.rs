#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CHX_RX_CTRL {
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
pub type SR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
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
pub type RBSZ_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _RBSZW<'a> {
    w: &'a mut W,
}
impl<'a> _RBSZW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 3)) | (((value as u32) & 0x0fff) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RXPBL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RXPBLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPBLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RPF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RPFW<'a> {
    w: &'a mut W,
}
impl<'a> _RPFW<'a> {
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
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 3:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits() >> 3) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn rx_pbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Start or Stop Receive When this bit is set, the DMA tries to acquire the from the receive list and processes the incoming packets."]
    #[inline(always)]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
    #[doc = "Bits 3:14 - Receive Buffer size This field indicates the size of the Rx buffers specified in bytes."]
    #[inline(always)]
    pub fn rbsz(&mut self) -> _RBSZW {
        _RBSZW { w: self }
    }
    #[doc = "Bits 16:21 - Receive Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn rx_pbl(&mut self) -> _RXPBLW {
        _RXPBLW { w: self }
    }
    #[doc = "Bit 31 - DMA Rx Channel 0 Packet Flush When this bit is set to 1, the DMA will automatically flush the packet from the Rx Queues destined to DMA Rx Channel 0 when the DMA Rx Channel 0 is stopped after a system bus error has occurred."]
    #[inline(always)]
    pub fn rpf(&mut self) -> _RPFW {
        _RPFW { w: self }
    }
}
