#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DYNAMICCONFIG {
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
pub type MD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MDW<'a> {
    w: &'a mut W,
}
impl<'a> _MDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AM0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _AM0W<'a> {
    w: &'a mut W,
}
impl<'a> _AM0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AM1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AM1W<'a> {
    w: &'a mut W,
}
impl<'a> _AM1W<'a> {
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
pub type B_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BW<'a> {
    w: &'a mut W,
}
impl<'a> _BW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type P_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PW<'a> {
    w: &'a mut W,
}
impl<'a> _PW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&self) -> AM0_R {
        AM0_R::new(((self.bits() >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&self) -> AM1_R {
        AM1_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&mut self) -> _MDW {
        _MDW { w: self }
    }
    #[doc = "Bits 7:12 - See Table 933."]
    #[inline(always)]
    pub fn am0(&mut self) -> _AM0W {
        _AM0W { w: self }
    }
    #[doc = "Bit 14 - See Table 933."]
    #[inline(always)]
    pub fn am1(&mut self) -> _AM1W {
        _AM1W { w: self }
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&mut self) -> _BW {
        _BW { w: self }
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> _PW {
        _PW { w: self }
    }
}
