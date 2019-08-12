#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INFO {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type FRAME_NR_R = crate::FR<u16, u16>;
#[doc = r"Reader of the field"]
pub type ERR_CODE_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MINREV_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MAJREV_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Frame number."]
    #[inline(always)]
    pub fn frame_nr(&self) -> FRAME_NR_R {
        FRAME_NR_R::new((self.bits() & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - The error code which last occurred:."]
    #[inline(always)]
    pub fn err_code(&self) -> ERR_CODE_R {
        ERR_CODE_R::new(((self.bits() >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Minor revision."]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major revision."]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
