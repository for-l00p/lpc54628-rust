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
pub type END_OF_PROG_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - EEPROM program operation finished interrupt status bit."]
    #[inline(always)]
    pub fn end_of_prog(&self) -> END_OF_PROG_R {
        END_OF_PROG_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
