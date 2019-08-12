#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POL {
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
pub type PCD_LO_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PCD_LOW<'a> {
    w: &'a mut W,
}
impl<'a> _PCD_LOW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ACB_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ACBW<'a> {
    w: &'a mut W,
}
impl<'a> _ACBW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IVS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IVSW<'a> {
    w: &'a mut W,
}
impl<'a> _IVSW<'a> {
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
pub type IHS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IHSW<'a> {
    w: &'a mut W,
}
impl<'a> _IHSW<'a> {
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
#[doc = r"Reader of the field"]
pub type IPC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IPCW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IOE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IOEW<'a> {
    w: &'a mut W,
}
impl<'a> _IOEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CPL_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _CPLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BCD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BCDW<'a> {
    w: &'a mut W,
}
impl<'a> _BCDW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PCD_HI_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PCD_HIW<'a> {
    w: &'a mut W,
}
impl<'a> _PCD_HIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_lo(&self) -> PCD_LO_R {
        PCD_LO_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - AC bias pin frequency."]
    #[inline(always)]
    pub fn acb(&self) -> ACB_R {
        ACB_R::new(((self.bits() >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Invert vertical synchronization."]
    #[inline(always)]
    pub fn ivs(&self) -> IVS_R {
        IVS_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Invert horizontal synchronization."]
    #[inline(always)]
    pub fn ihs(&self) -> IHS_R {
        IHS_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Invert panel clock."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Invert output enable."]
    #[inline(always)]
    pub fn ioe(&self) -> IOE_R {
        IOE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Clocks per line."]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits() >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Bypass panel clock divider."]
    #[inline(always)]
    pub fn bcd(&self) -> BCD_R {
        BCD_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_hi(&self) -> PCD_HI_R {
        PCD_HI_R::new(((self.bits() >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_lo(&mut self) -> _PCD_LOW {
        _PCD_LOW { w: self }
    }
    #[doc = "Bits 6:10 - AC bias pin frequency."]
    #[inline(always)]
    pub fn acb(&mut self) -> _ACBW {
        _ACBW { w: self }
    }
    #[doc = "Bit 11 - Invert vertical synchronization."]
    #[inline(always)]
    pub fn ivs(&mut self) -> _IVSW {
        _IVSW { w: self }
    }
    #[doc = "Bit 12 - Invert horizontal synchronization."]
    #[inline(always)]
    pub fn ihs(&mut self) -> _IHSW {
        _IHSW { w: self }
    }
    #[doc = "Bit 13 - Invert panel clock."]
    #[inline(always)]
    pub fn ipc(&mut self) -> _IPCW {
        _IPCW { w: self }
    }
    #[doc = "Bit 14 - Invert output enable."]
    #[inline(always)]
    pub fn ioe(&mut self) -> _IOEW {
        _IOEW { w: self }
    }
    #[doc = "Bits 16:25 - Clocks per line."]
    #[inline(always)]
    pub fn cpl(&mut self) -> _CPLW {
        _CPLW { w: self }
    }
    #[doc = "Bit 26 - Bypass panel clock divider."]
    #[inline(always)]
    pub fn bcd(&mut self) -> _BCDW {
        _BCDW { w: self }
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_hi(&mut self) -> _PCD_HIW {
        _PCD_HIW { w: self }
    }
}
