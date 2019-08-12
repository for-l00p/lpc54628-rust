#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MINTSTS {
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
pub type CDET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CDETW<'a> {
    w: &'a mut W,
}
impl<'a> _CDETW<'a> {
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
pub type RE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
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
pub type CDONE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _CDONEW<'a> {
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
pub type DTO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOW<'a> {
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
pub type TXDR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXDRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDRW<'a> {
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
pub type RXDR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXDRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDRW<'a> {
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
pub type RCRC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RCRCW<'a> {
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
pub type DCRC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCRCW<'a> {
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
pub type RTO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RTOW<'a> {
    w: &'a mut W,
}
impl<'a> _RTOW<'a> {
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
pub type DRTO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DRTOW<'a> {
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
pub type HTO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HTOW<'a> {
    w: &'a mut W,
}
impl<'a> _HTOW<'a> {
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
pub type FRUN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _FRUNW<'a> {
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
pub type HLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HLEW<'a> {
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
pub type SBE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SBEW<'a> {
    w: &'a mut W,
}
impl<'a> _SBEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ACD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ACDW<'a> {
    w: &'a mut W,
}
impl<'a> _ACDW<'a> {
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
pub type EBE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EBEW<'a> {
    w: &'a mut W,
}
impl<'a> _EBEW<'a> {
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
pub type SDIO_INTERRUPT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SDIO_INTERRUPTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_INTERRUPTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&self) -> CDET_R {
        CDET_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&self) -> CDONE_R {
        CDONE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&self) -> DTO_R {
        DTO_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&self) -> TXDR_R {
        TXDR_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&self) -> DCRC_R {
        DCRC_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&self) -> DRTO_R {
        DRTO_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&self) -> HTO_R {
        HTO_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&self) -> FRUN_R {
        FRUN_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&self) -> HLE_R {
        HLE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&self) -> ACD_R {
        ACD_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub fn ebe(&self) -> EBE_R {
        EBE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt from SDIO card."]
    #[inline(always)]
    pub fn sdio_interrupt(&self) -> SDIO_INTERRUPT_R {
        SDIO_INTERRUPT_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&mut self) -> _CDETW {
        _CDETW { w: self }
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&mut self) -> _CDONEW {
        _CDONEW { w: self }
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&mut self) -> _DTOW {
        _DTOW { w: self }
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&mut self) -> _TXDRW {
        _TXDRW { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&mut self) -> _RXDRW {
        _RXDRW { w: self }
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&mut self) -> _RCRCW {
        _RCRCW { w: self }
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&mut self) -> _DCRCW {
        _DCRCW { w: self }
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&mut self) -> _RTOW {
        _RTOW { w: self }
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&mut self) -> _DRTOW {
        _DRTOW { w: self }
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&mut self) -> _HTOW {
        _HTOW { w: self }
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&mut self) -> _FRUNW {
        _FRUNW { w: self }
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&mut self) -> _HLEW {
        _HLEW { w: self }
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&mut self) -> _SBEW {
        _SBEW { w: self }
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&mut self) -> _ACDW {
        _ACDW { w: self }
    }
    #[doc = "Bit 15 - End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub fn ebe(&mut self) -> _EBEW {
        _EBEW { w: self }
    }
    #[doc = "Bit 16 - Interrupt from SDIO card."]
    #[inline(always)]
    pub fn sdio_interrupt(&mut self) -> _SDIO_INTERRUPTW {
        _SDIO_INTERRUPTW { w: self }
    }
}
