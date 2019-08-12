#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub type LCDEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDENW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDENW<'a> {
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
pub type LCDBPP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _LCDBPPW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDBPPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LCDBW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDBWW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDBWW<'a> {
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
pub type LCDTFT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDTFTW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDTFTW<'a> {
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
pub type LCDMONO8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDMONO8W<'a> {
    w: &'a mut W,
}
impl<'a> _LCDMONO8W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LCDDUAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDDUALW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDDUALW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BGR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BGRW<'a> {
    w: &'a mut W,
}
impl<'a> _BGRW<'a> {
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
pub type BEBO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BEBOW<'a> {
    w: &'a mut W,
}
impl<'a> _BEBOW<'a> {
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
pub type BEPO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BEPOW<'a> {
    w: &'a mut W,
}
impl<'a> _BEPOW<'a> {
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
pub type LCDPWR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCDPWRW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDPWRW<'a> {
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
pub type LCDVCOMP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _LCDVCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _LCDVCOMPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WATERMARK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WATERMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _WATERMARKW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LCD enable control bit."]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - LCD bits per pixel."]
    #[inline(always)]
    pub fn lcdbpp(&self) -> LCDBPP_R {
        LCDBPP_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection."]
    #[inline(always)]
    pub fn lcdbw(&self) -> LCDBW_R {
        LCDBW_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCD panel TFT type selection."]
    #[inline(always)]
    pub fn lcdtft(&self) -> LCDTFT_R {
        LCDTFT_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Monochrome LCD interface width."]
    #[inline(always)]
    pub fn lcdmono8(&self) -> LCDMONO8_R {
        LCDMONO8_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection."]
    #[inline(always)]
    pub fn lcddual(&self) -> LCDDUAL_R {
        LCDDUAL_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Color format selection."]
    #[inline(always)]
    pub fn bgr(&self) -> BGR_R {
        BGR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Big-endian Byte Order."]
    #[inline(always)]
    pub fn bebo(&self) -> BEBO_R {
        BEBO_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering."]
    #[inline(always)]
    pub fn bepo(&self) -> BEPO_R {
        BEPO_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCD power enable."]
    #[inline(always)]
    pub fn lcdpwr(&self) -> LCDPWR_R {
        LCDPWR_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt."]
    #[inline(always)]
    pub fn lcdvcomp(&self) -> LCDVCOMP_R {
        LCDVCOMP_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level."]
    #[inline(always)]
    pub fn watermark(&self) -> WATERMARK_R {
        WATERMARK_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LCD enable control bit."]
    #[inline(always)]
    pub fn lcden(&mut self) -> _LCDENW {
        _LCDENW { w: self }
    }
    #[doc = "Bits 1:3 - LCD bits per pixel."]
    #[inline(always)]
    pub fn lcdbpp(&mut self) -> _LCDBPPW {
        _LCDBPPW { w: self }
    }
    #[doc = "Bit 4 - STN LCD monochrome/color selection."]
    #[inline(always)]
    pub fn lcdbw(&mut self) -> _LCDBWW {
        _LCDBWW { w: self }
    }
    #[doc = "Bit 5 - LCD panel TFT type selection."]
    #[inline(always)]
    pub fn lcdtft(&mut self) -> _LCDTFTW {
        _LCDTFTW { w: self }
    }
    #[doc = "Bit 6 - Monochrome LCD interface width."]
    #[inline(always)]
    pub fn lcdmono8(&mut self) -> _LCDMONO8W {
        _LCDMONO8W { w: self }
    }
    #[doc = "Bit 7 - Single or Dual LCD panel selection."]
    #[inline(always)]
    pub fn lcddual(&mut self) -> _LCDDUALW {
        _LCDDUALW { w: self }
    }
    #[doc = "Bit 8 - Color format selection."]
    #[inline(always)]
    pub fn bgr(&mut self) -> _BGRW {
        _BGRW { w: self }
    }
    #[doc = "Bit 9 - Big-endian Byte Order."]
    #[inline(always)]
    pub fn bebo(&mut self) -> _BEBOW {
        _BEBOW { w: self }
    }
    #[doc = "Bit 10 - Big-Endian Pixel Ordering."]
    #[inline(always)]
    pub fn bepo(&mut self) -> _BEPOW {
        _BEPOW { w: self }
    }
    #[doc = "Bit 11 - LCD power enable."]
    #[inline(always)]
    pub fn lcdpwr(&mut self) -> _LCDPWRW {
        _LCDPWRW { w: self }
    }
    #[doc = "Bits 12:13 - LCD Vertical Compare Interrupt."]
    #[inline(always)]
    pub fn lcdvcomp(&mut self) -> _LCDVCOMPW {
        _LCDVCOMPW { w: self }
    }
    #[doc = "Bit 16 - LCD DMA FIFO watermark level."]
    #[inline(always)]
    pub fn watermark(&mut self) -> _WATERMARKW {
        _WATERMARKW { w: self }
    }
}
