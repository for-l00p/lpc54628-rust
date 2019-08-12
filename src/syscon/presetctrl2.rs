#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL2 {
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
pub type LCD_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCD_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _LCD_RSTW<'a> {
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
pub type SDIO_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SDIO_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIO_RSTW<'a> {
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
pub type USB1H_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1H_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1H_RSTW<'a> {
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
pub type USB1D_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1D_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1D_RSTW<'a> {
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
pub type USB1RAM_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1RAM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1RAM_RSTW<'a> {
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
pub type EMC_RESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMC_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _EMC_RESETW<'a> {
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
pub type ETH_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETH_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ETH_RSTW<'a> {
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
pub type GPIO4_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO4_RSTW<'a> {
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
pub type GPIO5_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO5_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO5_RSTW<'a> {
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
pub type AES_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AES_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _AES_RSTW<'a> {
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
pub type OTP_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OTP_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _OTP_RSTW<'a> {
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
pub type RNG_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RNG_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RNG_RSTW<'a> {
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
pub type FC8_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC8_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC8_RSTW<'a> {
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
pub type FC9_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC9_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC9_RSTW<'a> {
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
pub type USB0HMR_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0HMR_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0HMR_RSTW<'a> {
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
pub type USB0HSL_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0HSL_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0HSL_RSTW<'a> {
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
pub type SHA_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SHA_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SHA_RSTW<'a> {
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
pub type SC0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SC0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SC0_RSTW<'a> {
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
pub type SC1_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SC1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SC1_RSTW<'a> {
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
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&self) -> LCD_RST_R {
        LCD_RST_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&self) -> USB1H_RST_R {
        USB1H_RST_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&self) -> USB1D_RST_R {
        USB1D_RST_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&self) -> USB1RAM_RST_R {
        USB1RAM_RST_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&self) -> EMC_RESET_R {
        EMC_RESET_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&self) -> ETH_RST_R {
        ETH_RST_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&self) -> GPIO4_RST_R {
        GPIO4_RST_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&self) -> GPIO5_RST_R {
        GPIO5_RST_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&self) -> AES_RST_R {
        AES_RST_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&self) -> OTP_RST_R {
        OTP_RST_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&self) -> FC8_RST_R {
        FC8_RST_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&self) -> FC9_RST_R {
        FC9_RST_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&self) -> USB0HMR_RST_R {
        USB0HMR_RST_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&self) -> USB0HSL_RST_R {
        USB0HSL_RST_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&self) -> SHA_RST_R {
        SHA_RST_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&self) -> SC0_RST_R {
        SC0_RST_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&self) -> SC1_RST_R {
        SC1_RST_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - LCD reset control."]
    #[inline(always)]
    pub fn lcd_rst(&mut self) -> _LCD_RSTW {
        _LCD_RSTW { w: self }
    }
    #[doc = "Bit 3 - SDIO reset control."]
    #[inline(always)]
    pub fn sdio_rst(&mut self) -> _SDIO_RSTW {
        _SDIO_RSTW { w: self }
    }
    #[doc = "Bit 4 - USB1 Host reset control."]
    #[inline(always)]
    pub fn usb1h_rst(&mut self) -> _USB1H_RSTW {
        _USB1H_RSTW { w: self }
    }
    #[doc = "Bit 5 - USB1 Device reset control."]
    #[inline(always)]
    pub fn usb1d_rst(&mut self) -> _USB1D_RSTW {
        _USB1D_RSTW { w: self }
    }
    #[doc = "Bit 6 - USB1 RAM reset control."]
    #[inline(always)]
    pub fn usb1ram_rst(&mut self) -> _USB1RAM_RSTW {
        _USB1RAM_RSTW { w: self }
    }
    #[doc = "Bit 7 - EMC reset control."]
    #[inline(always)]
    pub fn emc_reset(&mut self) -> _EMC_RESETW {
        _EMC_RESETW { w: self }
    }
    #[doc = "Bit 8 - Ethernet reset control."]
    #[inline(always)]
    pub fn eth_rst(&mut self) -> _ETH_RSTW {
        _ETH_RSTW { w: self }
    }
    #[doc = "Bit 9 - GPIO4 reset control."]
    #[inline(always)]
    pub fn gpio4_rst(&mut self) -> _GPIO4_RSTW {
        _GPIO4_RSTW { w: self }
    }
    #[doc = "Bit 10 - GPIO5 reset control."]
    #[inline(always)]
    pub fn gpio5_rst(&mut self) -> _GPIO5_RSTW {
        _GPIO5_RSTW { w: self }
    }
    #[doc = "Bit 11 - AES reset control."]
    #[inline(always)]
    pub fn aes_rst(&mut self) -> _AES_RSTW {
        _AES_RSTW { w: self }
    }
    #[doc = "Bit 12 - OTP reset control."]
    #[inline(always)]
    pub fn otp_rst(&mut self) -> _OTP_RSTW {
        _OTP_RSTW { w: self }
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> _RNG_RSTW {
        _RNG_RSTW { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 8 reset control."]
    #[inline(always)]
    pub fn fc8_rst(&mut self) -> _FC8_RSTW {
        _FC8_RSTW { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 9 reset control."]
    #[inline(always)]
    pub fn fc9_rst(&mut self) -> _FC9_RSTW {
        _FC9_RSTW { w: self }
    }
    #[doc = "Bit 16 - USB0 HOST master reset control."]
    #[inline(always)]
    pub fn usb0hmr_rst(&mut self) -> _USB0HMR_RSTW {
        _USB0HMR_RSTW { w: self }
    }
    #[doc = "Bit 17 - USB0 HOST slave reset control."]
    #[inline(always)]
    pub fn usb0hsl_rst(&mut self) -> _USB0HSL_RSTW {
        _USB0HSL_RSTW { w: self }
    }
    #[doc = "Bit 18 - SHA reset control."]
    #[inline(always)]
    pub fn sha_rst(&mut self) -> _SHA_RSTW {
        _SHA_RSTW { w: self }
    }
    #[doc = "Bit 19 - Smart card 0 reset control."]
    #[inline(always)]
    pub fn sc0_rst(&mut self) -> _SC0_RSTW {
        _SC0_RSTW { w: self }
    }
    #[doc = "Bit 20 - Smart card 1 reset control."]
    #[inline(always)]
    pub fn sc1_rst(&mut self) -> _SC1_RSTW {
        _SC1_RSTW { w: self }
    }
}
