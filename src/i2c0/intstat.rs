#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTPENDING_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MSTARBLOSS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MSTSTSTPERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SLVPENDING_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SLVNOTSTR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SLVDESEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MONRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MONOV_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MONIDLE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EVENTTIMEOUT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SCLTIMEOUT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Pending."]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending."]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching status."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag."]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event time-out Interrupt flag."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL time-out Interrupt flag."]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
