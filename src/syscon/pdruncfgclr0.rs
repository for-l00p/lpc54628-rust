#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDRUNCFGCLR0 {
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
pub type PDEN_FRO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_FROW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_FROW<'a> {
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
pub type PDEN_TS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_TSW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_TSW<'a> {
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
pub type PDEN_BOD_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_BOD_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_BOD_RSTW<'a> {
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
pub type PDEN_BOD_INTR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_BOD_INTRW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_BOD_INTRW<'a> {
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
pub type PDEN_VD2_ANA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VD2_ANAW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VD2_ANAW<'a> {
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
pub type PDEN_ADC0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_ADC0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_ADC0W<'a> {
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
pub type PDEN_SRAMX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_SRAMXW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_SRAMXW<'a> {
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
pub type PDEN_SRAM0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_SRAM0W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_SRAM0W<'a> {
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
pub type PDEN_SRAM1_2_3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_SRAM1_2_3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_SRAM1_2_3W<'a> {
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
pub type PDEN_USB_RAM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_USB_RAMW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_USB_RAMW<'a> {
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
pub type PDEN_ROM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_ROMW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_ROMW<'a> {
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
pub type PDEN_VDDA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VDDAW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VDDAW<'a> {
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
pub type PDEN_WDT_OSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_WDT_OSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_WDT_OSCW<'a> {
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
pub type PDEN_USB0_PHY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_USB0_PHYW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_USB0_PHYW<'a> {
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
pub type PDEN_SYS_PLL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_SYS_PLLW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_SYS_PLLW<'a> {
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
pub type PDEN_VREFP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VREFPW<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VREFPW<'a> {
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
pub type PDEN_VD3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VD3W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VD3W<'a> {
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
pub type PDEN_VD4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VD4W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VD4W<'a> {
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
pub type PDEN_VD5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VD5W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VD5W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PDEN_VD6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDEN_VD6W<'a> {
    w: &'a mut W,
}
impl<'a> _PDEN_VD6W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - FRO oscillator."]
    #[inline(always)]
    pub fn pden_fro(&self) -> PDEN_FRO_R {
        PDEN_FRO_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Temp sensor."]
    #[inline(always)]
    pub fn pden_ts(&self) -> PDEN_TS_R {
        PDEN_TS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Brown-out Detect reset."]
    #[inline(always)]
    pub fn pden_bod_rst(&self) -> PDEN_BOD_RST_R {
        PDEN_BOD_RST_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Brown-out Detect interrupt."]
    #[inline(always)]
    pub fn pden_bod_intr(&self) -> PDEN_BOD_INTR_R {
        PDEN_BOD_INTR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
    #[inline(always)]
    pub fn pden_vd2_ana(&self) -> PDEN_VD2_ANA_R {
        PDEN_VD2_ANA_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC power."]
    #[inline(always)]
    pub fn pden_adc0(&self) -> PDEN_ADC0_R {
        PDEN_ADC0_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sramx(&self) -> PDEN_SRAMX_R {
        PDEN_SRAMX_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram0(&self) -> PDEN_SRAM0_R {
        PDEN_SRAM0_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram1_2_3(&self) -> PDEN_SRAM1_2_3_R {
        PDEN_SRAM1_2_3_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_usb_ram(&self) -> PDEN_USB_RAM_R {
        PDEN_USB_RAM_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ROM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_rom(&self) -> PDEN_ROM_R {
        PDEN_ROM_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
    #[inline(always)]
    pub fn pden_vdda(&self) -> PDEN_VDDA_R {
        PDEN_VDDA_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Watchdog oscillator."]
    #[inline(always)]
    pub fn pden_wdt_osc(&self) -> PDEN_WDT_OSC_R {
        PDEN_WDT_OSC_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USB0 PHY power (also enable/disable bit 28)."]
    #[inline(always)]
    pub fn pden_usb0_phy(&self) -> PDEN_USB0_PHY_R {
        PDEN_USB0_PHY_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - System PLL (PLL0) power (also enable/disable bit 26)."]
    #[inline(always)]
    pub fn pden_sys_pll(&self) -> PDEN_SYS_PLL_R {
        PDEN_SYS_PLL_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
    #[inline(always)]
    pub fn pden_vrefp(&self) -> PDEN_VREFP_R {
        PDEN_VREFP_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Power control for all PLLs."]
    #[inline(always)]
    pub fn pden_vd3(&self) -> PDEN_VD3_R {
        PDEN_VD3_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Power control for all SRAMs and ROM."]
    #[inline(always)]
    pub fn pden_vd4(&self) -> PDEN_VD4_R {
        PDEN_VD4_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power control both USB0 PHY and USB1 PHY."]
    #[inline(always)]
    pub fn pden_vd5(&self) -> PDEN_VD5_R {
        PDEN_VD5_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Power control for EEPROM."]
    #[inline(always)]
    pub fn pden_vd6(&self) -> PDEN_VD6_R {
        PDEN_VD6_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - FRO oscillator."]
    #[inline(always)]
    pub fn pden_fro(&mut self) -> _PDEN_FROW {
        _PDEN_FROW { w: self }
    }
    #[doc = "Bit 6 - Temp sensor."]
    #[inline(always)]
    pub fn pden_ts(&mut self) -> _PDEN_TSW {
        _PDEN_TSW { w: self }
    }
    #[doc = "Bit 7 - Brown-out Detect reset."]
    #[inline(always)]
    pub fn pden_bod_rst(&mut self) -> _PDEN_BOD_RSTW {
        _PDEN_BOD_RSTW { w: self }
    }
    #[doc = "Bit 8 - Brown-out Detect interrupt."]
    #[inline(always)]
    pub fn pden_bod_intr(&mut self) -> _PDEN_BOD_INTRW {
        _PDEN_BOD_INTRW { w: self }
    }
    #[doc = "Bit 9 - Analog supply for System Oscillator (also enable/disable bit 3 in PDRUNCFG1 register), Temperature Sensor (also, enable/disable bit 6), ADC (also, enable/disable bits 10, 19, and 23)."]
    #[inline(always)]
    pub fn pden_vd2_ana(&mut self) -> _PDEN_VD2_ANAW {
        _PDEN_VD2_ANAW { w: self }
    }
    #[doc = "Bit 10 - ADC power."]
    #[inline(always)]
    pub fn pden_adc0(&mut self) -> _PDEN_ADC0W {
        _PDEN_ADC0W { w: self }
    }
    #[doc = "Bit 13 - PDEN_SRAMX controls SRAMX (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sramx(&mut self) -> _PDEN_SRAMXW {
        _PDEN_SRAMXW { w: self }
    }
    #[doc = "Bit 14 - PDEN_SRAM0 controls SRAM0 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram0(&mut self) -> _PDEN_SRAM0W {
        _PDEN_SRAM0W { w: self }
    }
    #[doc = "Bit 15 - PDEN_SRAM1_2_3 controls SRAM1, SRAM2, and SRAM3 (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_sram1_2_3(&mut self) -> _PDEN_SRAM1_2_3W {
        _PDEN_SRAM1_2_3W { w: self }
    }
    #[doc = "Bit 16 - PDEN_USB_SRAM controls USB_RAM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_usb_ram(&mut self) -> _PDEN_USB_RAMW {
        _PDEN_USB_RAMW { w: self }
    }
    #[doc = "Bit 17 - ROM (also enable/disable bit 27)."]
    #[inline(always)]
    pub fn pden_rom(&mut self) -> _PDEN_ROMW {
        _PDEN_ROMW { w: self }
    }
    #[doc = "Bit 19 - Vdda to the ADC, must be enabled for the ADC to work (also enable/disable bit 9, 10, and 23)."]
    #[inline(always)]
    pub fn pden_vdda(&mut self) -> _PDEN_VDDAW {
        _PDEN_VDDAW { w: self }
    }
    #[doc = "Bit 20 - Watchdog oscillator."]
    #[inline(always)]
    pub fn pden_wdt_osc(&mut self) -> _PDEN_WDT_OSCW {
        _PDEN_WDT_OSCW { w: self }
    }
    #[doc = "Bit 21 - USB0 PHY power (also enable/disable bit 28)."]
    #[inline(always)]
    pub fn pden_usb0_phy(&mut self) -> _PDEN_USB0_PHYW {
        _PDEN_USB0_PHYW { w: self }
    }
    #[doc = "Bit 22 - System PLL (PLL0) power (also enable/disable bit 26)."]
    #[inline(always)]
    pub fn pden_sys_pll(&mut self) -> _PDEN_SYS_PLLW {
        _PDEN_SYS_PLLW { w: self }
    }
    #[doc = "Bit 23 - VREFP to the ADC must be enabled for the ADC to work (also enable/disable bit 9, 10, and 19)."]
    #[inline(always)]
    pub fn pden_vrefp(&mut self) -> _PDEN_VREFPW {
        _PDEN_VREFPW { w: self }
    }
    #[doc = "Bit 26 - Power control for all PLLs."]
    #[inline(always)]
    pub fn pden_vd3(&mut self) -> _PDEN_VD3W {
        _PDEN_VD3W { w: self }
    }
    #[doc = "Bit 27 - Power control for all SRAMs and ROM."]
    #[inline(always)]
    pub fn pden_vd4(&mut self) -> _PDEN_VD4W {
        _PDEN_VD4W { w: self }
    }
    #[doc = "Bit 28 - Power control both USB0 PHY and USB1 PHY."]
    #[inline(always)]
    pub fn pden_vd5(&mut self) -> _PDEN_VD5W {
        _PDEN_VD5W { w: self }
    }
    #[doc = "Bit 29 - Power control for EEPROM."]
    #[inline(always)]
    pub fn pden_vd6(&mut self) -> _PDEN_VD6W {
        _PDEN_VD6W { w: self }
    }
}
