#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTMODE {
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
        0x0004_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type ID0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ID0W<'a> {
    w: &'a mut W,
}
impl<'a> _ID0W<'a> {
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
pub type ID0_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ID0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ID0_ENW<'a> {
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
pub type DEV_ENABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEV_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ENABLEW<'a> {
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
pub type SW_CTRL_PDCOM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SW_CTRL_PDCOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_CTRL_PDCOMW<'a> {
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
pub type SW_PDCOM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SW_PDCOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_PDCOMW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Port 0 ID pin value."]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 0 ID pin pull-up enable."]
    #[inline(always)]
    pub fn id0_en(&self) -> ID0_EN_R {
        ID0_EN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&self) -> SW_CTRL_PDCOM_R {
        SW_CTRL_PDCOM_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&self) -> SW_PDCOM_R {
        SW_PDCOM_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Port 0 ID pin value."]
    #[inline(always)]
    pub fn id0(&mut self) -> _ID0W {
        _ID0W { w: self }
    }
    #[doc = "Bit 8 - Port 0 ID pin pull-up enable."]
    #[inline(always)]
    pub fn id0_en(&mut self) -> _ID0_ENW {
        _ID0_ENW { w: self }
    }
    #[doc = "Bit 16 - If this bit is set to one, one of the ports will behave as a USB device."]
    #[inline(always)]
    pub fn dev_enable(&mut self) -> _DEV_ENABLEW {
        _DEV_ENABLEW { w: self }
    }
    #[doc = "Bit 18 - This bit indicates if the PHY power-down input is controlled by software or by hardware."]
    #[inline(always)]
    pub fn sw_ctrl_pdcom(&mut self) -> _SW_CTRL_PDCOMW {
        _SW_CTRL_PDCOMW { w: self }
    }
    #[doc = "Bit 19 - This bit is only used when SW_CTRL_PDCOM is set to 1b."]
    #[inline(always)]
    pub fn sw_pdcom(&mut self) -> _SW_PDCOMW {
        _SW_PDCOMW { w: self }
    }
}
