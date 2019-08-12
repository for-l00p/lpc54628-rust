#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTER0 {
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
pub type WDT_BOD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDT_BODW<'a> {
    w: &'a mut W,
}
impl<'a> _WDT_BODW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type GINT0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _GINT0W<'a> {
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
pub type GINT1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _GINT1W<'a> {
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
pub type PIN_INT0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PIN_INT0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN_INT0W<'a> {
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
pub type PIN_INT1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PIN_INT1W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN_INT1W<'a> {
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
pub type PIN_INT2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PIN_INT2W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN_INT2W<'a> {
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
pub type PIN_INT3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PIN_INT3W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN_INT3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CTIMER3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADC0_SEQA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADC0_SEQAW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_SEQAW<'a> {
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
pub type ADC0_SEQB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADC0_SEQBW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_SEQBW<'a> {
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
pub type ADC0_THCMP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADC0_THCMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0_THCMPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HWVAD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HWVADW<'a> {
    w: &'a mut W,
}
impl<'a> _HWVADW<'a> {
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
pub type USB0_NEEDCLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0_NEEDCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB0_NEEDCLKW<'a> {
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
pub type USB0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB0W<'a> {
    w: &'a mut W,
}
impl<'a> _USB0W<'a> {
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
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&self) -> WDT_BOD_R {
        WDT_BOD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&self) -> GINT0_R {
        GINT0_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&self) -> GINT1_R {
        GINT1_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&self) -> PIN_INT0_R {
        PIN_INT0_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&self) -> PIN_INT1_R {
        PIN_INT1_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&self) -> PIN_INT2_R {
        PIN_INT2_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&self) -> PIN_INT3_R {
        PIN_INT3_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&self) -> ADC0_SEQA_R {
        ADC0_SEQA_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&self) -> ADC0_SEQB_R {
        ADC0_SEQB_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&self) -> ADC0_THCMP_R {
        ADC0_THCMP_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&self) -> HWVAD_R {
        HWVAD_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&mut self) -> _WDT_BODW {
        _WDT_BODW { w: self }
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&mut self) -> _GINT0W {
        _GINT0W { w: self }
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&mut self) -> _GINT1W {
        _GINT1W { w: self }
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&mut self) -> _PIN_INT0W {
        _PIN_INT0W { w: self }
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&mut self) -> _PIN_INT1W {
        _PIN_INT1W { w: self }
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&mut self) -> _PIN_INT2W {
        _PIN_INT2W { w: self }
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&mut self) -> _PIN_INT3W {
        _PIN_INT3W { w: self }
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&mut self) -> _UTICKW {
        _UTICKW { w: self }
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&mut self) -> _MRTW {
        _MRTW { w: self }
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> _CTIMER0W {
        _CTIMER0W { w: self }
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> _CTIMER1W {
        _CTIMER1W { w: self }
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&mut self) -> _SCT0W {
        _SCT0W { w: self }
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> _CTIMER3W {
        _CTIMER3W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> _FLEXCOMM0W {
        _FLEXCOMM0W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> _FLEXCOMM1W {
        _FLEXCOMM1W { w: self }
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> _FLEXCOMM2W {
        _FLEXCOMM2W { w: self }
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> _FLEXCOMM3W {
        _FLEXCOMM3W { w: self }
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> _FLEXCOMM4W {
        _FLEXCOMM4W { w: self }
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> _FLEXCOMM5W {
        _FLEXCOMM5W { w: self }
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> _FLEXCOMM6W {
        _FLEXCOMM6W { w: self }
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> _FLEXCOMM7W {
        _FLEXCOMM7W { w: self }
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&mut self) -> _ADC0_SEQAW {
        _ADC0_SEQAW { w: self }
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&mut self) -> _ADC0_SEQBW {
        _ADC0_SEQBW { w: self }
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&mut self) -> _ADC0_THCMPW {
        _ADC0_THCMPW { w: self }
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&mut self) -> _DMICW {
        _DMICW { w: self }
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&mut self) -> _HWVADW {
        _HWVADW { w: self }
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> _USB0_NEEDCLKW {
        _USB0_NEEDCLKW { w: self }
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&mut self) -> _USB0W {
        _USB0W { w: self }
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&mut self) -> _RTCW {
        _RTCW { w: self }
    }
}
