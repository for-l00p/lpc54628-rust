#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_PMT_CRTL_STAT {
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
pub type PWRDWN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MGKPKTEN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RWKPKTEN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MGKPRCVD_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RWKPRCVD_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type GLBLUCAST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GLBLUCASTW<'a> {
    w: &'a mut W,
}
impl<'a> _GLBLUCASTW<'a> {
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
#[doc = r"Reader of the field"]
pub type RWKPFE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWKPFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RWKPFEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RWKPTR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RWKPTRW<'a> {
    w: &'a mut W,
}
impl<'a> _RWKPTRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RWKFILTRST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWKFILTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RWKFILTRSTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Remote Wake-Up Packet Enable When this bit is set, a power management event is generated when the MAC receives a remote wake-up packet."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Remote Wake-Up Packet Received."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits() >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 9 - Global Unicast When this bit set, any unicast packet filtered by the MAC (DAF) address recognition is detected as a remote wake-up packet."]
    #[inline(always)]
    pub fn glblucast(&mut self) -> _GLBLUCASTW {
        _GLBLUCASTW { w: self }
    }
    #[doc = "Bit 10 - Remote Wake-up Packet Forwarding Enable When this bit is set along with RWKPKTEN, the MAC receiver drops all received frames until it receives the expected wake-up frame."]
    #[inline(always)]
    pub fn rwkpfe(&mut self) -> _RWKPFEW {
        _RWKPFEW { w: self }
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer This field gives the current value (0 to 7) of the Remote Wake-up Packet Filter register pointer."]
    #[inline(always)]
    pub fn rwkptr(&mut self) -> _RWKPTRW {
        _RWKPTRW { w: self }
    }
    #[doc = "Bit 31 - Remote Wake-Up Packet Filter Register Pointer Reset When this bit is set, the remote wake-up packet filter register pointer is reset to 3'b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> _RWKFILTRSTW {
        _RWKFILTRSTW { w: self }
    }
}
