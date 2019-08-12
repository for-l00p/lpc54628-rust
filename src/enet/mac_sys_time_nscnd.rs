#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_SYS_TIME_NSCND {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TSSS_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits() & 0x7fff_ffff) as u32)
    }
}
