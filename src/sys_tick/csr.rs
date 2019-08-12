#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
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
        0x04
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "counter disabled"]
    ENABLE_0,
    #[doc = "counter enabled"]
    ENABLE_1,
}
impl crate::ToBits<bool> for ENABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENABLER::ENABLE_0 => false,
            ENABLER::ENABLE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLE_R = crate::FR<bool, ENABLER>;
impl ENABLE_R {
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLER::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLER::ENABLE_1
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLEW {
    #[doc = "counter disabled"]
    ENABLE_0,
    #[doc = "counter enabled"]
    ENABLE_1,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::ENABLE_0 => false,
            ENABLEW::ENABLE_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_0)
    }
    #[doc = "counter enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLE_1)
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
#[doc = "Possible values of the field `TICKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINTR {
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    TICKINT_0,
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    TICKINT_1,
}
impl crate::ToBits<bool> for TICKINTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TICKINTR::TICKINT_0 => false,
            TICKINTR::TICKINT_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TICKINT_R = crate::FR<bool, TICKINTR>;
impl TICKINT_R {
    #[doc = "Checks if the value of the field is `TICKINT_0`"]
    #[inline(always)]
    pub fn is_tickint_0(&self) -> bool {
        *self == TICKINTR::TICKINT_0
    }
    #[doc = "Checks if the value of the field is `TICKINT_1`"]
    #[inline(always)]
    pub fn is_tickint_1(&self) -> bool {
        *self == TICKINTR::TICKINT_1
    }
}
#[doc = "Values that can be written to the field `TICKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINTW {
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    TICKINT_0,
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    TICKINT_1,
}
impl TICKINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TICKINTW::TICKINT_0 => false,
            TICKINTW::TICKINT_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TICKINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TICKINTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICKINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline(always)]
    pub fn tickint_0(self) -> &'a mut W {
        self.variant(TICKINTW::TICKINT_0)
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline(always)]
    pub fn tickint_1(self) -> &'a mut W {
        self.variant(TICKINTW::TICKINT_1)
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
#[doc = "Possible values of the field `CLKSOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCER {
    #[doc = "external clock"]
    CLKSOURCE_0,
    #[doc = "processor clock"]
    CLKSOURCE_1,
}
impl crate::ToBits<bool> for CLKSOURCER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CLKSOURCER::CLKSOURCE_0 => false,
            CLKSOURCER::CLKSOURCE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKSOURCE_R = crate::FR<bool, CLKSOURCER>;
impl CLKSOURCE_R {
    #[doc = "Checks if the value of the field is `CLKSOURCE_0`"]
    #[inline(always)]
    pub fn is_clksource_0(&self) -> bool {
        *self == CLKSOURCER::CLKSOURCE_0
    }
    #[doc = "Checks if the value of the field is `CLKSOURCE_1`"]
    #[inline(always)]
    pub fn is_clksource_1(&self) -> bool {
        *self == CLKSOURCER::CLKSOURCE_1
    }
}
#[doc = "Values that can be written to the field `CLKSOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCEW {
    #[doc = "external clock"]
    CLKSOURCE_0,
    #[doc = "processor clock"]
    CLKSOURCE_1,
}
impl CLKSOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSOURCEW::CLKSOURCE_0 => false,
            CLKSOURCEW::CLKSOURCE_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLKSOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSOURCEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSOURCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "external clock"]
    #[inline(always)]
    pub fn clksource_0(self) -> &'a mut W {
        self.variant(CLKSOURCEW::CLKSOURCE_0)
    }
    #[doc = "processor clock"]
    #[inline(always)]
    pub fn clksource_1(self) -> &'a mut W {
        self.variant(CLKSOURCEW::CLKSOURCE_1)
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
#[doc = r"Reader of the field"]
pub type COUNTFLAG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _COUNTFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTFLAGW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn tickint(&mut self) -> _TICKINTW {
        _TICKINTW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn clksource(&mut self) -> _CLKSOURCEW {
        _CLKSOURCEW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn countflag(&mut self) -> _COUNTFLAGW {
        _COUNTFLAGW { w: self }
    }
}
