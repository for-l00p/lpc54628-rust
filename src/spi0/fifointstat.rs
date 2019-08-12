#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FIFOINTSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXLVL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXLVL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PERINT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TX FIFO error."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO error."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO level interrupt."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO level interrupt."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral interrupt."]
    #[inline(always)]
    pub fn perint(&self) -> PERINT_R {
        PERINT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
