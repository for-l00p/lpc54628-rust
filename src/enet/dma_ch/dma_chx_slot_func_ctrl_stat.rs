#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CHX_SLOT_FUNC_CTRL_STAT {
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
pub type ESC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ESCW<'a> {
    w: &'a mut W,
}
impl<'a> _ESCW<'a> {
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
pub type ASC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ASCW<'a> {
    w: &'a mut W,
}
impl<'a> _ASCW<'a> {
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
pub type RSN_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Reference Slot Number This field gives the current value of the reference slot number in the DMA."]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable Slot Comparison When set, this bit enables the checking of the slot numbers programmed in the Tx descriptor with the current reference given in the RSN field."]
    #[inline(always)]
    pub fn esc(&mut self) -> _ESCW {
        _ESCW { w: self }
    }
    #[doc = "Bit 1 - Advance Slot Check When set, this bit enables the D MA to fetch the data from the buffer when the slot number (SLOTNUM) programmed in the Tx descriptor is equal to the reference slot number given in the RSN field or, ahead of the reference slot number by up to two slots This bit is applicable only when the ESC bit is set."]
    #[inline(always)]
    pub fn asc(&mut self) -> _ASCW {
        _ASCW { w: self }
    }
}
