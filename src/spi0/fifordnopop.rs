#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FIFORDNOPOP {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXDATA_R = crate::FR<u16, u16>;
#[doc = r"Reader of the field"]
pub type RXSSEL0_N_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSSEL1_N_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSSEL2_N_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSSEL3_N_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SOT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits() & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> RXSSEL0_N_R {
        RXSSEL0_N_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> RXSSEL1_N_R {
        RXSSEL1_N_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> RXSSEL2_N_R {
        RXSSEL2_N_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> RXSSEL3_N_R {
        RXSSEL3_N_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Start of transfer flag."]
    #[inline(always)]
    pub fn sot(&self) -> SOT_R {
        SOT_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
