#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTSTAT {
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
pub type EP0OUT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP0OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP0OUTW<'a> {
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
pub type EP0IN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP0INW<'a> {
    w: &'a mut W,
}
impl<'a> _EP0INW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EP1OUT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP1OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP1OUTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EP1IN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP1INW<'a> {
    w: &'a mut W,
}
impl<'a> _EP1INW<'a> {
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
pub type EP2OUT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP2OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP2OUTW<'a> {
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
pub type EP2IN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP2INW<'a> {
    w: &'a mut W,
}
impl<'a> _EP2INW<'a> {
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
pub type EP3OUT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP3OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP3OUTW<'a> {
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
pub type EP3IN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP3INW<'a> {
    w: &'a mut W,
}
impl<'a> _EP3INW<'a> {
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
pub type EP4OUT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP4OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP4OUTW<'a> {
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
pub type EP4IN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP4INW<'a> {
    w: &'a mut W,
}
impl<'a> _EP4INW<'a> {
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
pub type EP5OUT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP5OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP5OUTW<'a> {
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
pub type EP5IN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EP5INW<'a> {
    w: &'a mut W,
}
impl<'a> _EP5INW<'a> {
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
pub type FRAME_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FRAME_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAME_INTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DEV_INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEV_INTW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_INTW<'a> {
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
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn ep0out(&self) -> EP0OUT_R {
        EP0OUT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn ep0in(&self) -> EP0IN_R {
        EP0IN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn ep1out(&self) -> EP1OUT_R {
        EP1OUT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn ep1in(&self) -> EP1IN_R {
        EP1IN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn ep2out(&self) -> EP2OUT_R {
        EP2OUT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn ep2in(&self) -> EP2IN_R {
        EP2IN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn ep3out(&self) -> EP3OUT_R {
        EP3OUT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn ep3in(&self) -> EP3IN_R {
        EP3IN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn ep4out(&self) -> EP4OUT_R {
        EP4OUT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn ep4in(&self) -> EP4IN_R {
        EP4IN_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn ep5out(&self) -> EP5OUT_R {
        EP5OUT_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn ep5in(&self) -> EP5IN_R {
        EP5IN_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    pub fn frame_int(&self) -> FRAME_INT_R {
        FRAME_INT_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    pub fn dev_int(&self) -> DEV_INT_R {
        DEV_INT_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn ep0out(&mut self) -> _EP0OUTW {
        _EP0OUTW { w: self }
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn ep0in(&mut self) -> _EP0INW {
        _EP0INW { w: self }
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn ep1out(&mut self) -> _EP1OUTW {
        _EP1OUTW { w: self }
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn ep1in(&mut self) -> _EP1INW {
        _EP1INW { w: self }
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn ep2out(&mut self) -> _EP2OUTW {
        _EP2OUTW { w: self }
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn ep2in(&mut self) -> _EP2INW {
        _EP2INW { w: self }
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn ep3out(&mut self) -> _EP3OUTW {
        _EP3OUTW { w: self }
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn ep3in(&mut self) -> _EP3INW {
        _EP3INW { w: self }
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn ep4out(&mut self) -> _EP4OUTW {
        _EP4OUTW { w: self }
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn ep4in(&mut self) -> _EP4INW {
        _EP4INW { w: self }
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn ep5out(&mut self) -> _EP5OUTW {
        _EP5OUTW { w: self }
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn ep5in(&mut self) -> _EP5INW {
        _EP5INW { w: self }
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    pub fn frame_int(&mut self) -> _FRAME_INTW {
        _FRAME_INTW { w: self }
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    pub fn dev_int(&mut self) -> _DEV_INTW {
        _DEV_INTW { w: self }
    }
}
