#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type B_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type S_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SA_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Busy."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write buffer status."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh acknowledge."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
