#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_DBG_STAT {
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
pub type AHSTS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AHSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _AHSTSW<'a> {
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
pub type RPS0_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type TPS0_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RPS1_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type TPS1_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&self) -> AHSTS_R {
        AHSTS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0: 0x0: Stopped (Reset or Stop Receive Command issued) 0x1: Running (Fetching Rx Transfer ) 0x2: Reserved 0x3: Running (Waiting for Rx packet) 0x4: Suspended (Rx Unavailable) 0x5: Running (Closing the Rx) 0x6: Timestamp write state 0x7: Running (Transferring the received packet data from the Rx buffer to the system memory) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0: 000: Stopped (Reset or Stop Transmit Command issued) 0x1: Running (Fetching Tx Transfer) 0x2: Running (Waiting for status) 0x3: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO)) 0x4: Timestamp write state 0x5: Reserved for future use 0x6: Suspended (Tx Unavailable or Tx Buffer Underflow) 0x7: Running (Closing Tx ) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits() >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&mut self) -> _AHSTSW {
        _AHSTSW { w: self }
    }
}
