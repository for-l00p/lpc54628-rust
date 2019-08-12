#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSPLLCTRL {
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
pub type SELR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SELRW<'a> {
    w: &'a mut W,
}
impl<'a> _SELRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SELI_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SELIW<'a> {
    w: &'a mut W,
}
impl<'a> _SELIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SELP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SELPW<'a> {
    w: &'a mut W,
}
impl<'a> _SELPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    DISABLED,
    #[doc = "Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
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
    #[doc = "Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    DISABLED,
    #[doc = "Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
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
    #[doc = "Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASSW::DISABLED)
    }
    #[doc = "Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type UPLIMOFF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UPLIMOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _UPLIMOFFW<'a> {
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
pub type DIRECTI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DIRECTIW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTIW<'a> {
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
#[doc = "Possible values of the field `DIRECTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTOR {
    #[doc = "Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    DISABLED,
    #[doc = "Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    ENABLED,
}
impl crate::ToBits<bool> for DIRECTOR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIRECTOR::DISABLED => false,
            DIRECTOR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIRECTO_R = crate::FR<bool, DIRECTOR>;
impl DIRECTO_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIRECTOR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIRECTOR::ENABLED
    }
}
#[doc = "Values that can be written to the field `DIRECTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTOW {
    #[doc = "Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    DISABLED,
    #[doc = "Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    ENABLED,
}
impl DIRECTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRECTOW::DISABLED => false,
            DIRECTOW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIRECTOW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRECTOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECTOW::DISABLED)
    }
    #[doc = "Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECTOW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&self) -> SELR_R {
        SELR_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&self) -> SELI_R {
        SELI_R::new(((self.bits() >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&self) -> SELP_R {
        SELP_R::new(((self.bits() >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - PLL bypass control."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable upper frequency limiter."]
    #[inline(always)]
    pub fn uplimoff(&self) -> UPLIMOFF_R {
        UPLIMOFF_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PLL0 direct input enable."]
    #[inline(always)]
    pub fn directi(&self) -> DIRECTI_R {
        DIRECTI_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PLL0 direct output enable."]
    #[inline(always)]
    pub fn directo(&self) -> DIRECTO_R {
        DIRECTO_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&mut self) -> _SELRW {
        _SELRW { w: self }
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&mut self) -> _SELIW {
        _SELIW { w: self }
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&mut self) -> _SELPW {
        _SELPW { w: self }
    }
    #[doc = "Bit 15 - PLL bypass control."]
    #[inline(always)]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 17 - Disable upper frequency limiter."]
    #[inline(always)]
    pub fn uplimoff(&mut self) -> _UPLIMOFFW {
        _UPLIMOFFW { w: self }
    }
    #[doc = "Bit 19 - PLL0 direct input enable."]
    #[inline(always)]
    pub fn directi(&mut self) -> _DIRECTIW {
        _DIRECTIW { w: self }
    }
    #[doc = "Bit 20 - PLL0 direct output enable."]
    #[inline(always)]
    pub fn directo(&mut self) -> _DIRECTOW {
        _DIRECTOW { w: self }
    }
}
