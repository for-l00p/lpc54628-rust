#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFSR {
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
#[doc = "Possible values of the field `VECTTBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBLR {
    #[doc = "no BusFault on vector table read"]
    VECTTBL_0,
    #[doc = "BusFault on vector table read"]
    VECTTBL_1,
}
impl crate::ToBits<bool> for VECTTBLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            VECTTBLR::VECTTBL_0 => false,
            VECTTBLR::VECTTBL_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type VECTTBL_R = crate::FR<bool, VECTTBLR>;
impl VECTTBL_R {
    #[doc = "Checks if the value of the field is `VECTTBL_0`"]
    #[inline(always)]
    pub fn is_vecttbl_0(&self) -> bool {
        *self == VECTTBLR::VECTTBL_0
    }
    #[doc = "Checks if the value of the field is `VECTTBL_1`"]
    #[inline(always)]
    pub fn is_vecttbl_1(&self) -> bool {
        *self == VECTTBLR::VECTTBL_1
    }
}
#[doc = "Values that can be written to the field `VECTTBL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTTBLW {
    #[doc = "no BusFault on vector table read"]
    VECTTBL_0,
    #[doc = "BusFault on vector table read"]
    VECTTBL_1,
}
impl VECTTBLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            VECTTBLW::VECTTBL_0 => false,
            VECTTBLW::VECTTBL_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _VECTTBLW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTTBLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTTBLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl_0(self) -> &'a mut W {
        self.variant(VECTTBLW::VECTTBL_0)
    }
    #[doc = "BusFault on vector table read"]
    #[inline(always)]
    pub fn vecttbl_1(self) -> &'a mut W {
        self.variant(VECTTBLW::VECTTBL_1)
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
#[doc = "Possible values of the field `FORCED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDR {
    #[doc = "no forced HardFault"]
    FORCED_0,
    #[doc = "forced HardFault"]
    FORCED_1,
}
impl crate::ToBits<bool> for FORCEDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FORCEDR::FORCED_0 => false,
            FORCEDR::FORCED_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FORCED_R = crate::FR<bool, FORCEDR>;
impl FORCED_R {
    #[doc = "Checks if the value of the field is `FORCED_0`"]
    #[inline(always)]
    pub fn is_forced_0(&self) -> bool {
        *self == FORCEDR::FORCED_0
    }
    #[doc = "Checks if the value of the field is `FORCED_1`"]
    #[inline(always)]
    pub fn is_forced_1(&self) -> bool {
        *self == FORCEDR::FORCED_1
    }
}
#[doc = "Values that can be written to the field `FORCED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEDW {
    #[doc = "no forced HardFault"]
    FORCED_0,
    #[doc = "forced HardFault"]
    FORCED_1,
}
impl FORCEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEDW::FORCED_0 => false,
            FORCEDW::FORCED_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FORCEDW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no forced HardFault"]
    #[inline(always)]
    pub fn forced_0(self) -> &'a mut W {
        self.variant(FORCEDW::FORCED_0)
    }
    #[doc = "forced HardFault"]
    #[inline(always)]
    pub fn forced_1(self) -> &'a mut W {
        self.variant(FORCEDW::FORCED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DEBUGEVT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEBUGEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUGEVTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
    pub fn vecttbl(&self) -> VECTTBL_R {
        VECTTBL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn forced(&self) -> FORCED_R {
        FORCED_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn debugevt(&self) -> DEBUGEVT_R {
        DEBUGEVT_R::new(((self.bits() >> 31) & 0x01) != 0)
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
    pub fn vecttbl(&mut self) -> _VECTTBLW {
        _VECTTBLW { w: self }
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn forced(&mut self) -> _FORCEDW {
        _FORCEDW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn debugevt(&mut self) -> _DEBUGEVTW {
        _DEBUGEVTW { w: self }
    }
}
