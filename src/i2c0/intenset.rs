#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `MSTPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGENR {
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MSTPENDINGENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGENR::DISABLED => false,
            MSTPENDINGENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTPENDINGEN_R = crate::FR<bool, MSTPENDINGENR>;
impl MSTPENDINGEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTPENDINGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTPENDINGENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGENW {
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    ENABLED,
}
impl MSTPENDINGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGENW::DISABLED => false,
            MSTPENDINGENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTPENDINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTPENDINGENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTPENDINGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTPENDINGENW::DISABLED)
    }
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTPENDINGENW::ENABLED)
    }
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
#[doc = "Possible values of the field `MSTARBLOSSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSENR {
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MSTARBLOSSENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSENR::DISABLED => false,
            MSTARBLOSSENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTARBLOSSEN_R = crate::FR<bool, MSTARBLOSSENR>;
impl MSTARBLOSSEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTARBLOSSENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTARBLOSSENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTARBLOSSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSENW {
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    ENABLED,
}
impl MSTARBLOSSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSENW::DISABLED => false,
            MSTARBLOSSENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTARBLOSSENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTARBLOSSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSENW::DISABLED)
    }
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSENW::ENABLED)
    }
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
#[doc = "Possible values of the field `MSTSTSTPERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRENR {
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MSTSTSTPERRENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRENR::DISABLED => false,
            MSTSTSTPERRENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTSTSTPERREN_R = crate::FR<bool, MSTSTSTPERRENR>;
impl MSTSTSTPERREN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTSTSTPERRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTSTSTPERRENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTSTSTPERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRENW {
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    ENABLED,
}
impl MSTSTSTPERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRENW::DISABLED => false,
            MSTSTSTPERRENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTSTSTPERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTSTPERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERRENW::DISABLED)
    }
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERRENW::ENABLED)
    }
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
#[doc = "Possible values of the field `SLVPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGENR {
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for SLVPENDINGENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGENR::DISABLED => false,
            SLVPENDINGENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVPENDINGEN_R = crate::FR<bool, SLVPENDINGENR>;
impl SLVPENDINGEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVPENDINGENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVPENDINGENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SLVPENDINGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGENW {
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    ENABLED,
}
impl SLVPENDINGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGENW::DISABLED => false,
            SLVPENDINGENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVPENDINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVPENDINGENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVPENDINGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVPENDINGENW::DISABLED)
    }
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVPENDINGENW::ENABLED)
    }
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
#[doc = "Possible values of the field `SLVNOTSTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRENR {
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for SLVNOTSTRENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRENR::DISABLED => false,
            SLVNOTSTRENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVNOTSTREN_R = crate::FR<bool, SLVNOTSTRENR>;
impl SLVNOTSTREN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVNOTSTRENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVNOTSTRENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SLVNOTSTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRENW {
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    ENABLED,
}
impl SLVNOTSTRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRENW::DISABLED => false,
            SLVNOTSTRENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVNOTSTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNOTSTRENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVNOTSTRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVNOTSTRENW::DISABLED)
    }
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVNOTSTRENW::ENABLED)
    }
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
#[doc = "Possible values of the field `SLVDESELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELENR {
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for SLVDESELENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVDESELENR::DISABLED => false,
            SLVDESELENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVDESELEN_R = crate::FR<bool, SLVDESELENR>;
impl SLVDESELEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDESELENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDESELENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SLVDESELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELENW {
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    ENABLED,
}
impl SLVDESELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDESELENW::DISABLED => false,
            SLVDESELENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVDESELENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDESELENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDESELENW::DISABLED)
    }
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDESELENW::ENABLED)
    }
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
#[doc = "Possible values of the field `MONRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYENR {
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MONRDYENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONRDYENR::DISABLED => false,
            MONRDYENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONRDYEN_R = crate::FR<bool, MONRDYENR>;
impl MONRDYEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONRDYENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONRDYENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MONRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYENW {
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    ENABLED,
}
impl MONRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYENW::DISABLED => false,
            MONRDYENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONRDYENW::DISABLED)
    }
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONRDYENW::ENABLED)
    }
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
#[doc = "Possible values of the field `MONOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVENR {
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MONOVENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONOVENR::DISABLED => false,
            MONOVENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONOVEN_R = crate::FR<bool, MONOVENR>;
