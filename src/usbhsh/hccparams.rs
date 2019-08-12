#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HCCPARAMS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPMC_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 17 - Link Power Management Capability."]
    #[inline(always)]
    pub fn lpmc(&self) -> LPMC_R {
        LPMC_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
