#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CHX_STAT {
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
pub type TI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TIW<'a> {
    w: &'a mut W,
}
impl<'a> _TIW<'a> {
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
pub type TPS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TPSW<'a> {
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
pub type TBU_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TBUW<'a> {
    w: &'a mut W,
}
impl<'a> _TBUW<'a> {
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
pub type RI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RIW<'a> {
    w: &'a mut W,
}
impl<'a> _RIW<'a> {
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
pub type RBU_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RBUW<'a> {
    w: &'a mut W,
}
impl<'a> _RBUW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RPS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RPSW<'a> {
    w: &'a mut W,
}
impl<'a> _RPSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RWT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWTW<'a> {
    w: &'a mut W,
}
impl<'a> _RWTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ETI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETIW<'a> {
    w: &'a mut W,
}
impl<'a> _ETIW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ERI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ERIW<'a> {
    w: &'a mut W,
}
impl<'a> _ERIW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FBE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FBEW<'a> {
    w: &'a mut W,
}
impl<'a> _FBEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AISW<'a> {
    w: &'a mut W,
}
impl<'a> _AISW<'a> {
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
pub type NIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NISW<'a> {
    w: &'a mut W,
}
impl<'a> _NISW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EB_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EBW<'a> {
    w: &'a mut W,
}
impl<'a> _EBW<'a> {
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
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the packet transmission is complete."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the transmit list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the packet reception is complete."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the application owns the next in the receive list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog time out This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet."]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Ear1y Transmit Interrupt Bit 12: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA Channel Interrupt Enable register Table 778) affect the Normal Interrupt Summary bit."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the packet transmission is complete."]
    #[inline(always)]
    pub fn ti(&mut self) -> _TIW {
        _TIW { w: self }
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&mut self) -> _TPSW {
        _TPSW { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the application owns the next descriptor in the transmit list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn tbu(&mut self) -> _TBUW {
        _TBUW { w: self }
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the packet reception is complete."]
    #[inline(always)]
    pub fn ri(&mut self) -> _RIW {
        _RIW { w: self }
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the application owns the next in the receive list, and the DMA cannot acquire it."]
    #[inline(always)]
    pub fn rbu(&mut self) -> _RBUW {
        _RBUW { w: self }
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Rx process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&mut self) -> _RPSW {
        _RPSW { w: self }
    }
    #[doc = "Bit 9 - Receive Watchdog time out This bit is asserted when a packet with length greater than 2,048 bytes (10,240 bytes when Jumbo Packet mode is enabled) is received."]
    #[inline(always)]
    pub fn rwt(&mut self) -> _RWTW {
        _RWTW { w: self }
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the packet to be transmitted is fully transferred to the MTL Tx FIFO."]
    #[inline(always)]
    pub fn eti(&mut self) -> _ETIW {
        _ETIW { w: self }
    }
    #[doc = "Bit 11 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet."]
    #[inline(always)]
    pub fn eri(&mut self) -> _ERIW {
        _ERIW { w: self }
    }
    #[doc = "Bit 12 - Fatal Bus Error This bit indicates that a bus error occurred (as described in the EB field)."]
    #[inline(always)]
    pub fn fbe(&mut self) -> _FBEW {
        _FBEW { w: self }
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 1: Transmit Process Stopped Bit 7: Receive Buffer Unavailable Bit 8: Receive Process Stopped Bit 10: Ear1y Transmit Interrupt Bit 12: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit."]
    #[inline(always)]
    pub fn ais(&mut self) -> _AISW {
        _AISW { w: self }
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in the DMA Channel Interrupt Enable register Table 778: Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in DMA Channel Interrupt Enable register Table 778) affect the Normal Interrupt Summary bit."]
    #[inline(always)]
    pub fn nis(&mut self) -> _NISW {
        _NISW { w: self }
    }
    #[doc = "Bits 16:18 - DMA Error Bits This field indicates the type of error that caused a Bus Error."]
    #[inline(always)]
    pub fn eb(&mut self) -> _EBW {
        _EBW { w: self }
    }
}
