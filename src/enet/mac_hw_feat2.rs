#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_HW_FEAT2 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXQCNT_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type TXQCNT_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RXCHCNT_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type TXCHCNT_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PPSOUTNUM_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type AUXSNAPNUM_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Number of MTL Receive Queues."]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Number of MTL Transmit Queues."]
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits() >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of DMA Receive Channels."]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Number of DMA Transmit Channels."]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits() >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Number of PPS Outputs."]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits() >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Number of Auxiliary Snapshot Inputs."]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits() >> 28) & 0x07) as u8)
    }
}
