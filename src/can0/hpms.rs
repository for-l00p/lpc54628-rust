#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HPMS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type BIDX_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MSI_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type FIDX_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type FLST_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Buffer index."]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits() & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message storage indicator."]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - Filter index."]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter list."]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
