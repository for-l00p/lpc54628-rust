#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Possible values of the field `SLEEPONEXIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXITR {
    #[doc = "o not sleep when returning to Thread mode"]
    SLEEPONEXIT_0,
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    SLEEPONEXIT_1,
}
impl crate::ToBits<bool> for SLEEPONEXITR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLEEPONEXITR::SLEEPONEXIT_0 => false,
            SLEEPONEXITR::SLEEPONEXIT_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLEEPONEXIT_R = crate::FR<bool, SLEEPONEXITR>;
impl SLEEPONEXIT_R {
    #[doc = "Checks if the value of the field is `SLEEPONEXIT_0`"]
    #[inline(always)]
    pub fn is_sleeponexit_0(&self) -> bool {
        *self == SLEEPONEXITR::SLEEPONEXIT_0
    }
    #[doc = "Checks if the value of the field is `SLEEPONEXIT_1`"]
    #[inline(always)]
    pub fn is_sleeponexit_1(&self) -> bool {
        *self == SLEEPONEXITR::SLEEPONEXIT_1
    }
}
#[doc = "Values that can be written to the field `SLEEPONEXIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXITW {
    #[doc = "o not sleep when returning to Thread mode"]
    SLEEPONEXIT_0,
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    SLEEPONEXIT_1,
}
impl SLEEPONEXITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPONEXITW::SLEEPONEXIT_0 => false,
            SLEEPONEXITW::SLEEPONEXIT_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLEEPONEXITW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPONEXITW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPONEXITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "o not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit_0(self) -> &'a mut W {
        self.variant(SLEEPONEXITW::SLEEPONEXIT_0)
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn sleeponexit_1(self) -> &'a mut W {
        self.variant(SLEEPONEXITW::SLEEPONEXIT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `SLEEPDEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEPR {
    #[doc = "sleep"]
    SLEEPDEEP_0,
    #[doc = "deep sleep"]
    SLEEPDEEP_1,
}
impl crate::ToBits<bool> for SLEEPDEEPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLEEPDEEPR::SLEEPDEEP_0 => false,
            SLEEPDEEPR::SLEEPDEEP_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLEEPDEEP_R = crate::FR<bool, SLEEPDEEPR>;
impl SLEEPDEEP_R {
    #[doc = "Checks if the value of the field is `SLEEPDEEP_0`"]
    #[inline(always)]
    pub fn is_sleepdeep_0(&self) -> bool {
        *self == SLEEPDEEPR::SLEEPDEEP_0
    }
    #[doc = "Checks if the value of the field is `SLEEPDEEP_1`"]
    #[inline(always)]
    pub fn is_sleepdeep_1(&self) -> bool {
        *self == SLEEPDEEPR::SLEEPDEEP_1
    }
}
#[doc = "Values that can be written to the field `SLEEPDEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEPW {
    #[doc = "sleep"]
    SLEEPDEEP_0,
    #[doc = "deep sleep"]
    SLEEPDEEP_1,
}
impl SLEEPDEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPDEEPW::SLEEPDEEP_0 => false,
            SLEEPDEEPW::SLEEPDEEP_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLEEPDEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPDEEPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPDEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sleep"]
    #[inline(always)]
    pub fn sleepdeep_0(self) -> &'a mut W {
        self.variant(SLEEPDEEPW::SLEEPDEEP_0)
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn sleepdeep_1(self) -> &'a mut W {
        self.variant(SLEEPDEEPW::SLEEPDEEP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `SEVONPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPENDR {
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    SEVONPEND_0,
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    SEVONPEND_1,
}
impl crate::ToBits<bool> for SEVONPENDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEVONPENDR::SEVONPEND_0 => false,
            SEVONPENDR::SEVONPEND_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEVONPEND_R = crate::FR<bool, SEVONPENDR>;
impl SEVONPEND_R {
    #[doc = "Checks if the value of the field is `SEVONPEND_0`"]
    #[inline(always)]
    pub fn is_sevonpend_0(&self) -> bool {
        *self == SEVONPENDR::SEVONPEND_0
    }
    #[doc = "Checks if the value of the field is `SEVONPEND_1`"]
    #[inline(always)]
    pub fn is_sevonpend_1(&self) -> bool {
        *self == SEVONPENDR::SEVONPEND_1
    }
}
#[doc = "Values that can be written to the field `SEVONPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPENDW {
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    SEVONPEND_0,
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    SEVONPEND_1,
}
impl SEVONPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SEVONPENDW::SEVONPEND_0 => false,
            SEVONPENDW::SEVONPEND_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SEVONPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEVONPENDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVONPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn sevonpend_0(self) -> &'a mut W {
        self.variant(SEVONPENDW::SEVONPEND_0)
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn sevonpend_1(self) -> &'a mut W {
        self.variant(SEVONPENDW::SEVONPEND_1)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> _SLEEPONEXITW {
        _SLEEPONEXITW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> _SLEEPDEEPW {
        _SLEEPDEEPW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> _SEVONPENDW {
        _SEVONPENDW { w: self }
    }
}
