#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FPDSCR {
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
#[doc = "Possible values of the field `RMode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMODER {
    #[doc = "Round to Nearest (RN) mode"]
    RMODE_0,
    #[doc = "Round towards Plus Infinity (RP) mode."]
    RMODE_1,
    #[doc = "Round towards Minus Infinity (RM) mode."]
    RMODE_2,
    #[doc = "Round towards Zero (RZ) mode."]
    RMODE_3,
}
impl crate::ToBits<u8> for RMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RMODER::RMODE_0 => 0,
            RMODER::RMODE_1 => 1,
            RMODER::RMODE_2 => 2,
            RMODER::RMODE_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RMODE_R = crate::FR<u8, RMODER>;
impl RMODE_R {
    #[doc = "Checks if the value of the field is `RMODE_0`"]
    #[inline(always)]
    pub fn is_rmode_0(&self) -> bool {
        *self == RMODER::RMODE_0
    }
    #[doc = "Checks if the value of the field is `RMODE_1`"]
    #[inline(always)]
    pub fn is_rmode_1(&self) -> bool {
        *self == RMODER::RMODE_1
    }
    #[doc = "Checks if the value of the field is `RMODE_2`"]
    #[inline(always)]
    pub fn is_rmode_2(&self) -> bool {
        *self == RMODER::RMODE_2
    }
    #[doc = "Checks if the value of the field is `RMODE_3`"]
    #[inline(always)]
    pub fn is_rmode_3(&self) -> bool {
        *self == RMODER::RMODE_3
    }
}
#[doc = "Values that can be written to the field `RMode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMODEW {
    #[doc = "Round to Nearest (RN) mode"]
    RMODE_0,
    #[doc = "Round towards Plus Infinity (RP) mode."]
    RMODE_1,
    #[doc = "Round towards Minus Infinity (RM) mode."]
    RMODE_2,
    #[doc = "Round towards Zero (RZ) mode."]
    RMODE_3,
}
impl RMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RMODEW::RMODE_0 => 0,
            RMODEW::RMODE_1 => 1,
            RMODEW::RMODE_2 => 2,
            RMODEW::RMODE_3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Round to Nearest (RN) mode"]
    #[inline(always)]
    pub fn rmode_0(self) -> &'a mut W {
        self.variant(RMODEW::RMODE_0)
    }
    #[doc = "Round towards Plus Infinity (RP) mode."]
    #[inline(always)]
    pub fn rmode_1(self) -> &'a mut W {
        self.variant(RMODEW::RMODE_1)
    }
    #[doc = "Round towards Minus Infinity (RM) mode."]
    #[inline(always)]
    pub fn rmode_2(self) -> &'a mut W {
        self.variant(RMODEW::RMODE_2)
    }
    #[doc = "Round towards Zero (RZ) mode."]
    #[inline(always)]
    pub fn rmode_3(self) -> &'a mut W {
        self.variant(RMODEW::RMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `FZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FZR {
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    FZ_0,
    #[doc = "Flush-to-zero mode enabled."]
    FZ_1,
}
impl crate::ToBits<bool> for FZR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FZR::FZ_0 => false,
            FZR::FZ_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FZ_R = crate::FR<bool, FZR>;
impl FZ_R {
    #[doc = "Checks if the value of the field is `FZ_0`"]
    #[inline(always)]
    pub fn is_fz_0(&self) -> bool {
        *self == FZR::FZ_0
    }
    #[doc = "Checks if the value of the field is `FZ_1`"]
    #[inline(always)]
    pub fn is_fz_1(&self) -> bool {
        *self == FZR::FZ_1
    }
}
#[doc = "Values that can be written to the field `FZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FZW {
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    FZ_0,
    #[doc = "Flush-to-zero mode enabled."]
    FZ_1,
}
impl FZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FZW::FZ_0 => false,
            FZW::FZ_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FZW<'a> {
    w: &'a mut W,
}
impl<'a> _FZW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    #[inline(always)]
    pub fn fz_0(self) -> &'a mut W {
        self.variant(FZW::FZ_0)
    }
    #[doc = "Flush-to-zero mode enabled."]
    #[inline(always)]
    pub fn fz_1(self) -> &'a mut W {
        self.variant(FZW::FZ_1)
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
#[doc = "Possible values of the field `DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNR {
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    DN_0,
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    DN_1,
}
impl crate::ToBits<bool> for DNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DNR::DN_0 => false,
            DNR::DN_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DN_R = crate::FR<bool, DNR>;
impl DN_R {
    #[doc = "Checks if the value of the field is `DN_0`"]
    #[inline(always)]
    pub fn is_dn_0(&self) -> bool {
        *self == DNR::DN_0
    }
    #[doc = "Checks if the value of the field is `DN_1`"]
    #[inline(always)]
    pub fn is_dn_1(&self) -> bool {
        *self == DNR::DN_1
    }
}
#[doc = "Values that can be written to the field `DN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNW {
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    DN_0,
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    DN_1,
}
impl DNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DNW::DN_0 => false,
            DNW::DN_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DNW<'a> {
    w: &'a mut W,
}
impl<'a> _DNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    #[inline(always)]
    pub fn dn_0(self) -> &'a mut W {
        self.variant(DNW::DN_0)
    }
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    #[inline(always)]
    pub fn dn_1(self) -> &'a mut W {
        self.variant(DNW::DN_1)
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
#[doc = "Possible values of the field `AHP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHPR {
    #[doc = "IEEE half-precision format selected."]
    AHP_0,
    #[doc = "Alternative half-precision format selected."]
    AHP_1,
}
impl crate::ToBits<bool> for AHPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AHPR::AHP_0 => false,
            AHPR::AHP_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AHP_R = crate::FR<bool, AHPR>;
impl AHP_R {
    #[doc = "Checks if the value of the field is `AHP_0`"]
    #[inline(always)]
    pub fn is_ahp_0(&self) -> bool {
        *self == AHPR::AHP_0
    }
    #[doc = "Checks if the value of the field is `AHP_1`"]
    #[inline(always)]
    pub fn is_ahp_1(&self) -> bool {
        *self == AHPR::AHP_1
    }
}
#[doc = "Values that can be written to the field `AHP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHPW {
    #[doc = "IEEE half-precision format selected."]
    AHP_0,
    #[doc = "Alternative half-precision format selected."]
    AHP_1,
}
impl AHPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AHPW::AHP_0 => false,
            AHPW::AHP_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AHPW<'a> {
    w: &'a mut W,
}
impl<'a> _AHPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IEEE half-precision format selected."]
    #[inline(always)]
    pub fn ahp_0(self) -> &'a mut W {
        self.variant(AHPW::AHP_0)
    }
    #[doc = "Alternative half-precision format selected."]
    #[inline(always)]
    pub fn ahp_1(self) -> &'a mut W {
        self.variant(AHPW::AHP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline(always)]
    pub fn rmode(&mut self) -> _RMODEW {
        _RMODEW { w: self }
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline(always)]
    pub fn fz(&mut self) -> _FZW {
        _FZW { w: self }
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline(always)]
    pub fn dn(&mut self) -> _DNW {
        _DNW { w: self }
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline(always)]
    pub fn ahp(&mut self) -> _AHPW {
        _AHPW { w: self }
    }
}
