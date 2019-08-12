#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ULPIDEBUG {
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
pub type PHY_ADDR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHY_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_ADDRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHY_WDATA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHY_WDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_WDATAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHY_RDATA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHY_RDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_RDATAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHY_RW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PHY_RWW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_RWW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHY_ACCESS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PHY_ACCESSW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_ACCESSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHY_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PHY_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_MODEW<'a> {
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
    #[doc = "Bits 0:7 - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_wdata(&self) -> PHY_WDATA_R {
        PHY_WDATA_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
    #[inline(always)]
    pub fn phy_rdata(&self) -> PHY_RDATA_R {
        PHY_RDATA_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_rw(&self) -> PHY_RW_R {
        PHY_RW_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Software writes this bit to one to start a read or write operation."]
    #[inline(always)]
    pub fn phy_access(&self) -> PHY_ACCESS_R {
        PHY_ACCESS_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
    #[inline(always)]
    pub fn phy_mode(&self) -> PHY_MODE_R {
        PHY_MODE_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - ULPI mode: Bits 7:0 are used as the address when doing a register access over the ULPI interface."]
    #[inline(always)]
    pub fn phy_addr(&mut self) -> _PHY_ADDRW {
        _PHY_ADDRW { w: self }
    }
    #[doc = "Bits 8:15 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_wdata(&mut self) -> _PHY_WDATAW {
        _PHY_WDATAW { w: self }
    }
    #[doc = "Bits 16:23 - UTMI+ mode: Bits 7:0 contains the value returned by the VStatus signal on Vendor Interface of UTMI+."]
    #[inline(always)]
    pub fn phy_rdata(&mut self) -> _PHY_RDATAW {
        _PHY_RDATAW { w: self }
    }
    #[doc = "Bit 24 - UTMI+ mode: Reserved."]
    #[inline(always)]
    pub fn phy_rw(&mut self) -> _PHY_RWW {
        _PHY_RWW { w: self }
    }
    #[doc = "Bit 25 - Software writes this bit to one to start a read or write operation."]
    #[inline(always)]
    pub fn phy_access(&mut self) -> _PHY_ACCESSW {
        _PHY_ACCESSW { w: self }
    }
    #[doc = "Bit 31 - This bit indicates if the interface between the controller is UTMI+ or ULPI."]
    #[inline(always)]
    pub fn phy_mode(&mut self) -> _PHY_MODEW {
        _PHY_MODEW { w: self }
    }
}
