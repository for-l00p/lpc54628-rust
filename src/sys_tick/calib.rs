#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CALIB {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TENMS_R = crate::FR<u32, u32>;
#[doc = "Possible values of the field `SKEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWR {
    #[doc = "10ms calibration value is exact"]
    SKEW_0,
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    SKEW_1,
}
impl crate::ToBits<bool> for SKEWR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SKEWR::SKEW_0 => false,
            SKEWR::SKEW_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SKEW_R = crate::FR<bool, SKEWR>;
impl SKEW_R {
    #[doc = "Checks if the value of the field is `SKEW_0`"]
    #[inline(always)]
    pub fn is_skew_0(&self) -> bool {
        *self == SKEWR::SKEW_0
    }
    #[doc = "Checks if the value of the field is `SKEW_1`"]
    #[inline(always)]
    pub fn is_skew_1(&self) -> bool {
        *self == SKEWR::SKEW_1
    }
}
#[doc = "Possible values of the field `NOREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREFR {
    #[doc = "The reference clock is provided"]
    NOREF_0,
    #[doc = "The reference clock is not provided"]
    NOREF_1,
}
impl crate::ToBits<bool> for NOREFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NOREFR::NOREF_0 => false,
            NOREFR::NOREF_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NOREF_R = crate::FR<bool, NOREFR>;
impl NOREF_R {
    #[doc = "Checks if the value of the field is `NOREF_0`"]
    #[inline(always)]
    pub fn is_noref_0(&self) -> bool {
        *self == NOREFR::NOREF_0
    }
    #[doc = "Checks if the value of the field is `NOREF_1`"]
    #[inline(always)]
    pub fn is_noref_1(&self) -> bool {
        *self == NOREFR::NOREF_1
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits() & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
