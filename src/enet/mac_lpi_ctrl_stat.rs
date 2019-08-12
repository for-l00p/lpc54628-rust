#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_LPI_CTRL_STAT {
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
pub type TLPIEN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TLPIEX_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RLPIEN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RLPIEX_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TLPIST_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RLPIST_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LPIEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPIENW<'a> {
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
pub type PLS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PLSW<'a> {
    w: &'a mut W,
}
impl<'a> _PLSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPITXA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPITXAW<'a> {
    w: &'a mut W,
}
impl<'a> _LPITXAW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPIATE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPIATEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPIATEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPITCSE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPITCSEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPITCSEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired."]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state."]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface, exited the LPI state, and resumed the normal reception."]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    #[inline(always)]
    pub fn lpiate(&self) -> LPIATE_R {
        LPIATE_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    #[inline(always)]
    pub fn lpitcse(&self) -> LPITCSE_R {
        LPITCSE_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    #[inline(always)]
    pub fn lpien(&mut self) -> _LPIENW {
        _LPIENW { w: self }
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY."]
    #[inline(always)]
    pub fn pls(&mut self) -> _PLSW {
        _PLSW { w: self }
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    #[inline(always)]
    pub fn lpitxa(&mut self) -> _LPITXAW {
        _LPITXAW { w: self }
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    #[inline(always)]
    pub fn lpiate(&mut self) -> _LPIATEW {
        _LPIATEW { w: self }
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    #[inline(always)]
    pub fn lpitcse(&mut self) -> _LPITCSEW {
        _LPITCSEW { w: self }
    }
}
