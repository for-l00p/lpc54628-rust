#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB0CLKCTRL {
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
pub type AP_FS_DEV_CLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AP_FS_DEV_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AP_FS_DEV_CLKW<'a> {
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
pub type POL_FS_DEV_CLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _POL_FS_DEV_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _POL_FS_DEV_CLKW<'a> {
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
pub type AP_FS_HOST_CLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AP_FS_HOST_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AP_FS_HOST_CLKW<'a> {
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
pub type POL_FS_HOST_CLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _POL_FS_HOST_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _POL_FS_HOST_CLKW<'a> {
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
pub type PU_DISABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PU_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PU_DISABLEW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&self) -> AP_FS_DEV_CLK_R {
        AP_FS_DEV_CLK_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&self) -> POL_FS_DEV_CLK_R {
        POL_FS_DEV_CLK_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&self) -> AP_FS_HOST_CLK_R {
        AP_FS_HOST_CLK_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&self) -> POL_FS_HOST_CLK_R {
        POL_FS_HOST_CLK_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Internal pull-up disable control."]
    #[inline(always)]
    pub fn pu_disable(&self) -> PU_DISABLE_R {
        PU_DISABLE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_dev_clk(&mut self) -> _AP_FS_DEV_CLKW {
        _AP_FS_DEV_CLKW { w: self }
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_dev_clk(&mut self) -> _POL_FS_DEV_CLKW {
        _POL_FS_DEV_CLKW { w: self }
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control."]
    #[inline(always)]
    pub fn ap_fs_host_clk(&mut self) -> _AP_FS_HOST_CLKW {
        _AP_FS_HOST_CLKW { w: self }
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt."]
    #[inline(always)]
    pub fn pol_fs_host_clk(&mut self) -> _POL_FS_HOST_CLKW {
        _POL_FS_HOST_CLKW { w: self }
    }
    #[doc = "Bit 4 - Internal pull-up disable control."]
    #[inline(always)]
    pub fn pu_disable(&mut self) -> _PU_DISABLEW {
        _PU_DISABLEW { w: self }
    }
}
