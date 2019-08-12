#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXTSSTSLO_R = crate::FR<u32, u32>;
#[doc = r"Reader of the field"]
pub type TXTSSTSMIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:30 - Transmit timestamp status low."]
    #[inline(always)]
    pub fn txtsstslo(&self) -> TXTSSTSLO_R {
        TXTSSTSLO_R::new((self.bits() & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Transmit timestamp status missed."]
    #[inline(always)]
    pub fn txtsstsmis(&self) -> TXTSSTSMIS_R {
        TXTSSTSMIS_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
