#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_MODE {
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
pub type SWR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DAW<'a> {
    w: &'a mut W,
}
impl<'a> _DAW<'a> {
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
pub type TAA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TAAW<'a> {
    w: &'a mut W,
}
impl<'a> _TAAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TXPR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXPRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPRW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC and the OMA controller reset the logic and all internal registers of the OMA, MTL, and MAC."]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The Tx path has priority over the Rx path when the TXPR bit is set."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Transmit Arbitration Algorithm This field is used to select the arbitration algorithm for the Transmit side when multiple Tx DMAs are selected."]
    #[inline(always)]
    pub fn taa(&self) -> TAA_R {
        TAA_R::new(((self.bits() >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Transmit Priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Priority Ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits() >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC and the OMA controller reset the logic and all internal registers of the OMA, MTL, and MAC."]
    #[inline(always)]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme This bit specifies the arbitration scheme between the Transmit and Receive paths of all channels: The Tx path has priority over the Rx path when the TXPR bit is set."]
    #[inline(always)]
    pub fn da(&mut self) -> _DAW {
        _DAW { w: self }
    }
    #[doc = "Bits 2:4 - Transmit Arbitration Algorithm This field is used to select the arbitration algorithm for the Transmit side when multiple Tx DMAs are selected."]
    #[inline(always)]
    pub fn taa(&mut self) -> _TAAW {
        _TAAW { w: self }
    }
    #[doc = "Bit 11 - Transmit Priority When set, this bit indicates that the Tx DMA has higher priority than the Rx DMA during arbitration for the system-side bus."]
    #[inline(always)]
    pub fn txpr(&mut self) -> _TXPRW {
        _TXPRW { w: self }
    }
    #[doc = "Bits 12:14 - Priority Ratio These bits control the priority ratio in weighted round-robin arbitration between the Rx DMA and Tx DMA."]
    #[inline(always)]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
}
