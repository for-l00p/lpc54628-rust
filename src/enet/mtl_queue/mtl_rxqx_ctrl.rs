#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_RXQX_CTRL {
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
pub type RXQ_WEGT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RXQ_WEGTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXQ_WEGTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RXQ_FRM_ARBIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXQ_FRM_ARBITW<'a> {
    w: &'a mut W,
}
impl<'a> _RXQ_FRM_ARBITW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Receive Queue Weight This field indicates the weight assigned to the Rx Queue 0."]
    #[inline(always)]
    pub fn rxq_wegt(&mut self) -> _RXQ_WEGTW {
        _RXQ_WEGTW { w: self }
    }
    #[doc = "Bit 3 - Receive Queue Packet Arbitration When this bit is set, the The ethernet block drives the packet data to the ARI interface such that the entire packet data of currently-selected queue is transmitted before switching to other queue."]
    #[inline(always)]
    pub fn rxq_frm_arbit(&mut self) -> _RXQ_FRM_ARBITW {
        _RXQ_FRM_ARBITW { w: self }
    }
}
