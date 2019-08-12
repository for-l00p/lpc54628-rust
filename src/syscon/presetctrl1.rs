#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL1 {
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
pub type MRT_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MRT_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MRT_RSTW<'a> {
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
pub type SCT0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCT0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_RSTW<'a> {
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
pub type MCAN0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MCAN0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MCAN0_RSTW<'a> {
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
pub type MCAN1_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MCAN1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MCAN1_RSTW<'a> {
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
pub type UTICK_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UTICK_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _UTICK_RSTW<'a> {
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
pub type FC0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC0_RSTW<'a> {
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
pub type FC1_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC1_RSTW<'a> {
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
pub type FC2_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC2_RSTW<'a> {
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
pub type FC3_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC3_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC3_RSTW<'a> {
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
pub type FC4_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC4_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC4_RSTW<'a> {
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
pub type FC5_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC5_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC5_RSTW<'a> {
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
pub type FC6_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC6_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC6_RSTW<'a> {
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
pub type FC7_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FC7_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _FC7_RSTW<'a> {
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
pub type DMIC_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMIC_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DMIC_RSTW<'a> {
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
pub type CTIMER2_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER2_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2_RSTW<'a> {
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
pub type USB0D_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0D_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0D_RSTW<'a> {
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
pub type CTIMER0_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER0_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER0_RSTW<'a> {
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
pub type CTIMER1_RST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER1_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER1_RSTW<'a> {
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
    #[doc = "Bit 0 - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mrt_rst(&self) -> MRT_RST_R {
        MRT_RST_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn sct0_rst(&self) -> SCT0_RST_R {
        SCT0_RST_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan0_rst(&self) -> MCAN0_RST_R {
        MCAN0_RST_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan1_rst(&self) -> MCAN1_RST_R {
        MCAN1_RST_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn utick_rst(&self) -> UTICK_RST_R {
        UTICK_RST_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc0_rst(&self) -> FC0_RST_R {
        FC0_RST_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc1_rst(&self) -> FC1_RST_R {
        FC1_RST_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc2_rst(&self) -> FC2_RST_R {
        FC2_RST_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc3_rst(&self) -> FC3_RST_R {
        FC3_RST_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc4_rst(&self) -> FC4_RST_R {
        FC4_RST_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc5_rst(&self) -> FC5_RST_R {
        FC5_RST_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc6_rst(&self) -> FC6_RST_R {
        FC6_RST_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc7_rst(&self) -> FC7_RST_R {
        FC7_RST_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dmic_rst(&self) -> DMIC_RST_R {
        DMIC_RST_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
    #[inline(always)]
    pub fn ctimer2_rst(&self) -> CTIMER2_RST_R {
        CTIMER2_RST_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn usb0d_rst(&self) -> USB0D_RST_R {
        USB0D_RST_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer0_rst(&self) -> CTIMER0_RST_R {
        CTIMER0_RST_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer1_rst(&self) -> CTIMER1_RST_R {
        CTIMER1_RST_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mrt_rst(&mut self) -> _MRT_RSTW {
        _MRT_RSTW { w: self }
    }
    #[doc = "Bit 2 - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn sct0_rst(&mut self) -> _SCT0_RSTW {
        _SCT0_RSTW { w: self }
    }
    #[doc = "Bit 7 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan0_rst(&mut self) -> _MCAN0_RSTW {
        _MCAN0_RSTW { w: self }
    }
    #[doc = "Bit 8 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan1_rst(&mut self) -> _MCAN1_RSTW {
        _MCAN1_RSTW { w: self }
    }
    #[doc = "Bit 10 - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn utick_rst(&mut self) -> _UTICK_RSTW {
        _UTICK_RSTW { w: self }
    }
    #[doc = "Bit 11 - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc0_rst(&mut self) -> _FC0_RSTW {
        _FC0_RSTW { w: self }
    }
    #[doc = "Bit 12 - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc1_rst(&mut self) -> _FC1_RSTW {
        _FC1_RSTW { w: self }
    }
    #[doc = "Bit 13 - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc2_rst(&mut self) -> _FC2_RSTW {
        _FC2_RSTW { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc3_rst(&mut self) -> _FC3_RSTW {
        _FC3_RSTW { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc4_rst(&mut self) -> _FC4_RSTW {
        _FC4_RSTW { w: self }
    }
    #[doc = "Bit 16 - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc5_rst(&mut self) -> _FC5_RSTW {
        _FC5_RSTW { w: self }
    }
    #[doc = "Bit 17 - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc6_rst(&mut self) -> _FC6_RSTW {
        _FC6_RSTW { w: self }
    }
    #[doc = "Bit 18 - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc7_rst(&mut self) -> _FC7_RSTW {
        _FC7_RSTW { w: self }
    }
    #[doc = "Bit 19 - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dmic_rst(&mut self) -> _DMIC_RSTW {
        _DMIC_RSTW { w: self }
    }
    #[doc = "Bit 22 - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
    #[inline(always)]
    pub fn ctimer2_rst(&mut self) -> _CTIMER2_RSTW {
        _CTIMER2_RSTW { w: self }
    }
    #[doc = "Bit 25 - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn usb0d_rst(&mut self) -> _USB0D_RSTW {
        _USB0D_RSTW { w: self }
    }
    #[doc = "Bit 26 - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer0_rst(&mut self) -> _CTIMER0_RSTW {
        _CTIMER0_RSTW { w: self }
    }
    #[doc = "Bit 27 - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer1_rst(&mut self) -> _CTIMER1_RSTW {
        _CTIMER1_RSTW { w: self }
    }
}
