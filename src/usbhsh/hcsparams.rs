#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HCSPARAMS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type N_PORTS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PPC_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P_INDICATOR_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - This register specifies the number of physical downstream ports implemented on this host controller."]
    #[inline(always)]
    pub fn n_ports(&self) -> N_PORTS_R {
        N_PORTS_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This field indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub fn ppc(&self) -> PPC_R {
        PPC_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit indicates whether the ports support port indicator control."]
    #[inline(always)]
    pub fn p_indicator(&self) -> P_INDICATOR_R {
        P_INDICATOR_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
