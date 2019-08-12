#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIDR2 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type JEP106_IDENTITY_CODE_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type REVISION_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - JEP106 identity code \\[6:4\\]"]
    #[inline(always)]
    pub fn jep106_identity_code(&self) -> JEP106_IDENTITY_CODE_R {
        JEP106_IDENTITY_CODE_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
