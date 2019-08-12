#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IIR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTSTATUS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type INTID_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type FIFOENABLE_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt status."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Copies of SCInFCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
}
