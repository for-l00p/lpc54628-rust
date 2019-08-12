#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIDR1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PARTNUMBER_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type JEP106_IDENTITY_CODE_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Part Number \\[11:8\\]"]
    #[inline(always)]
    pub fn part_number(&self) -> PARTNUMBER_R {
        PARTNUMBER_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code \\[3:0\\]"]
    #[inline(always)]
    pub fn jep106_identity_code(&self) -> JEP106_IDENTITY_CODE_R {
        JEP106_IDENTITY_CODE_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
