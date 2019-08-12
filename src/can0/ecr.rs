#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ECR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TEC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type REC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RP_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CEL_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Transmit error counter."]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive error counter."]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive error passive."]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CAN error logging."]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
}
