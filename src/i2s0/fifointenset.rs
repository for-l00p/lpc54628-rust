#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOINTENSET {
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
#[doc = "Possible values of the field `TXERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERRR {
    #[doc = "No interrupt will be generated for a transmit error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    ENABLED,
}
impl crate::ToBits<bool> for TXERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TXERRR::DISABLED => false,
            TXERRR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXERR_R = crate::FR<bool, TXERRR>;
impl TXERR_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXERRR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TXERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXERRW {
    #[doc = "No interrupt will be generated for a transmit error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    ENABLED,
}
impl TXERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXERRW::DISABLED => false,
            TXERRW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXERRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated for a transmit error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXERRW::DISABLED)
    }
    #[doc = "An interrupt will be generated when a transmit error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXERRW::ENABLED)
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
#[doc = "Possible values of the field `RXERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERRR {
    #[doc = "No interrupt will be generated for a receive error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a receive error occurs."]
    ENABLED,
}
impl crate::ToBits<bool> for RXERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RXERRR::DISABLED => false,
            RXERRR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXERR_R = crate::FR<bool, RXERRR>;
impl RXERR_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXERRR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RXERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERRW {
    #[doc = "No interrupt will be generated for a receive error."]
    DISABLED,
    #[doc = "An interrupt will be generated when a receive error occurs."]
    ENABLED,
}
impl RXERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RXERRW::DISABLED => false,
            RXERRW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated for a receive error."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXERRW::DISABLED)
    }
    #[doc = "An interrupt will be generated when a receive error occurs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXERRW::ENABLED)
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
#[doc = "Possible values of the field `TXLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLR {
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    DISABLED,
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl crate::ToBits<bool> for TXLVLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TXLVLR::DISABLED => false,
            TXLVLR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXLVL_R = crate::FR<bool, TXLVLR>;
impl TXLVL_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVLR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TXLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLVLW {
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    DISABLED,
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl TXLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXLVLW::DISABLED => false,
            TXLVLW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLVLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLVLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated based on the TX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLW::DISABLED)
    }
    #[doc = "If TXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the TX FIFO level decreases to the level specified by TXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `RXLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLR {
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    DISABLED,
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl crate::ToBits<bool> for RXLVLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RXLVLR::DISABLED => false,
            RXLVLR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXLVL_R = crate::FR<bool, RXLVLR>;
impl RXLVL_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVLR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RXLVL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLVLW {
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    DISABLED,
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    ENABLED,
}
impl RXLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RXLVLW::DISABLED => false,
            RXLVLW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLVLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLVLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated based on the RX FIFO level."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLW::DISABLED)
    }
    #[doc = "If RXLVLENA in the FIFOTRIG register = 1, an interrupt will be generated when the when the RX FIFO level increases to the level specified by RXLVL in the FIFOTRIG register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn txerr(&mut self) -> _TXERRW {
        _TXERRW { w: self }
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn rxerr(&mut self) -> _RXERRW {
        _RXERRW { w: self }
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> _TXLVLW {
        _TXLVLW { w: self }
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> _RXLVLW {
        _RXLVLW { w: self }
    }
}
