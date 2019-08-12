#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIDR3 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CUSTOMERMODIFIED_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type REVAND_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Customer Modified."]
    #[inline(always)]
    pub fn customer_modified(&self) -> CUSTOMERMODIFIED_R {
        CUSTOMERMODIFIED_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn rev_and(&self) -> REVAND_R {
        REVAND_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
