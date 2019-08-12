#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RDR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type THRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TEMT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXFE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receiver Data Ready."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Framing Error."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
