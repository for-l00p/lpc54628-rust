#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IR {
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
pub type RF0N_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0NW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0NW<'a> {
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
pub type RF0W_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0WW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0WW<'a> {
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
pub type RF0F_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0FW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0FW<'a> {
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
pub type RF0L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0LW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0LW<'a> {
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
pub type RF1N_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1NW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1NW<'a> {
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
pub type RF1W_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1WW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1WW<'a> {
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
pub type RF1F_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1FW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1FW<'a> {
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
pub type RF1L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1LW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1LW<'a> {
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
pub type HPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HPMW<'a> {
    w: &'a mut W,
}
impl<'a> _HPMW<'a> {
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
pub type TC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCW<'a> {
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
pub type TCF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
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
pub type TFE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFEW<'a> {
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
pub type TEFN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFNW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFNW<'a> {
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
pub type TEFW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFWW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFWW<'a> {
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
pub type TEFF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFFW<'a> {
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
pub type TEFL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFLW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFLW<'a> {
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
pub type TSW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSWW<'a> {
    w: &'a mut W,
}
impl<'a> _TSWW<'a> {
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
pub type MRAF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MRAFW<'a> {
    w: &'a mut W,
}
impl<'a> _MRAFW<'a> {
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
pub type TOO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TOOW<'a> {
    w: &'a mut W,
}
impl<'a> _TOOW<'a> {
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
pub type DRX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRXW<'a> {
    w: &'a mut W,
}
impl<'a> _DRXW<'a> {
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
pub type BEC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BECW<'a> {
    w: &'a mut W,
}
impl<'a> _BECW<'a> {
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
pub type BEU_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BEUW<'a> {
    w: &'a mut W,
}
impl<'a> _BEUW<'a> {
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
pub type ELO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ELOW<'a> {
    w: &'a mut W,
}
impl<'a> _ELOW<'a> {
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
pub type EP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPW<'a> {
    w: &'a mut W,
}
impl<'a> _EPW<'a> {
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
pub type EW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EWW<'a> {
    w: &'a mut W,
}
impl<'a> _EWW<'a> {
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
pub type BO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BOW<'a> {
    w: &'a mut W,
}
impl<'a> _BOW<'a> {
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
pub type WDI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDIW<'a> {
    w: &'a mut W,
}
impl<'a> _WDIW<'a> {
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
pub type PEA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEAW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAW<'a> {
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
pub type PED_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PEDW<'a> {
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
pub type ARA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ARAW<'a> {
    w: &'a mut W,
}
impl<'a> _ARAW<'a> {
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
    #[doc = "Bit 0 - Rx FIFO 0 new message."]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message."]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High priority message."]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission completed."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission cancellation finished."]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry."]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached."]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx event FIFO full."]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp wraparound."]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM access failure."]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout occurred."]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit error corrected."]
    #[inline(always)]
    pub fn bec(&self) -> BEC_R {
        BEC_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit error uncorrected."]
    #[inline(always)]
    pub fn beu(&self) -> BEU_R {
        BEU_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error logging overflow."]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error passive."]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning status."]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog interrupt."]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase."]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol error in data phase."]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to reserved address."]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rx FIFO 0 new message."]
    #[inline(always)]
    pub fn rf0n(&mut self) -> _RF0NW {
        _RF0NW { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub fn rf0w(&mut self) -> _RF0WW {
        _RF0WW { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn rf0f(&mut self) -> _RF0FW {
        _RF0FW { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&mut self) -> _RF0LW {
        _RF0LW { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message."]
    #[inline(always)]
    pub fn rf1n(&mut self) -> _RF1NW {
        _RF1NW { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub fn rf1w(&mut self) -> _RF1WW {
        _RF1WW { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO 1 full."]
    #[inline(always)]
    pub fn rf1f(&mut self) -> _RF1FW {
        _RF1FW { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost."]
    #[inline(always)]
    pub fn rf1l(&mut self) -> _RF1LW {
        _RF1LW { w: self }
    }
    #[doc = "Bit 8 - High priority message."]
    #[inline(always)]
    pub fn hpm(&mut self) -> _HPMW {
        _HPMW { w: self }
    }
    #[doc = "Bit 9 - Transmission completed."]
    #[inline(always)]
    pub fn tc(&mut self) -> _TCW {
        _TCW { w: self }
    }
    #[doc = "Bit 10 - Transmission cancellation finished."]
    #[inline(always)]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
    #[doc = "Bit 11 - Tx FIFO empty."]
    #[inline(always)]
    pub fn tfe(&mut self) -> _TFEW {
        _TFEW { w: self }
    }
    #[doc = "Bit 12 - Tx event FIFO new entry."]
    #[inline(always)]
    pub fn tefn(&mut self) -> _TEFNW {
        _TEFNW { w: self }
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached."]
    #[inline(always)]
    pub fn tefw(&mut self) -> _TEFWW {
        _TEFWW { w: self }
    }
    #[doc = "Bit 14 - Tx event FIFO full."]
    #[inline(always)]
    pub fn teff(&mut self) -> _TEFFW {
        _TEFFW { w: self }
    }
    #[doc = "Bit 15 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&mut self) -> _TEFLW {
        _TEFLW { w: self }
    }
    #[doc = "Bit 16 - Timestamp wraparound."]
    #[inline(always)]
    pub fn tsw(&mut self) -> _TSWW {
        _TSWW { w: self }
    }
    #[doc = "Bit 17 - Message RAM access failure."]
    #[inline(always)]
    pub fn mraf(&mut self) -> _MRAFW {
        _MRAFW { w: self }
    }
    #[doc = "Bit 18 - Timeout occurred."]
    #[inline(always)]
    pub fn too(&mut self) -> _TOOW {
        _TOOW { w: self }
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub fn drx(&mut self) -> _DRXW {
        _DRXW { w: self }
    }
    #[doc = "Bit 20 - Bit error corrected."]
    #[inline(always)]
    pub fn bec(&mut self) -> _BECW {
        _BECW { w: self }
    }
    #[doc = "Bit 21 - Bit error uncorrected."]
    #[inline(always)]
    pub fn beu(&mut self) -> _BEUW {
        _BEUW { w: self }
    }
    #[doc = "Bit 22 - Error logging overflow."]
    #[inline(always)]
    pub fn elo(&mut self) -> _ELOW {
        _ELOW { w: self }
    }
    #[doc = "Bit 23 - Error passive."]
    #[inline(always)]
    pub fn ep(&mut self) -> _EPW {
        _EPW { w: self }
    }
    #[doc = "Bit 24 - Warning status."]
    #[inline(always)]
    pub fn ew(&mut self) -> _EWW {
        _EWW { w: self }
    }
    #[doc = "Bit 25 - Bus_Off Status."]
    #[inline(always)]
    pub fn bo(&mut self) -> _BOW {
        _BOW { w: self }
    }
    #[doc = "Bit 26 - Watchdog interrupt."]
    #[inline(always)]
    pub fn wdi(&mut self) -> _WDIW {
        _WDIW { w: self }
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase."]
    #[inline(always)]
    pub fn pea(&mut self) -> _PEAW {
        _PEAW { w: self }
    }
    #[doc = "Bit 28 - Protocol error in data phase."]
    #[inline(always)]
    pub fn ped(&mut self) -> _PEDW {
        _PEDW { w: self }
    }
    #[doc = "Bit 29 - Access to reserved address."]
    #[inline(always)]
    pub fn ara(&mut self) -> _ARAW {
        _ARAW { w: self }
    }
}
