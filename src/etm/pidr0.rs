#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIDR0 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PARTNUMBER_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Part Number \\[7:0\\]"]
    #[inline(always)]
    pub fn part_number(&self) -> PARTNUMBER_R {
        PARTNUMBER_R::new((self.bits() & 0xff) as u8)
    }
}
