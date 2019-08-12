#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RXF1C {
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
pub type F1SA_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _F1SAW<'a> {
    w: &'a mut W,
}
impl<'a> _F1SAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F1S_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F1SW<'a> {
    w: &'a mut W,
}
impl<'a> _F1SW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F1WM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _F1WMW<'a> {
    w: &'a mut W,
}
impl<'a> _F1WMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type F1OM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _F1OMW<'a> {
    w: &'a mut W,
}
impl<'a> _F1OMW<'a> {
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
    #[doc = "Bits 2:15 - Rx FIFO 1 start address."]
    #[inline(always)]
    pub fn f1sa(&self) -> F1SA_R {
        F1SA_R::new(((self.bits() >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 size 0 = No Rx FIFO 1."]
    #[inline(always)]
    pub fn f1s(&self) -> F1S_R {
        F1S_R::new(((self.bits() >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub fn f1wm(&self) -> F1WM_R {
        F1WM_R::new(((self.bits() >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 operation mode."]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 2:15 - Rx FIFO 1 start address."]
    #[inline(always)]
    pub fn f1sa(&mut self) -> _F1SAW {
        _F1SAW { w: self }
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 size 0 = No Rx FIFO 1."]
    #[inline(always)]
    pub fn f1s(&mut self) -> _F1SW {
        _F1SW { w: self }
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub fn f1wm(&mut self) -> _F1WMW {
        _F1WMW { w: self }
    }
    #[doc = "Bit 31 - FIFO 1 operation mode."]
    #[inline(always)]
    pub fn f1om(&mut self) -> _F1OMW {
        _F1OMW { w: self }
    }
}
