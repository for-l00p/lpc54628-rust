#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USE2FS {
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
#[doc = "Possible values of the field `USE2FS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE2FSR {
    #[doc = "Use 1FS output for PCM data."]
    USE_1FS,
    #[doc = "Use 2FS output for PCM data."]
    USE_2FS,
}
impl crate::ToBits<bool> for USE2FSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USE2FSR::USE_1FS => false,
            USE2FSR::USE_2FS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USE2FS_R = crate::FR<bool, USE2FSR>;
impl USE2FS_R {
    #[doc = "Checks if the value of the field is `USE_1FS`"]
    #[inline(always)]
    pub fn is_use_1fs(&self) -> bool {
        *self == USE2FSR::USE_1FS
    }
    #[doc = "Checks if the value of the field is `USE_2FS`"]
    #[inline(always)]
    pub fn is_use_2fs(&self) -> bool {
        *self == USE2FSR::USE_2FS
    }
}
#[doc = "Values that can be written to the field `USE2FS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE2FSW {
    #[doc = "Use 1FS output for PCM data."]
    USE_1FS,
    #[doc = "Use 2FS output for PCM data."]
    USE_2FS,
}
impl USE2FSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USE2FSW::USE_1FS => false,
            USE2FSW::USE_2FS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USE2FSW<'a> {
    w: &'a mut W,
}
impl<'a> _USE2FSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USE2FSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use 1FS output for PCM data."]
    #[inline(always)]
    pub fn use_1fs(self) -> &'a mut W {
        self.variant(USE2FSW::USE_1FS)
    }
    #[doc = "Use 2FS output for PCM data."]
    #[inline(always)]
    pub fn use_2fs(self) -> &'a mut W {
        self.variant(USE2FSW::USE_2FS)
    }
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&self) -> USE2FS_R {
        USE2FS_R::new((self.bits() & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Use 2FS register"]
    #[inline(always)]
    pub fn use2fs(&mut self) -> _USE2FSW {
        _USE2FSW { w: self }
    }
}
