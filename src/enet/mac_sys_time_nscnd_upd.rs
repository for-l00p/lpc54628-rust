#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_SYS_TIME_NSCND_UPD {
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
pub type TSSS_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _TSSSW<'a> {
    w: &'a mut W,
}
impl<'a> _TSSSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADDSUB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADDSUBW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDSUBW<'a> {
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
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits() & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Add or subtract time When this bit is set, the time value is subtracted with the contents of the update register."]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0."]
    #[inline(always)]
    pub fn tsss(&mut self) -> _TSSSW {
        _TSSSW { w: self }
    }
    #[doc = "Bit 31 - Add or subtract time When this bit is set, the time value is subtracted with the contents of the update register."]
    #[inline(always)]
    pub fn addsub(&mut self) -> _ADDSUBW {
        _ADDSUBW { w: self }
    }
}
