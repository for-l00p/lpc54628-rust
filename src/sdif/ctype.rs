#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTYPE {
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
pub type CARD_WIDTH0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CARD_WIDTH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_WIDTH0W<'a> {
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
pub type CARD_WIDTH1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CARD_WIDTH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_WIDTH1W<'a> {
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
    #[doc = "Bit 0 - Indicates if card is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card_width0(&self) -> CARD_WIDTH0_R {
        CARD_WIDTH0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 16 - Indicates if card is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card_width1(&self) -> CARD_WIDTH1_R {
        CARD_WIDTH1_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Indicates if card is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card_width0(&mut self) -> _CARD_WIDTH0W {
        _CARD_WIDTH0W { w: self }
    }
    #[doc = "Bit 16 - Indicates if card is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card_width1(&mut self) -> _CARD_WIDTH1W {
        _CARD_WIDTH1W { w: self }
    }
}
