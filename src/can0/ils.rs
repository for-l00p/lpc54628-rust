#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ILS {
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
pub type RF0NL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0NLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0NLW<'a> {
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
pub type RF0WL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0WLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0WLW<'a> {
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
pub type RF0FL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0FLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0FLW<'a> {
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
pub type RF0LL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0LLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0LLW<'a> {
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
pub type RF1NL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1NLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1NLW<'a> {
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
pub type RF1WL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1WLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1WLW<'a> {
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
pub type RF1FL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1FLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1FLW<'a> {
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
pub type RF1LL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1LLW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1LLW<'a> {
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
pub type HPML_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HPMLW<'a> {
    w: &'a mut W,
}
impl<'a> _HPMLW<'a> {
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
pub type TCL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCLW<'a> {
    w: &'a mut W,
}
impl<'a> _TCLW<'a> {
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
pub type TCFL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCFLW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFLW<'a> {
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
pub type TFEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TFELW<'a> {
    w: &'a mut W,
}
impl<'a> _TFELW<'a> {
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
pub type TEFNL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFNLW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFNLW<'a> {
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
pub type TEFWL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFWLW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFWLW<'a> {
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
pub type TEFFL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFFLW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFFLW<'a> {
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
pub type TEFLL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFLLW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFLLW<'a> {
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
pub type TSWL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSWLW<'a> {
    w: &'a mut W,
}
impl<'a> _TSWLW<'a> {
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
pub type MRAFL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MRAFLW<'a> {
    w: &'a mut W,
}
impl<'a> _MRAFLW<'a> {
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
pub type TOOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TOOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TOOLW<'a> {
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
pub type DRXL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRXLW<'a> {
    w: &'a mut W,
}
impl<'a> _DRXLW<'a> {
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
pub type BECL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BECLW<'a> {
    w: &'a mut W,
}
impl<'a> _BECLW<'a> {
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
pub type BEUL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BEULW<'a> {
    w: &'a mut W,
}
impl<'a> _BEULW<'a> {
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
pub type ELOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ELOLW<'a> {
    w: &'a mut W,
}
impl<'a> _ELOLW<'a> {
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
pub type EPL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPLW<'a> {
    w: &'a mut W,
}
impl<'a> _EPLW<'a> {
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
pub type EWL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EWLW<'a> {
    w: &'a mut W,
}
impl<'a> _EWLW<'a> {
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
pub type BOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BOLW<'a> {
    w: &'a mut W,
}
impl<'a> _BOLW<'a> {
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
pub type WDIL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDILW<'a> {
    w: &'a mut W,
}
impl<'a> _WDILW<'a> {
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
pub type PEAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEALW<'a> {
    w: &'a mut W,
}
impl<'a> _PEALW<'a> {
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
pub type PEDL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEDLW<'a> {
    w: &'a mut W,
}
impl<'a> _PEDLW<'a> {
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
pub type ARAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ARALW<'a> {
    w: &'a mut W,
}
impl<'a> _ARALW<'a> {
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
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt line."]
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached interrupt line."]
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full interrupt line."]
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost interrupt line."]
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message interrupt line."]
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached interrupt line."]
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full interrupt line."]
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost interrupt line."]
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High priority message interrupt line."]
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission completed interrupt line."]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission cancellation finished interrupt line."]
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO empty interrupt line."]
    #[inline(always)]
    pub fn tfel(&self) -> TFEL_R {
        TFEL_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry interrupt line."]
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached interrupt line."]
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx event FIFO full interrupt line."]
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost interrupt line."]
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp wraparound interrupt line."]
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM access failure interrupt line."]
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout occurred interrupt line."]
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer interrupt line."]
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit error corrected interrupt line."]
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit error uncorrected interrupt line."]
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error logging overflow interrupt line."]
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error passive interrupt line."]
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning status interrupt line."]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status interrupt line."]
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog interrupt line."]
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase interrupt line."]
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol error in data phase interrupt line."]
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to reserved address interrupt line."]
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt line."]
    #[inline(always)]
    pub fn rf0nl(&mut self) -> _RF0NLW {
        _RF0NLW { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached interrupt line."]
    #[inline(always)]
    pub fn rf0wl(&mut self) -> _RF0WLW {
        _RF0WLW { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO 0 full interrupt line."]
    #[inline(always)]
    pub fn rf0fl(&mut self) -> _RF0FLW {
        _RF0FLW { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost interrupt line."]
    #[inline(always)]
    pub fn rf0ll(&mut self) -> _RF0LLW {
        _RF0LLW { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message interrupt line."]
    #[inline(always)]
    pub fn rf1nl(&mut self) -> _RF1NLW {
        _RF1NLW { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached interrupt line."]
    #[inline(always)]
    pub fn rf1wl(&mut self) -> _RF1WLW {
        _RF1WLW { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO 1 full interrupt line."]
    #[inline(always)]
    pub fn rf1fl(&mut self) -> _RF1FLW {
        _RF1FLW { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost interrupt line."]
    #[inline(always)]
    pub fn rf1ll(&mut self) -> _RF1LLW {
        _RF1LLW { w: self }
    }
    #[doc = "Bit 8 - High priority message interrupt line."]
    #[inline(always)]
    pub fn hpml(&mut self) -> _HPMLW {
        _HPMLW { w: self }
    }
    #[doc = "Bit 9 - Transmission completed interrupt line."]
    #[inline(always)]
    pub fn tcl(&mut self) -> _TCLW {
        _TCLW { w: self }
    }
    #[doc = "Bit 10 - Transmission cancellation finished interrupt line."]
    #[inline(always)]
    pub fn tcfl(&mut self) -> _TCFLW {
        _TCFLW { w: self }
    }
    #[doc = "Bit 11 - Tx FIFO empty interrupt line."]
    #[inline(always)]
    pub fn tfel(&mut self) -> _TFELW {
        _TFELW { w: self }
    }
    #[doc = "Bit 12 - Tx event FIFO new entry interrupt line."]
    #[inline(always)]
    pub fn tefnl(&mut self) -> _TEFNLW {
        _TEFNLW { w: self }
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached interrupt line."]
    #[inline(always)]
    pub fn tefwl(&mut self) -> _TEFWLW {
        _TEFWLW { w: self }
    }
    #[doc = "Bit 14 - Tx event FIFO full interrupt line."]
    #[inline(always)]
    pub fn teffl(&mut self) -> _TEFFLW {
        _TEFFLW { w: self }
    }
    #[doc = "Bit 15 - Tx event FIFO element lost interrupt line."]
    #[inline(always)]
    pub fn tefll(&mut self) -> _TEFLLW {
        _TEFLLW { w: self }
    }
    #[doc = "Bit 16 - Timestamp wraparound interrupt line."]
    #[inline(always)]
    pub fn tswl(&mut self) -> _TSWLW {
        _TSWLW { w: self }
    }
    #[doc = "Bit 17 - Message RAM access failure interrupt line."]
    #[inline(always)]
    pub fn mrafl(&mut self) -> _MRAFLW {
        _MRAFLW { w: self }
    }
    #[doc = "Bit 18 - Timeout occurred interrupt line."]
    #[inline(always)]
    pub fn tool(&mut self) -> _TOOLW {
        _TOOLW { w: self }
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer interrupt line."]
    #[inline(always)]
    pub fn drxl(&mut self) -> _DRXLW {
        _DRXLW { w: self }
    }
    #[doc = "Bit 20 - Bit error corrected interrupt line."]
    #[inline(always)]
    pub fn becl(&mut self) -> _BECLW {
        _BECLW { w: self }
    }
    #[doc = "Bit 21 - Bit error uncorrected interrupt line."]
    #[inline(always)]
    pub fn beul(&mut self) -> _BEULW {
        _BEULW { w: self }
    }
    #[doc = "Bit 22 - Error logging overflow interrupt line."]
    #[inline(always)]
    pub fn elol(&mut self) -> _ELOLW {
        _ELOLW { w: self }
    }
    #[doc = "Bit 23 - Error passive interrupt line."]
    #[inline(always)]
    pub fn epl(&mut self) -> _EPLW {
        _EPLW { w: self }
    }
    #[doc = "Bit 24 - Warning status interrupt line."]
    #[inline(always)]
    pub fn ewl(&mut self) -> _EWLW {
        _EWLW { w: self }
    }
    #[doc = "Bit 25 - Bus_Off Status interrupt line."]
    #[inline(always)]
    pub fn bol(&mut self) -> _BOLW {
        _BOLW { w: self }
    }
    #[doc = "Bit 26 - Watchdog interrupt line."]
    #[inline(always)]
    pub fn wdil(&mut self) -> _WDILW {
        _WDILW { w: self }
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase interrupt line."]
    #[inline(always)]
    pub fn peal(&mut self) -> _PEALW {
        _PEALW { w: self }
    }
    #[doc = "Bit 28 - Protocol error in data phase interrupt line."]
    #[inline(always)]
    pub fn pedl(&mut self) -> _PEDLW {
        _PEDLW { w: self }
    }
    #[doc = "Bit 29 - Access to reserved address interrupt line."]
    #[inline(always)]
    pub fn aral(&mut self) -> _ARALW {
        _ARALW { w: self }
    }
}
