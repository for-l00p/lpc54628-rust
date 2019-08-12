#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_ADDR_HIGH {
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
        0x8000_ffff
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type A47_32_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _A47_32W<'a> {
    w: &'a mut W,
}
impl<'a> _A47_32W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DCS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DCSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&self) -> A47_32_R {
        A47_32_R::new((self.bits() & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Address Enable."]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\] This field contains the upper 16 bits (47:32) of the 6-byte first MAC address."]
    #[inline(always)]
    pub fn a47_32(&mut self) -> _A47_32W {
        _A47_32W { w: self }
    }
    #[doc = "Bit 16 - DMA Channel Select This field contains the DMA Channel number to which the Rx packet whose DA matches the MAC Address content is routed."]
    #[inline(always)]
    pub fn dcs(&mut self) -> _DCSW {
        _DCSW { w: self }
    }
}
