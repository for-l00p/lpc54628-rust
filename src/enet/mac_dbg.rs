#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_DBG {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type REPESTS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RFCFCSTS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type TPESTS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TFCSTS_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC MII receive protocol engine is actively receiving data, and it is not in the Idle state."]
    #[inline(always)]
    pub fn repests(&self) -> REPESTS_R {
        REPESTS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module."]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC or MII transmit protocol engine is actively transmitting data, and it is not in the Idle state."]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module."]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits() >> 17) & 0x03) as u8)
    }
}
