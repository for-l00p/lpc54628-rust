#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWVADLOWZ {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type LOWZ_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Noise envelope estimator value."]
    #[inline(always)]
    pub fn lowz(&self) -> LOWZ_R {
        LOWZ_R::new((self.bits() & 0xffff) as u16)
    }
}
