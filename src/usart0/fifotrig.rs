#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOTRIG {
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
#[doc = "Possible values of the field `TXLVLENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLENAR {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED,
}
impl crate::ToBits<bool> for TXLVLENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TXLVLENAR::DISABLED => false,
            TXLVLENAR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXLVLENA_R = crate::FR<bool, TXLVLENAR>;
impl TXLVLENA_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVLENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVLENAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TXLVLENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLENAW {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED,
}
impl TXLVLENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXLVLENAW::DISABLED => false,
            TXLVLENAW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXLVLENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLVLENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLVLENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLENAW::DISABLED)
    }
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLENAW::ENABLED)
    }
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
#[doc = "Possible values of the field `RXLVLENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLENAR {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED,
}
impl crate::ToBits<bool> for RXLVLENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RXLVLENAR::DISABLED => false,
            RXLVLENAR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXLVLENA_R = crate::FR<bool, RXLVLENAR>;
impl RXLVLENA_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVLENAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVLENAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RXLVLENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLENAW {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED,
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED,
}
impl RXLVLENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RXLVLENAW::DISABLED => false,
            RXLVLENAW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXLVLENAW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLVLENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLVLENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLENAW::DISABLED)
    }
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLENAW::ENABLED)
    }
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
pub type TXLVL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLVLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RXLVL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLVLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn txlvlena(&self) -> TXLVLENA_R {
        TXLVLENA_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn rxlvlena(&self) -> RXLVLENA_R {
        RXLVLENA_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn txlvlena(&mut self) -> _TXLVLENAW {
        _TXLVLENAW { w: self }
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn rxlvlena(&mut self) -> _RXLVLENAW {
        _RXLVLENAW { w: self }
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> _TXLVLW {
        _TXLVLW { w: self }
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> _RXLVLW {
        _RXLVLW { w: self }
    }
}
