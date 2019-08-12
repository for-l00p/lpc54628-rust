#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MTL_INTR_STAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type Q0IS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type Q1IS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Queue 0 Interrupt status This bit indicates that there is an interrupt from Queue 0."]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Queue 1 Interrupt status This bit indicates that there is an interrupt from Queue 1."]
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
