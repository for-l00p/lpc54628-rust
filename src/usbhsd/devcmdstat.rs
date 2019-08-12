#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVCMDSTAT {
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
        0x0800
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type DEV_ADDR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DEV_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ADDRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DEV_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ENW<'a> {
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
pub type SETUP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SETUPW<'a> {
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
pub type FORCE_NEEDCLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FORCE_NEEDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_NEEDCLKW<'a> {
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
pub type FORCE_VBUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FORCE_VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_VBUSW<'a> {
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
pub type LPM_SUP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPM_SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SUPW<'a> {
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
pub type INTONNAK_AO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INTONNAK_AOW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_AOW<'a> {
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
pub type INTONNAK_AI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INTONNAK_AIW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_AIW<'a> {
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
pub type INTONNAK_CO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INTONNAK_COW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_COW<'a> {
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
pub type INTONNAK_CI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INTONNAK_CIW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_CIW<'a> {
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
pub type DCON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCONW<'a> {
    w: &'a mut W,
}
impl<'a> _DCONW<'a> {
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
pub type DSUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DSUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSUSW<'a> {
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
pub type LPM_SUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPM_SUSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SUSW<'a> {
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
pub type LPM_REWP_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SPEED_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type DCON_C_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCON_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DCON_CW<'a> {
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
pub type DSUS_C_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DSUS_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DSUS_CW<'a> {
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
pub type DRES_C_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRES_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DRES_CW<'a> {
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
pub type VBUS_DEBOUNCED_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PHY_TEST_MODE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHY_TEST_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_TEST_MODEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - USB device address."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable."]
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub fn force_needclk(&self) -> FORCE_NEEDCLK_R {
        FORCE_NEEDCLK_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled."]
    #[inline(always)]
    pub fn force_vbus(&self) -> FORCE_VBUS_R {
        FORCE_VBUS_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPM Supported:."]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend."]
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub fn vbus_debounced(&self) -> VBUS_DEBOUNCED_R {
        VBUS_DEBOUNCED_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2."]
    #[inline(always)]
    pub fn phy_test_mode(&self) -> PHY_TEST_MODE_R {
        PHY_TEST_MODE_R::new(((self.bits() >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - USB device address."]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> _DEV_ADDRW {
        _DEV_ADDRW { w: self }
    }
    #[doc = "Bit 7 - USB device enable."]
    #[inline(always)]
    pub fn dev_en(&mut self) -> _DEV_ENW {
        _DEV_ENW { w: self }
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    pub fn setup(&mut self) -> _SETUPW {
        _SETUPW { w: self }
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub fn force_needclk(&mut self) -> _FORCE_NEEDCLKW {
        _FORCE_NEEDCLKW { w: self }
    }
    #[doc = "Bit 10 - If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled."]
    #[inline(always)]
    pub fn force_vbus(&mut self) -> _FORCE_VBUSW {
        _FORCE_VBUSW { w: self }
    }
    #[doc = "Bit 11 - LPM Supported:."]
    #[inline(always)]
    pub fn lpm_sup(&mut self) -> _LPM_SUPW {
        _LPM_SUPW { w: self }
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub fn intonnak_ao(&mut self) -> _INTONNAK_AOW {
        _INTONNAK_AOW { w: self }
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub fn intonnak_ai(&mut self) -> _INTONNAK_AIW {
        _INTONNAK_AIW { w: self }
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub fn intonnak_co(&mut self) -> _INTONNAK_COW {
        _INTONNAK_COW { w: self }
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub fn intonnak_ci(&mut self) -> _INTONNAK_CIW {
        _INTONNAK_CIW { w: self }
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    pub fn dcon(&mut self) -> _DCONW {
        _DCONW { w: self }
    }
    #[doc = "Bit 17 - Device status - suspend."]
    #[inline(always)]
    pub fn dsus(&mut self) -> _DSUSW {
        _DSUSW { w: self }
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    pub fn lpm_sus(&mut self) -> _LPM_SUSW {
        _LPM_SUSW { w: self }
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    pub fn dcon_c(&mut self) -> _DCON_CW {
        _DCON_CW { w: self }
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    pub fn dsus_c(&mut self) -> _DSUS_CW {
        _DSUS_CW { w: self }
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    pub fn dres_c(&mut self) -> _DRES_CW {
        _DRES_CW { w: self }
    }
    #[doc = "Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2."]
    #[inline(always)]
    pub fn phy_test_mode(&mut self) -> _PHY_TEST_MODEW {
        _PHY_TEST_MODEW { w: self }
    }
}
