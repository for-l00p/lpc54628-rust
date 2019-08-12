#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::JTAGIDCODE {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type JTAGID_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - JTAG ID code."]
    #[inline(always)]
    pub fn jtagid(&self) -> JTAGID_R {
        JTAGID_R::new((self.bits() & 0xffff_ffff) as u32)
    }
}
