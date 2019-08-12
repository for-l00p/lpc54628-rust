#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIDR4 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type JEP106_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type C4KB_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - JEP106 continuation code."]
    #[inline(always)]
    pub fn jep106(&self) -> JEP106_R {
        JEP106_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn c4kb(&self) -> C4KB_R {
        C4KB_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
