#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTRAW {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type FUFRIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LNBURIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type VCOMPRIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BERRAW_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - FIFO underflow raw interrupt status."]
    #[inline(always)]
    pub fn fufris(&self) -> FUFRIS_R {
        FUFRIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update raw interrupt status."]
    #[inline(always)]
    pub fn lnburis(&self) -> LNBURIS_R {
        LNBURIS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare raw interrupt status."]
    #[inline(always)]
    pub fn vcompris(&self) -> VCOMPRIS_R {
        VCOMPRIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error raw interrupt status."]
    #[inline(always)]
    pub fn berraw(&self) -> BERRAW_R {
        BERRAW_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
