#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_FRAME_FILTER {
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
pub type PR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
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
pub type DAIF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DAIFW<'a> {
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
#[doc = r"Reader of the field"]
pub type PM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
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
pub type DBF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DBFW<'a> {
    w: &'a mut W,
}
impl<'a> _DBFW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PCF_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PCFW<'a> {
    w: &'a mut W,
}
impl<'a> _PCFW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SAIF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SAF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RAW<'a> {
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
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in the inverse filtering mode for SA address comparison."]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received packets with the values programmed in the enabled SA registers."]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames regardless of its destination or source address."]
    #[inline(always)]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames."]
    #[inline(always)]
    pub fn daif(&mut self) -> _DAIFW {
        _DAIFW { w: self }
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed."]
    #[inline(always)]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module filters all incoming broadcast frames."]
    #[inline(always)]
    pub fn dbf(&mut self) -> _DBFW {
        _DBFW { w: self }
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast PAUSE frames)."]
    #[inline(always)]
    pub fn pcf(&mut self) -> _PCFW {
        _PCFW { w: self }
    }
    #[doc = "Bit 31 - Receive all When this bit is set, the MAC Receiver module passes to the Application all frames received irrespective of whether they pass the address filter."]
    #[inline(always)]
    pub fn ra(&mut self) -> _RAW {
        _RAW { w: self }
    }
}
