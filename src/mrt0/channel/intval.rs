#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTVAL {
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
pub type IVALUE_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _IVALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _IVALUEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Possible values of the field `LOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADR {
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD,
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD,
}
impl crate::ToBits<bool> for LOADR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LOADR::NO_FORCE_LOAD => false,
            LOADR::FORCE_LOAD => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LOAD_R = crate::FR<bool, LOADR>;
impl LOAD_R {
    #[doc = "Checks if the value of the field is `NO_FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_no_force_load(&self) -> bool {
        *self == LOADR::NO_FORCE_LOAD
    }
    #[doc = "Checks if the value of the field is `FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_force_load(&self) -> bool {
        *self == LOADR::FORCE_LOAD
    }
}
#[doc = "Values that can be written to the field `LOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOADW {
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD,
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD,
}
impl LOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LOADW::NO_FORCE_LOAD => false,
            LOADW::FORCE_LOAD => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _LOADW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    #[inline(always)]
    pub fn no_force_load(self) -> &'a mut W {
        self.variant(LOADW::NO_FORCE_LOAD)
    }
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    #[inline(always)]
    pub fn force_load(self) -> &'a mut W {
        self.variant(LOADW::FORCE_LOAD)
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
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn ivalue(&self) -> IVALUE_R {
        IVALUE_R::new((self.bits() & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn ivalue(&mut self) -> _IVALUEW {
        _IVALUEW { w: self }
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn load(&mut self) -> _LOADW {
        _LOADW { w: self }
    }
}
