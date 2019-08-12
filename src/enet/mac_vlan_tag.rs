#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_VLAN_TAG {
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
pub type VL_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _VLW<'a> {
    w: &'a mut W,
}
impl<'a> _VLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ETV_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETVW<'a> {
    w: &'a mut W,
}
impl<'a> _ETVW<'a> {
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
pub type VTIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _VTIMW<'a> {
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
pub type ESVL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ESVLW<'a> {
    w: &'a mut W,
}
impl<'a> _ESVLW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ERSVLM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ERSVLMW<'a> {
    w: &'a mut W,
}
impl<'a> _ERSVLMW<'a> {
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
pub type DOVLTC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DOVLTCW<'a> {
    w: &'a mut W,
}
impl<'a> _DOVLTCW<'a> {
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
pub type EVLS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EVLSW<'a> {
    w: &'a mut W,
}
impl<'a> _EVLSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EVLRXS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EVLRXSW<'a> {
    w: &'a mut W,
}
impl<'a> _EVLRXSW<'a> {
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
#[doc = r"Reader of the field"]
pub type VTHM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VTHMW<'a> {
    w: &'a mut W,
}
impl<'a> _VTHMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EDVLP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EDVLPW<'a> {
    w: &'a mut W,
}
impl<'a> _EDVLPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ERIVLT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ERIVLTW<'a> {
    w: &'a mut W,
}
impl<'a> _ERIVLTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EIVLS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EIVLSW<'a> {
    w: &'a mut W,
}
impl<'a> _EIVLSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EIVLRXS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EIVLRXSW<'a> {
    w: &'a mut W,
}
impl<'a> _EIVLRXSW<'a> {
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
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets."]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits() & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison."]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable."]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable S-VLAN."]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match."]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Disable VLAN Type Check."]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive."]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits() >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status."]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Disable VLAN Type Check."]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing."]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag."]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive."]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status."]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Packets."]
    #[inline(always)]
    pub fn vl(&mut self) -> _VLW {
        _VLW { w: self }
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison."]
    #[inline(always)]
    pub fn etv(&mut self) -> _ETVW {
        _ETVW { w: self }
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable."]
    #[inline(always)]
    pub fn vtim(&mut self) -> _VTIMW {
        _VTIMW { w: self }
    }
    #[doc = "Bit 18 - Enable S-VLAN."]
    #[inline(always)]
    pub fn esvl(&mut self) -> _ESVLW {
        _ESVLW { w: self }
    }
    #[doc = "Bit 19 - Enable Receive S-VLAN Match."]
    #[inline(always)]
    pub fn ersvlm(&mut self) -> _ERSVLMW {
        _ERSVLMW { w: self }
    }
    #[doc = "Bit 20 - Disable VLAN Type Check."]
    #[inline(always)]
    pub fn dovltc(&mut self) -> _DOVLTCW {
        _DOVLTCW { w: self }
    }
    #[doc = "Bits 21:22 - Enable VLAN Tag Stripping on Receive."]
    #[inline(always)]
    pub fn evls(&mut self) -> _EVLSW {
        _EVLSW { w: self }
    }
    #[doc = "Bit 24 - Enable VLAN Tag in Rx status."]
    #[inline(always)]
    pub fn evlrxs(&mut self) -> _EVLRXSW {
        _EVLRXSW { w: self }
    }
    #[doc = "Bit 25 - Disable VLAN Type Check."]
    #[inline(always)]
    pub fn vthm(&mut self) -> _VTHMW {
        _VTHMW { w: self }
    }
    #[doc = "Bit 26 - Enable Double VLAN Processing."]
    #[inline(always)]
    pub fn edvlp(&mut self) -> _EDVLPW {
        _EDVLPW { w: self }
    }
    #[doc = "Bit 27 - Enable Inner VLAN Tag."]
    #[inline(always)]
    pub fn erivlt(&mut self) -> _ERIVLTW {
        _ERIVLTW { w: self }
    }
    #[doc = "Bits 28:29 - Enable Inner VLAN Tag Stripping on Receive."]
    #[inline(always)]
    pub fn eivls(&mut self) -> _EIVLSW {
        _EIVLSW { w: self }
    }
    #[doc = "Bit 31 - Enable Inner VLAN Tag in Rx Status."]
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> _EIVLRXSW {
        _EIVLRXSW { w: self }
    }
}
