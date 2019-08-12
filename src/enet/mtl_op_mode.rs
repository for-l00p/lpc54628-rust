#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_OP_MODE {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type DTXSTS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DTXSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTXSTSW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RAA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SCHALG_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SCHALGW<'a> {
    w: &'a mut W,
}
impl<'a> _SCHALGW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CNTPRST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CNTPRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTPRSTW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CNTCLR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CNTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTCLRW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Arbitration Algorithm This field is used to select the arbitration algorithm for the Rx side."]
    #[inline(always)]
    pub fn raa(&self) -> RAA_R {
        RAA_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
    #[inline(always)]
    pub fn schalg(&self) -> SCHALG_R {
        SCHALG_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset."]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Drop Transmit Status When this bit is set, the Tx packet status received from the MAC is dropped in the MTL."]
    #[inline(always)]
    pub fn dtxsts(&mut self) -> _DTXSTSW {
        _DTXSTSW { w: self }
    }
    #[doc = "Bits 5:6 - Tx Scheduling Algorithm This field indicates the algorithm for Tx scheduling: 0x00: WRR algorithm 0x1: Reserved 0x2: Reserved 0x3: Strict priority algorithm."]
    #[inline(always)]
    pub fn schalg(&mut self) -> _SCHALGW {
        _SCHALGW { w: self }
    }
    #[doc = "Bit 8 - Counters Preset When this bit is set, MTL TxQ0 Underflow register (Table 762) and MTL_TxQ1_Underflow (Table 762) registers are initialized/preset to 0x7F0."]
    #[inline(always)]
    pub fn cntprst(&mut self) -> _CNTPRSTW {
        _CNTPRSTW { w: self }
    }
    #[doc = "Bit 9 - Counters Reset When this bit is set, all counters are reset."]
    #[inline(always)]
    pub fn cntclr(&mut self) -> _CNTCLRW {
        _CNTCLRW { w: self }
    }
}
