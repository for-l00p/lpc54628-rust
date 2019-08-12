#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_INTR_STAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PHYIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PMTIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LPIIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TSIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXSTSIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSTSIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - PHY Interrupt."]
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Status."]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Status."]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timestamp interrupt status."]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt."]
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt."]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
}
