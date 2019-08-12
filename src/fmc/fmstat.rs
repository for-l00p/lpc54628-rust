#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FMSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type SIG_DONE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed."]
    #[inline(always)]
    pub fn sig_done(&self) -> SIG_DONE_R {
        SIG_DONE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
