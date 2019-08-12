#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DBTP {
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
        0x0a33
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type DSJW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DSJWW<'a> {
    w: &'a mut W,
}
impl<'a> _DSJWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DTSEG2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DTSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _DTSEG2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DTSEG1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DTSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _DTSEG1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DBRP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DBRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBRPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TDC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TDCW<'a> {
    w: &'a mut W,
}
impl<'a> _TDCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Data (re)synchronization jump width."]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point."]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment before sample point."]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits() >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler."]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transmitter delay compensation."]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Data (re)synchronization jump width."]
    #[inline(always)]
    pub fn dsjw(&mut self) -> _DSJWW {
        _DSJWW { w: self }
    }
    #[doc = "Bits 4:7 - Data time segment after sample point."]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> _DTSEG2W {
        _DTSEG2W { w: self }
    }
    #[doc = "Bits 8:12 - Data time segment before sample point."]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> _DTSEG1W {
        _DTSEG1W { w: self }
    }
    #[doc = "Bits 16:20 - Data bit rate prescaler."]
    #[inline(always)]
    pub fn dbrp(&mut self) -> _DBRPW {
        _DBRPW { w: self }
    }
    #[doc = "Bit 23 - Transmitter delay compensation."]
    #[inline(always)]
    pub fn tdc(&mut self) -> _TDCW {
        _TDCW { w: self }
    }
}
