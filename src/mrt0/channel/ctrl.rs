#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTENR::DISABLED => false,
            INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTEN_R = crate::FR<bool, INTENR>;
impl INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENW {
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    ENABLED,
}
impl INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTENW::DISABLED => false,
            INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTENW::DISABLED)
    }
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTENW::ENABLED)
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Repeat interrupt mode."]
    REPEAT_INTERRUPT_MODE,
    #[doc = "One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_MODE,
    #[doc = "One-shot stall mode."]
    ONE_SHOT_STALL_MODE,
}
impl crate::ToBits<u8> for MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODER::REPEAT_INTERRUPT_MODE => 0,
            MODER::ONE_SHOT_INTERRUPT_MODE => 1,
            MODER::ONE_SHOT_STALL_MODE => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MODE_R = crate::FR<u8, MODER>;
impl MODE_R {
    #[doc = "Checks if the value of the field is `REPEAT_INTERRUPT_MODE`"]
    #[inline(always)]
    pub fn is_repeat_interrupt_mode(&self) -> bool {
        *self == MODER::REPEAT_INTERRUPT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_INTERRUPT_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_interrupt_mode(&self) -> bool {
        *self == MODER::ONE_SHOT_INTERRUPT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_STALL_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_stall_mode(&self) -> bool {
        *self == MODER::ONE_SHOT_STALL_MODE
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "Repeat interrupt mode."]
    REPEAT_INTERRUPT_MODE,
    #[doc = "One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_MODE,
    #[doc = "One-shot stall mode."]
    ONE_SHOT_STALL_MODE,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::REPEAT_INTERRUPT_MODE => 0,
            MODEW::ONE_SHOT_INTERRUPT_MODE => 1,
            MODEW::ONE_SHOT_STALL_MODE => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Repeat interrupt mode."]
    #[inline(always)]
    pub fn repeat_interrupt_mode(self) -> &'a mut W {
        self.variant(MODEW::REPEAT_INTERRUPT_MODE)
    }
    #[doc = "One-shot interrupt mode."]
    #[inline(always)]
    pub fn one_shot_interrupt_mode(self) -> &'a mut W {
        self.variant(MODEW::ONE_SHOT_INTERRUPT_MODE)
    }
    #[doc = "One-shot stall mode."]
    #[inline(always)]
    pub fn one_shot_stall_mode(self) -> &'a mut W {
        self.variant(MODEW::ONE_SHOT_STALL_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline(always)]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
