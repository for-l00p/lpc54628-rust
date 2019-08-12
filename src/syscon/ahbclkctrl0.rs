#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRL0 {
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
        0x0183
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type ROM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _ROMW<'a> {
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
pub type SRAM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SRAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM1W<'a> {
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
pub type SRAM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SRAM2W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM2W<'a> {
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
pub type SRAM3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SRAM3W<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM3W<'a> {
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
pub type FLASH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLASHW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHW<'a> {
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
pub type FMC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FMCW<'a> {
    w: &'a mut W,
}
impl<'a> _FMCW<'a> {
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
pub type EEPROM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EEPROMW<'a> {
    w: &'a mut W,
}
impl<'a> _EEPROMW<'a> {
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
pub type SPIFI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SPIFIW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIFIW<'a> {
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
pub type INPUTMUX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INPUTMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTMUXW<'a> {
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
pub type IOCON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IOCONW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCONW<'a> {
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
pub type GPIO0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO0W<'a> {
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
pub type GPIO1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO1W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO1W<'a> {
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
pub type GPIO2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO2W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO2W<'a> {
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
pub type GPIO3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPIO3W<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO3W<'a> {
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
pub type PINT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINTW<'a> {
    w: &'a mut W,
}
impl<'a> _PINTW<'a> {
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
pub type GINT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GINTW<'a> {
    w: &'a mut W,
}
impl<'a> _GINTW<'a> {
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
pub type DMA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
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
pub type CRC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCW<'a> {
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
pub type WWDT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WWDTW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDTW<'a> {
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
pub type RTC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RTCW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADC0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADC0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0W<'a> {
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
    #[doc = "Bit 1 - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for SRAM3."]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the clock for EEPROM."]
    #[inline(always)]
    pub fn eeprom(&self) -> EEPROM_R {
        EEPROM_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn spifi(&self) -> SPIFI_R {
        SPIFI_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn inputmux(&self) -> INPUTMUX_R {
        INPUTMUX_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2 port registers."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3 port registers."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC0 register interface."]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Enables the clock for the Boot ROM. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rom(&mut self) -> _ROMW {
        _ROMW { w: self }
    }
    #[doc = "Bit 3 - Enables the clock for SRAM1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram1(&mut self) -> _SRAM1W {
        _SRAM1W { w: self }
    }
    #[doc = "Bit 4 - Enables the clock for SRAM2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn sram2(&mut self) -> _SRAM2W {
        _SRAM2W { w: self }
    }
    #[doc = "Bit 5 - Enables the clock for SRAM3."]
    #[inline(always)]
    pub fn sram3(&mut self) -> _SRAM3W {
        _SRAM3W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for the flash controller. 0 = Disable; 1 = Enable. This clock is needed for flash programming, not for flash read."]
    #[inline(always)]
    pub fn flash(&mut self) -> _FLASHW {
        _FLASHW { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for the Flash accelerator. 0 = Disable; 1 = Enable. This clock is needed if the flash is being read."]
    #[inline(always)]
    pub fn fmc(&mut self) -> _FMCW {
        _FMCW { w: self }
    }
    #[doc = "Bit 9 - Enables the clock for EEPROM."]
    #[inline(always)]
    pub fn eeprom(&mut self) -> _EEPROMW {
        _EEPROMW { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the SPIFI. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn spifi(&mut self) -> _SPIFIW {
        _SPIFIW { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for the input muxes. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn inputmux(&mut self) -> _INPUTMUXW {
        _INPUTMUXW { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for the IOCON block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn iocon(&mut self) -> _IOCONW {
        _IOCONW { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> _GPIO0W {
        _GPIO0W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1 port registers. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> _GPIO1W {
        _GPIO1W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2 port registers."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> _GPIO2W {
        _GPIO2W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3 port registers."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> _GPIO3W {
        _GPIO3W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for the pin interrupt block.0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn pint(&mut self) -> _PINTW {
        _PINTW { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the grouped pin interrupt block. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn gint(&mut self) -> _GINTW {
        _GINTW { w: self }
    }
    #[doc = "Bit 20 - Enables the clock for the DMA controller. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 21 - Enables the clock for the CRC engine. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn crc(&mut self) -> _CRCW {
        _CRCW { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> _WWDTW {
        _WWDTW { w: self }
    }
    #[doc = "Bit 23 - Enables the bus clock for the RTC. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for the ADC0 register interface."]
    #[inline(always)]
    pub fn adc0(&mut self) -> _ADC0W {
        _ADC0W { w: self }
    }
}
