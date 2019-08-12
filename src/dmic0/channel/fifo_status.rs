#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFO_STATUS {
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
pub type INT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTW<'a> {
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
pub type OVERRUN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OVERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUNW<'a> {
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
pub type UNDERRUN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UNDERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERRUNW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt flag. Asserted when FIFO data reaches the level specified in the FIFOCTRL register. Writing a one to this bit clears the flag. Remark: note that the bus clock to the DMIC subsystem must be running in order for an interrupt to occur."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun flag. Indicates that a FIFO overflow has occurred at some point. Writing a one to this bit clears the flag. This flag does not cause an interrupt."]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Underrun flag. Indicates that a FIFO underflow has occurred at some point. Writing a one to this bit clears the flag."]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt flag. Asserted when FIFO data reaches the level specified in the FIFOCTRL register. Writing a one to this bit clears the flag. Remark: note that the bus clock to the DMIC subsystem must be running in order for an interrupt to occur."]
    #[inline(always)]
    pub fn int(&mut self) -> _INTW {
        _INTW { w: self }
    }
    #[doc = "Bit 1 - Overrun flag. Indicates that a FIFO overflow has occurred at some point. Writing a one to this bit clears the flag. This flag does not cause an interrupt."]
    #[inline(always)]
    pub fn overrun(&mut self) -> _OVERRUNW {
        _OVERRUNW { w: self }
    }
    #[doc = "Bit 2 - Underrun flag. Indicates that a FIFO underflow has occurred at some point. Writing a one to this bit clears the flag."]
    #[inline(always)]
    pub fn underrun(&mut self) -> _UNDERRUNW {
        _UNDERRUNW { w: self }
    }
}
