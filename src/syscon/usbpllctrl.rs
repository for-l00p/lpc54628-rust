#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBPLLCTRL {
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
pub type MSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `DIRECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTR {
    #[doc = "CCO Clock signal goes through post divider."]
    DISABLED,
    #[doc = "CCO Clock signal goes directly to output(s).."]
    ENABLED,
}
impl crate::ToBits<bool> for DIRECTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIRECTR::DISABLED => false,
            DIRECTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIRECT_R = crate::FR<bool, DIRECTR>;
impl DIRECT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIRECTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIRECTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `DIRECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTW {
    #[doc = "CCO Clock signal goes through post divider."]
    DISABLED,
    #[doc = "CCO Clock signal goes directly to output(s).."]
    ENABLED,
}
impl DIRECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRECTW::DISABLED => false,
            DIRECTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIRECTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCO Clock signal goes through post divider."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECTW::DISABLED)
    }
    #[doc = "CCO Clock signal goes directly to output(s).."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECTW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "CCO clock is sent to post dividers.."]
    DISABLED,
    #[doc = "PLL input clock is sent to post dividers.."]
    ENABLED,
}
impl crate::ToBits<bool> for BYPASSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BYPASSR::DISABLED => false,
            BYPASSR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BYPASS_R = crate::FR<bool, BYPASSR>;
impl BYPASS_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASSR::ENABLED
    }
}
#[doc = "Values that can be written to the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSW {
    #[doc = "CCO clock is sent to post dividers.."]
    DISABLED,
    #[doc = "PLL input clock is sent to post dividers.."]
    ENABLED,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::DISABLED => false,
            BYPASSW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCO clock is sent to post dividers.."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASSW::DISABLED)
    }
    #[doc = "PLL input clock is sent to post dividers.."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASSW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FBSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FBSELW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - PLL Divider value."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn nsel(&self) -> NSEL_R {
        NSEL_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Direct CCO clock output control."]
    #[inline(always)]
    pub fn direct(&self) -> DIRECT_R {
        DIRECT_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input clock bypass control."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Feedback divider input clock control."]
    #[inline(always)]
    pub fn fbsel(&self) -> FBSEL_R {
        FBSEL_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 8:9 - PLL Divider value."]
    #[inline(always)]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bits 10:11 - PLL feedback Divider value."]
    #[inline(always)]
    pub fn nsel(&mut self) -> _NSELW {
        _NSELW { w: self }
    }
    #[doc = "Bit 12 - Direct CCO clock output control."]
    #[inline(always)]
    pub fn direct(&mut self) -> _DIRECTW {
        _DIRECTW { w: self }
    }
    #[doc = "Bit 13 - Input clock bypass control."]
    #[inline(always)]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 14 - Feedback divider input clock control."]
    #[inline(always)]
    pub fn fbsel(&mut self) -> _FBSELW {
        _FBSELW { w: self }
    }
}
