#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TXEFS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type EFFL_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type EFGI_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type EFPI_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type EFF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TEFL_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Event FIFO fill level."]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits() & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO get index."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits() >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - Event FIFO put index."]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO full."]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
