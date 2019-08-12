#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CHX_CTRL {
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
pub type PBLX8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PBLX8W<'a> {
    w: &'a mut W,
}
impl<'a> _PBLX8W<'a> {
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
#[doc = r"Reader of the field"]
pub type DSL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DSLW<'a> {
    w: &'a mut W,
}
impl<'a> _DSLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits() >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - 8xPBL mode When this bit is set, the PBL value programmed in Bits\\[21:16\\] in DMA Channel Transmit Control Table 780 is multiplied eight times."]
    #[inline(always)]
    pub fn pblx8(&mut self) -> _PBLX8W {
        _PBLX8W { w: self }
    }
    #[doc = "Bits 18:20 - Skip Length This bit specifies the Word, Dword, or Lword number (depending on the 32- bit, 64-bit, or 128-bit bus) to skip between two unchained s."]
    #[inline(always)]
    pub fn dsl(&mut self) -> _DSLW {
        _DSLW { w: self }
    }
}
