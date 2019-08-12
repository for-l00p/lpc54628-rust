#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRL1 {
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
pub type MRT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MRTW<'a> {
    w: &'a mut W,
}
impl<'a> _MRTW<'a> {
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
pub type RIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RITW<'a> {
    w: &'a mut W,
}
impl<'a> _RITW<'a> {
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
pub type SCT0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0W<'a> {
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
pub type MCAN0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MCAN0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCAN0W<'a> {
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
pub type MCAN1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MCAN1W<'a> {
    w: &'a mut W,
}
impl<'a> _MCAN1W<'a> {
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
pub type UTICK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UTICKW<'a> {
    w: &'a mut W,
}
impl<'a> _UTICKW<'a> {
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
pub type FLEXCOMM0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM0W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM0W<'a> {
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
pub type FLEXCOMM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM1W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM1W<'a> {
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
pub type FLEXCOMM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM2W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM2W<'a> {
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
pub type FLEXCOMM3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM3W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM3W<'a> {
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
pub type FLEXCOMM4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM4W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM4W<'a> {
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
pub type FLEXCOMM5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM5W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM5W<'a> {
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
pub type FLEXCOMM6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM6W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM6W<'a> {
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
pub type FLEXCOMM7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM7W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM7W<'a> {
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
pub type DMIC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMICW<'a> {
    w: &'a mut W,
}
impl<'a> _DMICW<'a> {
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
pub type CTIMER2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2W<'a> {
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
pub type USB0D_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0DW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0DW<'a> {
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
pub type CTIMER0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER0W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER0W<'a> {
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
pub type CTIMER1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER1W<'a> {
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
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&self) -> RIT_R {
        RIT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&self) -> MCAN1_R {
        MCAN1_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&self) -> USB0D_R {
        USB0D_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&mut self) -> _MRTW {
        _MRTW { w: self }
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&mut self) -> _RITW {
        _RITW { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&mut self) -> _SCT0W {
        _SCT0W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&mut self) -> _MCAN0W {
        _MCAN0W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&mut self) -> _MCAN1W {
        _MCAN1W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&mut self) -> _UTICKW {
        _UTICKW { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> _FLEXCOMM0W {
        _FLEXCOMM0W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> _FLEXCOMM1W {
        _FLEXCOMM1W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> _FLEXCOMM2W {
        _FLEXCOMM2W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> _FLEXCOMM3W {
        _FLEXCOMM3W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> _FLEXCOMM4W {
        _FLEXCOMM4W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> _FLEXCOMM5W {
        _FLEXCOMM5W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> _FLEXCOMM6W {
        _FLEXCOMM6W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> _FLEXCOMM7W {
        _FLEXCOMM7W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&mut self) -> _DMICW {
        _DMICW { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> _CTIMER2W {
        _CTIMER2W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&mut self) -> _USB0DW {
        _USB0DW { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> _CTIMER0W {
        _CTIMER0W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> _CTIMER1W {
        _CTIMER1W { w: self }
    }
}
