#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_RXQX_DBG {
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
pub type RWCSTS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWCSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _RWCSTSW<'a> {
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
pub type RRCSTS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RXQSTS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PRXQ_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State This field gives the state of the Rx queue Read controller: 00: Idle state 01: Reading packet data 10: Reading packet status (or timestamp) 11: Flushing the packet data and status."]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status This field gives the status of the fill-level of the Rx Queue: 0x0: Rx Queue empty 0x1: Rx Queue fill-level below flow-control deactivate threshold 0x2: Rx Queue fill-level above flow-control activate threshold 0x3: Rx Queue full."]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue This field indicates the current number of packets in the Rx Queue."]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits() >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status When high, this bit indicates that the MTL Rx queue Write controller is active, and it is transferring a received packet to the Rx Queue."]
    #[inline(always)]
    pub fn rwcsts(&mut self) -> _RWCSTSW {
        _RWCSTSW { w: self }
    }
}
