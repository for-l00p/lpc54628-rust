#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL0 {
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
pub type FLASH_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLASH_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_RSTW<'a> {
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
pub type FMC_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FMC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FMC_RSTW<'a> {
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
pub type EEPROM_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EEPROM_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROM_RSTW<'a> {
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
pub type SPIFI_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SPIFI_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIFI_RSTW<'a> {
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
pub type MUX_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MUX_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MUX_RSTW<'a> {
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
pub type IOCON_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IOCON_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCON_RSTW<'a> {
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
pub type GPIO0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0_RSTW<'a> {
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
pub type GPIO1_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1_RSTW<'a> {
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
pub type GPIO2_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2_RSTW<'a> {
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
pub type GPIO3_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3_RSTW<'a> {
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
pub type PINT_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINT_RSTW<'a> {
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
pub type GINT_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GINT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _GINT_RSTW<'a> {
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
pub type DMA0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMA0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA0_RSTW<'a> {
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
pub type CRC_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CRC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_RSTW<'a> {
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
#[doc = r"Reader of the field"]
pub type WWDT_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WWDT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDT_RSTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADC0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADC0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_RSTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn flash_rst(&self) -> FLASH_RST_R {
        FLASH_RST_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fmc_rst(&self) -> FMC_RST_R {
        FMC_RST_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EEPROM reset control."]
    #[inline(always)]
    pub fn eeprom_rst(&self) -> EEPROM_RST_R {
        EEPROM_RST_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPIFI reset control."]
    #[inline(always)]
    pub fn spifi_rst(&self) -> SPIFI_RST_R {
        SPIFI_RST_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mux_rst(&self) -> MUX_RST_R {
        MUX_RST_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn iocon_rst(&self) -> IOCON_RST_R {
        IOCON_RST_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio0_rst(&self) -> GPIO0_RST_R {
        GPIO0_RST_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio1_rst(&self) -> GPIO1_RST_R {
        GPIO1_RST_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&self) -> GPIO2_RST_R {
        GPIO2_RST_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&self) -> GPIO3_RST_R {
        GPIO3_RST_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn pint_rst(&self) -> PINT_RST_R {
        PINT_RST_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gint_rst(&self) -> GINT_RST_R {
        GINT_RST_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dma0_rst(&self) -> DMA0_RST_R {
        DMA0_RST_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn crc_rst(&self) -> CRC_RST_R {
        CRC_RST_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn wwdt_rst(&self) -> WWDT_RST_R {
        WWDT_RST_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn adc0_rst(&self) -> ADC0_RST_R {
        ADC0_RST_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Flash controller reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn flash_rst(&mut self) -> _FLASH_RSTW {
        _FLASH_RSTW { w: self }
    }
    #[doc = "Bit 8 - Flash accelerator reset control. Note that the FMC must not be reset while executing from flash, and must be reconfigured after reset. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fmc_rst(&mut self) -> _FMC_RSTW {
        _FMC_RSTW { w: self }
    }
    #[doc = "Bit 9 - EEPROM reset control."]
    #[inline(always)]
    pub fn eeprom_rst(&mut self) -> _EEPROM_RSTW {
        _EEPROM_RSTW { w: self }
    }
    #[doc = "Bit 10 - SPIFI reset control."]
    #[inline(always)]
    pub fn spifi_rst(&mut self) -> _SPIFI_RSTW {
        _SPIFI_RSTW { w: self }
    }
    #[doc = "Bit 11 - Input mux reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mux_rst(&mut self) -> _MUX_RSTW {
        _MUX_RSTW { w: self }
    }
    #[doc = "Bit 13 - IOCON reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn iocon_rst(&mut self) -> _IOCON_RSTW {
        _IOCON_RSTW { w: self }
    }
    #[doc = "Bit 14 - GPIO0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio0_rst(&mut self) -> _GPIO0_RSTW {
        _GPIO0_RSTW { w: self }
    }
    #[doc = "Bit 15 - GPIO1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gpio1_rst(&mut self) -> _GPIO1_RSTW {
        _GPIO1_RSTW { w: self }
    }
    #[doc = "Bit 16 - GPIO2 reset control."]
    #[inline(always)]
    pub fn gpio2_rst(&mut self) -> _GPIO2_RSTW {
        _GPIO2_RSTW { w: self }
    }
    #[doc = "Bit 17 - GPIO3 reset control."]
    #[inline(always)]
    pub fn gpio3_rst(&mut self) -> _GPIO3_RSTW {
        _GPIO3_RSTW { w: self }
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn pint_rst(&mut self) -> _PINT_RSTW {
        _PINT_RSTW { w: self }
    }
    #[doc = "Bit 19 - Grouped interrupt (GINT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn gint_rst(&mut self) -> _GINT_RSTW {
        _GINT_RSTW { w: self }
    }
    #[doc = "Bit 20 - DMA0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dma0_rst(&mut self) -> _DMA0_RSTW {
        _DMA0_RSTW { w: self }
    }
    #[doc = "Bit 21 - CRC generator reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn crc_rst(&mut self) -> _CRC_RSTW {
        _CRC_RSTW { w: self }
    }
    #[doc = "Bit 22 - Watchdog timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn wwdt_rst(&mut self) -> _WWDT_RSTW {
        _WWDT_RSTW { w: self }
    }
    #[doc = "Bit 27 - ADC0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn adc0_rst(&mut self) -> _ADC0_RSTW {
        _ADC0_RSTW { w: self }
    }
}
