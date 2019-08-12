#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAP {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAP_VALUE_R = crate::FR<u32, u32>;
#[doc = r"Reader of the field"]
pub type VALID_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:30 - Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub fn cap_value(&self) -> CAP_VALUE_R {
        CAP_VALUE_R::new((self.bits() & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
