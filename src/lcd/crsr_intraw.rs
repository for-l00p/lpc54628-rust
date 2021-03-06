#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CRSR_INTRAW {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CRSRRIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Cursor raw interrupt status."]
    #[inline(always)]
    pub fn crsrris(&self) -> CRSRRIS_R {
        CRSRRIS_R::new((self.bits() & 0x01) != 0)
    }
}
