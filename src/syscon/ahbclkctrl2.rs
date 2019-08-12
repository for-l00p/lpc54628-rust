#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRL2 {
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
pub type LCD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDW<'a> {
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
pub type SDIO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIOW<'a> {
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
pub type USB1H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1HW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1HW<'a> {
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
pub type USB1D_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1DW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1DW<'a> {
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
pub type USB1RAM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1RAMW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1RAMW<'a> {
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
pub type EMC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMCW<'a> {
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
pub type ETH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETHW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHW<'a> {
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
pub type GPIO4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO4W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4W<'a> {
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
pub type GPIO5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO5W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5W<'a> {
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
pub type AES_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AESW<'a> {
    w: &'a mut W,
}
impl<'a> _AESW<'a> {
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
pub type OTP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OTPW<'a> {
    w: &'a mut W,
}
impl<'a> _OTPW<'a> {
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
pub type RNG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RNGW<'a> {
    w: &'a mut W,
}
impl<'a> _RNGW<'a> {
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
pub type FLEXCOMM8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM8W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM8W<'a> {
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
pub type FLEXCOMM9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM9W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM9W<'a> {
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
pub type USB0HMR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0HMRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0HMRW<'a> {
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
pub type USB0HSL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0HSLW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0HSLW<'a> {
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
pub type SHA0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SHA0W<'a> {
    w: &'a mut W,
}
impl<'a> _SHA0W<'a> {
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
pub type SC0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SC0W<'a> {
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
pub type SC1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SC1W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&self) -> USB1H_R {
        USB1H_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&self) -> USB1D_R {
        USB1D_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&self) -> USB1RAM_R {
        USB1RAM_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&self) -> EMC_R {
        EMC_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&self) -> USB0HMR_R {
        USB0HMR_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&self) -> USB0HSL_R {
        USB0HSL_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&self) -> SHA0_R {
        SHA0_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&self) -> SC0_R {
        SC0_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&self) -> SC1_R {
        SC1_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Enables the clock for the LCD interface."]
    #[inline(always)]
    pub fn lcd(&mut self) -> _LCDW {
        _LCDW { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for the SDIO interface."]
    #[inline(always)]
    pub fn sdio(&mut self) -> _SDIOW {
        _SDIOW { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for the USB1 host interface."]
    #[inline(always)]
    pub fn usb1h(&mut self) -> _USB1HW {
        _USB1HW { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for the USB1 device interface."]
    #[inline(always)]
    pub fn usb1d(&mut self) -> _USB1DW {
        _USB1DW { w: self }
    }
    #[doc = "Bit 6 - Enables the clock for the USB1 RAM interface."]
    #[inline(always)]
    pub fn usb1ram(&mut self) -> _USB1RAMW {
        _USB1RAMW { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the EMC interface."]
    #[inline(always)]
    pub fn emc(&mut self) -> _EMCW {
        _EMCW { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the ethernet interface."]
    #[inline(always)]
    pub fn eth(&mut self) -> _ETHW {
        _ETHW { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for the GPIO4 interface."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> _GPIO4W {
        _GPIO4W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the GPIO5 interface."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> _GPIO5W {
        _GPIO5W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the AES interface."]
    #[inline(always)]
    pub fn aes(&mut self) -> _AESW {
        _AESW { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for the OTP interface."]
    #[inline(always)]
    pub fn otp(&mut self) -> _OTPW {
        _OTPW { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the RNG interface."]
    #[inline(always)]
    pub fn rng(&mut self) -> _RNGW {
        _RNGW { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the Flexcomm8 interface."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> _FLEXCOMM8W {
        _FLEXCOMM8W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the Flexcomm9 interface."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> _FLEXCOMM9W {
        _FLEXCOMM9W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the USB host master interface."]
    #[inline(always)]
    pub fn usb0hmr(&mut self) -> _USB0HMRW {
        _USB0HMRW { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the USB host slave interface."]
    #[inline(always)]
    pub fn usb0hsl(&mut self) -> _USB0HSLW {
        _USB0HSLW { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the SHA interface."]
    #[inline(always)]
    pub fn sha0(&mut self) -> _SHA0W {
        _SHA0W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the Smart card0 interface."]
    #[inline(always)]
    pub fn sc0(&mut self) -> _SC0W {
        _SC0W { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the Smart card1 interface."]
    #[inline(always)]
    pub fn sc1(&mut self) -> _SC1W {
        _SC1W { w: self }
    }
}
