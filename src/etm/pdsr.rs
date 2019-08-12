#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PDSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type ETMPOWEREDUP_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - The value of this bit indicates whether you can access the ETM Trace Registers. The value of this bit is always 1, indicating that the ETM Trace Registers can be accessed."]
    #[inline(always)]
    pub fn etmpoweredup(&self) -> ETMPOWEREDUP_R {
        ETMPOWEREDUP_R::new((self.bits() & 0x01) != 0)
    }
}
