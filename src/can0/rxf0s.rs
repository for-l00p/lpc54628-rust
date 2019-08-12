#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXF0S {
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
pub type F0FL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F0FLW<'a> {
    w: &'a mut W,
}
impl<'a> _F0FLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F0GI_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F0GIW<'a> {
    w: &'a mut W,
}
impl<'a> _F0GIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F0PI_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F0PIW<'a> {
    w: &'a mut W,
}
impl<'a> _F0PIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F0F_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _F0FW<'a> {
    w: &'a mut W,
}
impl<'a> _F0FW<'a> {
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
pub type RF0L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RF0LW<'a> {
    w: &'a mut W,
}
impl<'a> _RF0LW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Rx FIFO 0 fill level."]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 get index."]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits() >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 put index."]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Rx FIFO 0 fill level."]
    #[inline(always)]
    pub fn f0fl(&mut self) -> _F0FLW {
        _F0FLW { w: self }
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 get index."]
    #[inline(always)]
    pub fn f0gi(&mut self) -> _F0GIW {
        _F0GIW { w: self }
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 put index."]
    #[inline(always)]
    pub fn f0pi(&mut self) -> _F0PIW {
        _F0PIW { w: self }
    }
    #[doc = "Bit 24 - Rx FIFO 0 full."]
    #[inline(always)]
    pub fn f0f(&mut self) -> _F0FW {
        _F0FW { w: self }
    }
    #[doc = "Bit 25 - Rx FIFO 0 message lost."]
    #[inline(always)]
    pub fn rf0l(&mut self) -> _RF0LW {
        _RF0LW { w: self }
    }
}
