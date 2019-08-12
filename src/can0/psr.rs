#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type LEC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type ACT_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type EP_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EW_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BO_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DLEC_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RESI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RBRS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RFDF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PXE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TDCV_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Last error code."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Activity."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Error Passive."]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Warning status."]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Off Status."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Data phase last error code."]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - ESI flag of the last received CAN FD message."]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received CAN FD message."]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Received a CAN FD message."]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol exception event."]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter delay compensation value."]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits() >> 16) & 0x7f) as u8)
    }
}
