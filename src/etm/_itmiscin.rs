#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::_ITMISCIN {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTIN_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type COREHALT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - A read of these bits returns the value of the EXTIN\\[1:0\\] input pins."]
    #[inline(always)]
    pub fn extin(&self) -> EXTIN_R {
        EXTIN_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 4 - A read of this bit returns the value of the COREHALT input pin."]
    #[inline(always)]
    pub fn corehalt(&self) -> COREHALT_R {
        COREHALT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
