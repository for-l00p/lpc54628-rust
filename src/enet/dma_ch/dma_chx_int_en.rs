#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CHX_INT_EN {
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
pub type TIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
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
pub type TSE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSEW<'a> {
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
pub type TBUE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TBUEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBUEW<'a> {
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
pub type RIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
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
pub type RBUE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RBUEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBUEW<'a> {
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
pub type RSE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RSEW<'a> {
    w: &'a mut W,
}
impl<'a> _RSEW<'a> {
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
pub type RWTE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWTEW<'a> {
    w: &'a mut W,
}
impl<'a> _RWTEW<'a> {
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
pub type ETIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ETIEW<'a> {
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
pub type ERIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ERIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERIEW<'a> {
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
pub type FBEE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FBEEW<'a> {
    w: &'a mut W,
}
impl<'a> _FBEEW<'a> {
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
pub type AIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AIEW<'a> {
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
pub type NIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NIEW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled."]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled."]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt summary is enabled."]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled."]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 1 - Transmit stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Transmission Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn tse(&mut self) -> _TSEW {
        _TSEW { w: self }
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Transmit Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn tbue(&mut self) -> _TBUEW {
        _TBUEW { w: self }
    }
    #[doc = "Bit 6 - Receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Buffer Unavailable Interrupt is enabled."]
    #[inline(always)]
    pub fn rbue(&mut self) -> _RBUEW {
        _RBUEW { w: self }
    }
    #[doc = "Bit 8 - Received stopped enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), Receive Stopped Interrupt is enabled."]
    #[inline(always)]
    pub fn rse(&mut self) -> _RSEW {
        _RSEW { w: self }
    }
    #[doc = "Bit 9 - Receive watchdog timeout enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Receive Watchdog Timeout Interrupt is enabled."]
    #[inline(always)]
    pub fn rwte(&mut self) -> _RWTEW {
        _RWTEW { w: self }
    }
    #[doc = "Bit 10 - Early transmit interrupt enable When this bit is set with an Abnormal Interrupt Summary Enable (bit 15 in this register), Early Transmit Interrupt is enabled."]
    #[inline(always)]
    pub fn etie(&mut self) -> _ETIEW {
        _ETIEW { w: self }
    }
    #[doc = "Bit 11 - Early receive interrupt enable When this bit is set with Normal Interrupt Summary Enable (bit 16 in this register), Early Receive Interrupt is enabled."]
    #[inline(always)]
    pub fn erie(&mut self) -> _ERIEW {
        _ERIEW { w: self }
    }
    #[doc = "Bit 12 - Fatal bus error enable When this bit is set with Abnormal Interrupt Summary Enable (bit 15 in this register), the Fatal Bus Error Interrupt is enabled."]
    #[inline(always)]
    pub fn fbee(&mut self) -> _FBEEW {
        _FBEEW { w: self }
    }
    #[doc = "Bit 14 - Abnormal interrupt summary enable When this bit is set, an Abnormal Interrupt summary is enabled."]
    #[inline(always)]
    pub fn aie(&mut self) -> _AIEW {
        _AIEW { w: self }
    }
    #[doc = "Bit 15 - Normal interrupt summary enable When this bit is set, a normal interrupt is enabled."]
    #[inline(always)]
    pub fn nie(&mut self) -> _NIEW {
        _NIEW { w: self }
    }
}
