#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_TXQX_INTCTRL_STAT {
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
pub type TXUNFIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXUNFISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUNFISW<'a> {
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
pub type ABPSIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ABPSISW<'a> {
    w: &'a mut W,
}
impl<'a> _ABPSISW<'a> {
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
pub type TXUIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXUIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXUIEW<'a> {
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
pub type ABPSIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ABPSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABPSIEW<'a> {
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
pub type RXOVFIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXOVFISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVFISW<'a> {
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
#[doc = r"Reader of the field"]
pub type RXOIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOIEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status This bit indicates that the Transmit Queue had an underflow while transmitting the packet."]
    #[inline(always)]
    pub fn txunfis(&mut self) -> _TXUNFISW {
        _TXUNFISW { w: self }
    }
    #[doc = "Bit 1 - Average Bits Per Slot Interrupt Status When set, this bit indicates that the MAC has updated the ABS value."]
    #[inline(always)]
    pub fn abpsis(&mut self) -> _ABPSISW {
        _ABPSISW { w: self }
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable When this bit is set, the Transmit Queue Underflow interrupt is enabled."]
    #[inline(always)]
    pub fn txuie(&mut self) -> _TXUIEW {
        _TXUIEW { w: self }
    }
    #[doc = "Bit 9 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts the interrupt when the average bits per slot status is updated."]
    #[inline(always)]
    pub fn abpsie(&mut self) -> _ABPSIEW {
        _ABPSIEW { w: self }
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status This bit indicates that the Receive Queue had an overflow while receiving the packet."]
    #[inline(always)]
    pub fn rxovfis(&mut self) -> _RXOVFISW {
        _RXOVFISW { w: self }
    }
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable When this bit is set, the Receive Queue Overflow interrupt is enabled."]
    #[inline(always)]
    pub fn rxoie(&mut self) -> _RXOIEW {
        _RXOIEW { w: self }
    }
}
