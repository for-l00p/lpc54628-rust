#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_RXQ_CTRL1 {
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
pub type AVCPQ_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _AVCPQW<'a> {
    w: &'a mut W,
}
impl<'a> _AVCPQW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AVPTPQ_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _AVPTPQW<'a> {
    w: &'a mut W,
}
impl<'a> _AVPTPQW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type UPQ_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _UPQW<'a> {
    w: &'a mut W,
}
impl<'a> _UPQW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MCBCQ_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MCBCQW<'a> {
    w: &'a mut W,
}
impl<'a> _MCBCQW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MCBCQEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MCBCQENW<'a> {
    w: &'a mut W,
}
impl<'a> _MCBCQENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits() >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub fn avcpq(&mut self) -> _AVCPQW {
        _AVCPQW { w: self }
    }
    #[doc = "Bits 4:6 - AV PTP Packets Queue."]
    #[inline(always)]
    pub fn avptpq(&mut self) -> _AVPTPQW {
        _AVPTPQW { w: self }
    }
    #[doc = "Bits 12:14 - Untagged Packet Queue."]
    #[inline(always)]
    pub fn upq(&mut self) -> _UPQW {
        _UPQW { w: self }
    }
    #[doc = "Bits 16:18 - Multicast and Broadcast Queue."]
    #[inline(always)]
    pub fn mcbcq(&mut self) -> _MCBCQW {
        _MCBCQW { w: self }
    }
    #[doc = "Bit 20 - Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> _MCBCQENW {
        _MCBCQENW { w: self }
    }
}
