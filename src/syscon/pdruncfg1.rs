#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDRUNCFG1 {
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
        0x14f8_1f40
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type PDEN_USB1_PHY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_USB1_PHYW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_USB1_PHYW<'a> {
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
pub type PDEN_USB1_PLL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_USB1_PLLW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_USB1_PLLW<'a> {
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
pub type PDEN_AUD_PLL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_AUD_PLLW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_AUD_PLLW<'a> {
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
pub type PDEN_SYSOSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_SYSOSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_SYSOSCW<'a> {
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
pub type PDEN_EEPROM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_EEPROMW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_EEPROMW<'a> {
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
pub type PDEN_RNG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_RNGW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_RNGW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_phy(&self) -> PDEN_USB1_PHY_R {
        PDEN_USB1_PHY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_pll(&self) -> PDEN_USB1_PLL_R {
        PDEN_USB1_PLL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_aud_pll(&self) -> PDEN_AUD_PLL_R {
        PDEN_AUD_PLL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_sysosc(&self) -> PDEN_SYSOSC_R {
        PDEN_SYSOSC_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_eeprom(&self) -> PDEN_EEPROM_R {
        PDEN_EEPROM_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Random Number Generator Power."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PDEN_RNG_R {
        PDEN_RNG_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_phy(&mut self) -> _PDEN_USB1_PHYW {
        _PDEN_USB1_PHYW { w: self }
    }
    #[doc = "Bit 1 - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_pll(&mut self) -> _PDEN_USB1_PLLW {
        _PDEN_USB1_PLLW { w: self }
    }
    #[doc = "Bit 2 - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_aud_pll(&mut self) -> _PDEN_AUD_PLLW {
        _PDEN_AUD_PLLW { w: self }
    }
    #[doc = "Bit 3 - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_sysosc(&mut self) -> _PDEN_SYSOSCW {
        _PDEN_SYSOSCW { w: self }
    }
    #[doc = "Bit 5 - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_eeprom(&mut self) -> _PDEN_EEPROMW {
        _PDEN_EEPROMW { w: self }
    }
    #[doc = "Bit 7 - Random Number Generator Power."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> _PDEN_RNGW {
        _PDEN_RNGW { w: self }
    }
}
