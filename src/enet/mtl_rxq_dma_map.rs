#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MTL_RXQ_DMA_MAP {
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
pub type Q0MDMACH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _Q0MDMACHW<'a> {
    w: &'a mut W,
}
impl<'a> _Q0MDMACHW<'a> {
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
pub type Q0DDMACH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _Q0DDMACHW<'a> {
    w: &'a mut W,
}
impl<'a> _Q0DDMACHW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type Q1MDMACH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _Q1MDMACHW<'a> {
    w: &'a mut W,
}
impl<'a> _Q1MDMACHW<'a> {
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
pub type Q1DDMACH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _Q1DDMACHW<'a> {
    w: &'a mut W,
}
impl<'a> _Q1DDMACHW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
    #[inline(always)]
    pub fn q0mdmach(&self) -> Q0MDMACH_R {
        Q0MDMACH_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q0ddmach(&self) -> Q0DDMACH_R {
        Q0DDMACH_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
    #[inline(always)]
    pub fn q1mdmach(&self) -> Q1MDMACH_R {
        Q1MDMACH_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q1ddmach(&self) -> Q1DDMACH_R {
        Q1DDMACH_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset."]
    #[inline(always)]
    pub fn q0mdmach(&mut self) -> _Q0MDMACHW {
        _Q0MDMACHW { w: self }
    }
    #[doc = "Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q0ddmach(&mut self) -> _Q0DDMACHW {
        _Q0DDMACHW { w: self }
    }
    #[doc = "Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset."]
    #[inline(always)]
    pub fn q1mdmach(&mut self) -> _Q1MDMACHW {
        _Q1MDMACHW { w: self }
    }
    #[doc = "Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address."]
    #[inline(always)]
    pub fn q1ddmach(&mut self) -> _Q1DDMACHW {
        _Q1DDMACHW { w: self }
    }
}
