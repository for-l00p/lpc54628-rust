#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IE {
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
pub type RF0NE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0NEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0NEW<'a> {
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
pub type RF0WE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0WEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0WEW<'a> {
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
pub type RF0FE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0FEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0FEW<'a> {
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
pub type RF0LE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0LEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0LEW<'a> {
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
pub type RF1NE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1NEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1NEW<'a> {
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
pub type RF1WE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1WEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1WEW<'a> {
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
pub type RF1FE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1FEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1FEW<'a> {
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
pub type RF1LE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF1LEW<'a> {
    w: &'a mut W,
}
impl<'a> _RF1LEW<'a> {
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
pub type HPME_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HPMEW<'a> {
    w: &'a mut W,
}
impl<'a> _HPMEW<'a> {
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
pub type TCE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCEW<'a> {
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
pub type TCFE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFEW<'a> {
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
pub type TFEE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TFEEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFEEW<'a> {
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
pub type TEFNE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFNEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFNEW<'a> {
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
pub type TEFWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFWEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFWEW<'a> {
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
pub type TEFFE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFFEW<'a> {
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
pub type TEFLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEFLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFLEW<'a> {
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
pub type TSWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSWEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSWEW<'a> {
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
pub type MRAFE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MRAFEW<'a> {
    w: &'a mut W,
}
impl<'a> _MRAFEW<'a> {
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
pub type TOOE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TOOEW<'a> {
    w: &'a mut W,
}
impl<'a> _TOOEW<'a> {
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
pub type DRXE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRXEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRXEW<'a> {
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
pub type BECE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BECEW<'a> {
    w: &'a mut W,
}
impl<'a> _BECEW<'a> {
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
pub type BEUE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BEUEW<'a> {
    w: &'a mut W,
}
impl<'a> _BEUEW<'a> {
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
pub type ELOE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ELOEW<'a> {
    w: &'a mut W,
}
impl<'a> _ELOEW<'a> {
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
pub type EPE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPEW<'a> {
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
pub type EWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EWEW<'a> {
    w: &'a mut W,
}
impl<'a> _EWEW<'a> {
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
pub type BOE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BOEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOEW<'a> {
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
pub type WDIE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WDIEW<'a> {
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
pub type PEAE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEAEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAEW<'a> {
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
pub type PEDE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEDEW<'a> {
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
pub type ARAE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ARAEW<'a> {
    w: &'a mut W,
}
impl<'a> _ARAEW<'a> {
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
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable."]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full interrupt enable."]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message interrupt enable."]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full interrupt enable."]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High priority message interrupt enable."]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission completed interrupt enable."]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission cancellation finished interrupt enable."]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry interrupt enable."]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached interrupt enable."]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx event FIFO full interrupt enable."]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost interrupt enable."]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp wraparound interrupt enable."]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM access failure interrupt enable."]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout occurred interrupt enable."]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer interrupt enable."]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit error corrected interrupt enable."]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit error uncorrected interrupt enable."]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error logging overflow interrupt enable."]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error passive interrupt enable."]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning status interrupt enable."]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status interrupt enable."]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog interrupt enable."]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase interrupt enable."]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol error in data phase interrupt enable."]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to reserved address interrupt enable."]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable."]
    #[inline(always)]
    pub fn rf0ne(&mut self) -> _RF0NEW {
        _RF0NEW { w: self }
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf0we(&mut self) -> _RF0WEW {
        _RF0WEW { w: self }
    }
    #[doc = "Bit 2 - Rx FIFO 0 full interrupt enable."]
    #[inline(always)]
    pub fn rf0fe(&mut self) -> _RF0FEW {
        _RF0FEW { w: self }
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf0le(&mut self) -> _RF0LEW {
        _RF0LEW { w: self }
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message interrupt enable."]
    #[inline(always)]
    pub fn rf1ne(&mut self) -> _RF1NEW {
        _RF1NEW { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf1we(&mut self) -> _RF1WEW {
        _RF1WEW { w: self }
    }
    #[doc = "Bit 6 - Rx FIFO 1 full interrupt enable."]
    #[inline(always)]
    pub fn rf1fe(&mut self) -> _RF1FEW {
        _RF1FEW { w: self }
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf1le(&mut self) -> _RF1LEW {
        _RF1LEW { w: self }
    }
    #[doc = "Bit 8 - High priority message interrupt enable."]
    #[inline(always)]
    pub fn hpme(&mut self) -> _HPMEW {
        _HPMEW { w: self }
    }
    #[doc = "Bit 9 - Transmission completed interrupt enable."]
    #[inline(always)]
    pub fn tce(&mut self) -> _TCEW {
        _TCEW { w: self }
    }
    #[doc = "Bit 10 - Transmission cancellation finished interrupt enable."]
    #[inline(always)]
    pub fn tcfe(&mut self) -> _TCFEW {
        _TCFEW { w: self }
    }
    #[doc = "Bit 11 - Tx FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn tfee(&mut self) -> _TFEEW {
        _TFEEW { w: self }
    }
    #[doc = "Bit 12 - Tx event FIFO new entry interrupt enable."]
    #[inline(always)]
    pub fn tefne(&mut self) -> _TEFNEW {
        _TEFNEW { w: self }
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached interrupt enable."]
    #[inline(always)]
    pub fn tefwe(&mut self) -> _TEFWEW {
        _TEFWEW { w: self }
    }
    #[doc = "Bit 14 - Tx event FIFO full interrupt enable."]
    #[inline(always)]
    pub fn teffe(&mut self) -> _TEFFEW {
        _TEFFEW { w: self }
    }
    #[doc = "Bit 15 - Tx event FIFO element lost interrupt enable."]
    #[inline(always)]
    pub fn tefle(&mut self) -> _TEFLEW {
        _TEFLEW { w: self }
    }
    #[doc = "Bit 16 - Timestamp wraparound interrupt enable."]
    #[inline(always)]
    pub fn tswe(&mut self) -> _TSWEW {
        _TSWEW { w: self }
    }
    #[doc = "Bit 17 - Message RAM access failure interrupt enable."]
    #[inline(always)]
    pub fn mrafe(&mut self) -> _MRAFEW {
        _MRAFEW { w: self }
    }
    #[doc = "Bit 18 - Timeout occurred interrupt enable."]
    #[inline(always)]
    pub fn tooe(&mut self) -> _TOOEW {
        _TOOEW { w: self }
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer interrupt enable."]
    #[inline(always)]
    pub fn drxe(&mut self) -> _DRXEW {
        _DRXEW { w: self }
    }
    #[doc = "Bit 20 - Bit error corrected interrupt enable."]
    #[inline(always)]
    pub fn bece(&mut self) -> _BECEW {
        _BECEW { w: self }
    }
    #[doc = "Bit 21 - Bit error uncorrected interrupt enable."]
    #[inline(always)]
    pub fn beue(&mut self) -> _BEUEW {
        _BEUEW { w: self }
    }
    #[doc = "Bit 22 - Error logging overflow interrupt enable."]
    #[inline(always)]
    pub fn eloe(&mut self) -> _ELOEW {
        _ELOEW { w: self }
    }
    #[doc = "Bit 23 - Error passive interrupt enable."]
    #[inline(always)]
    pub fn epe(&mut self) -> _EPEW {
        _EPEW { w: self }
    }
    #[doc = "Bit 24 - Warning status interrupt enable."]
    #[inline(always)]
    pub fn ewe(&mut self) -> _EWEW {
        _EWEW { w: self }
    }
    #[doc = "Bit 25 - Bus_Off Status interrupt enable."]
    #[inline(always)]
    pub fn boe(&mut self) -> _BOEW {
        _BOEW { w: self }
    }
    #[doc = "Bit 26 - Watchdog interrupt enable."]
    #[inline(always)]
    pub fn wdie(&mut self) -> _WDIEW {
        _WDIEW { w: self }
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase interrupt enable."]
    #[inline(always)]
    pub fn peae(&mut self) -> _PEAEW {
        _PEAEW { w: self }
    }
    #[doc = "Bit 28 - Protocol error in data phase interrupt enable."]
    #[inline(always)]
    pub fn pede(&mut self) -> _PEDEW {
        _PEDEW { w: self }
    }
    #[doc = "Bit 29 - Access to reserved address interrupt enable."]
    #[inline(always)]
    pub fn arae(&mut self) -> _ARAEW {
        _ARAEW { w: self }
    }
}
