#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SSD_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MSTIDLE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Slave Select Assert."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deassert."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master Idle status flag."]
    #[inline(always)]
    pub fn mstidle(&self) -> MSTIDLE_R {
        MSTIDLE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
