#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type FUFMIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LNBUMIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type VCOMPMIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BERMIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - FIFO underflow masked interrupt status."]
    #[inline(always)]
    pub fn fufmis(&self) -> FUFMIS_R {
        FUFMIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update masked interrupt status."]
    #[inline(always)]
    pub fn lnbumis(&self) -> LNBUMIS_R {
        LNBUMIS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare masked interrupt status."]
    #[inline(always)]
    pub fn vcompmis(&self) -> VCOMPMIS_R {
        VCOMPMIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error masked interrupt status."]
    #[inline(always)]
    pub fn bermis(&self) -> BERMIS_R {
        BERMIS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
