#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMP_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUSR {
    #[doc = "Access permitted."]
    STATUS_0,
    #[doc = "Write access to the component is blocked. All writes to control registers are ignored. Reads are permitted."]
    STATUS_1,
}
impl crate::ToBits<bool> for STATUSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STATUSR::STATUS_0 => false,
            STATUSR::STATUS_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type STATUS_R = crate::FR<bool, STATUSR>;
impl STATUS_R {
    #[doc = "Checks if the value of the field is `STATUS_0`"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == STATUSR::STATUS_0
    }
    #[doc = "Checks if the value of the field is `STATUS_1`"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == STATUSR::STATUS_1
    }
}
#[doc = r"Reader of the field"]
pub type S8BIT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Lock mechanism is implemented. This bit always reads 1."]
    #[inline(always)]
    pub fn imp(&self) -> IMP_R {
        IMP_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Status. This bit is HIGH when the device is locked, and LOW when unlocked."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Access Lock Register size. This bit reads 0 to indicate a 32-bit register is present."]
    #[inline(always)]
    pub fn s8bit(&self) -> S8BIT_R {
        S8BIT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