impl MONOVEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONOVENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONOVENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MONOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVENW {
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    ENABLED,
}
impl MONOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOVENW::DISABLED => false,
            MONOVENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONOVENW::DISABLED)
    }
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONOVENW::ENABLED)
    }
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
#[doc = "Possible values of the field `MONIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEENR {
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MONIDLEENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONIDLEENR::DISABLED => false,
            MONIDLEENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONIDLEEN_R = crate::FR<bool, MONIDLEENR>;
impl MONIDLEEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONIDLEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONIDLEENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MONIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEENW {
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    ENABLED,
}
impl MONIDLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONIDLEENW::DISABLED => false,
            MONIDLEENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLEENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONIDLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONIDLEENW::DISABLED)
    }
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONIDLEENW::ENABLED)
    }
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
#[doc = "Possible values of the field `EVENTTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTENR {
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for EVENTTIMEOUTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTENR::DISABLED => false,
            EVENTTIMEOUTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EVENTTIMEOUTEN_R = crate::FR<bool, EVENTTIMEOUTENR>;
impl EVENTTIMEOUTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EVENTTIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EVENTTIMEOUTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `EVENTTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTENW {
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    ENABLED,
}
impl EVENTTIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTENW::DISABLED => false,
            EVENTTIMEOUTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EVENTTIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTTIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTENW::DISABLED)
    }
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTENW::ENABLED)
    }
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
#[doc = "Possible values of the field `SCLTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTENR {
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for SCLTIMEOUTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTENR::DISABLED => false,
            SCLTIMEOUTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SCLTIMEOUTEN_R = crate::FR<bool, SCLTIMEOUTENR>;
impl SCLTIMEOUTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCLTIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCLTIMEOUTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SCLTIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTENW {
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    ENABLED,
}
impl SCLTIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTENW::DISABLED => false,
            SCLTIMEOUTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SCLTIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLTIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTENW::DISABLED)
    }
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTENW::ENABLED)
    }
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn mstpendingen(&self) -> MSTPENDINGEN_R {
        MSTPENDINGEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn mstarblossen(&self) -> MSTARBLOSSEN_R {
        MSTARBLOSSEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn mstststperren(&self) -> MSTSTSTPERREN_R {
        MSTSTSTPERREN_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn slvpendingen(&self) -> SLVPENDINGEN_R {
        SLVPENDINGEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn slvnotstren(&self) -> SLVNOTSTREN_R {
        SLVNOTSTREN_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn slvdeselen(&self) -> SLVDESELEN_R {
        SLVDESELEN_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn monrdyen(&self) -> MONRDYEN_R {
        MONRDYEN_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn monoven(&self) -> MONOVEN_R {
        MONOVEN_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn monidleen(&self) -> MONIDLEEN_R {
        MONIDLEEN_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn eventtimeouten(&self) -> EVENTTIMEOUTEN_R {
        EVENTTIMEOUTEN_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn scltimeouten(&self) -> SCLTIMEOUTEN_R {
        SCLTIMEOUTEN_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn mstpendingen(&mut self) -> _MSTPENDINGENW {
        _MSTPENDINGENW { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn mstarblossen(&mut self) -> _MSTARBLOSSENW {
        _MSTARBLOSSENW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn mstststperren(&mut self) -> _MSTSTSTPERRENW {
        _MSTSTSTPERRENW { w: self }
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn slvpendingen(&mut self) -> _SLVPENDINGENW {
        _SLVPENDINGENW { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn slvnotstren(&mut self) -> _SLVNOTSTRENW {
        _SLVNOTSTRENW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn slvdeselen(&mut self) -> _SLVDESELENW {
        _SLVDESELENW { w: self }
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn monrdyen(&mut self) -> _MONRDYENW {
        _MONRDYENW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn monoven(&mut self) -> _MONOVENW {
        _MONOVENW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn monidleen(&mut self) -> _MONIDLEENW {
        _MONIDLEENW { w: self }
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn eventtimeouten(&mut self) -> _EVENTTIMEOUTENW {
        _EVENTTIMEOUTENW { w: self }
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn scltimeouten(&mut self) -> _SCLTIMEOUTENW {
        _SCLTIMEOUTENW { w: self }
    }
}
