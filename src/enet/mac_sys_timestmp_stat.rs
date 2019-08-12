#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_SYS_TIMESTMP_STAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TSSOVF_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Time stamp seconds overflow When set, indicates that the seconds value of the Time stamp has overflowed beyond 0xFFFF_FFFF."]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits() & 0x01) != 0)
    }
}
