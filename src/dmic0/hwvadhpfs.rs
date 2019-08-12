#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWVADHPFS {
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
        0x01
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `HPFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPFSR {
    #[doc = "First filter by-pass."]
    BYPASS,
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    HIGH_PASS_1750HZ,
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    HIGH_PASS_215HZ,
}
impl crate::ToBits<u8> for HPFSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            HPFSR::BYPASS => 0,
            HPFSR::HIGH_PASS_1750HZ => 1,
            HPFSR::HIGH_PASS_215HZ => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HPFS_R = crate::FR<u8, HPFSR>;
impl HPFS_R {
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == HPFSR::BYPASS
    }
    #[doc = "Checks if the value of the field is `HIGH_PASS_1750HZ`"]
    #[inline(always)]
    pub fn is_high_pass_1750hz(&self) -> bool {
        *self == HPFSR::HIGH_PASS_1750HZ
    }
    #[doc = "Checks if the value of the field is `HIGH_PASS_215HZ`"]
    #[inline(always)]
    pub fn is_high_pass_215hz(&self) -> bool {
        *self == HPFSR::HIGH_PASS_215HZ
    }
}
#[doc = "Values that can be written to the field `HPFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPFSW {
    #[doc = "First filter by-pass."]
    BYPASS,
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    HIGH_PASS_1750HZ,
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    HIGH_PASS_215HZ,
}
impl HPFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPFSW::BYPASS => 0,
            HPFSW::HIGH_PASS_1750HZ => 1,
            HPFSW::HIGH_PASS_215HZ => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HPFSW<'a> {
    w: &'a mut W,
}
impl<'a> _HPFSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPFSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "First filter by-pass."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(HPFSW::BYPASS)
    }
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    #[inline(always)]
    pub fn high_pass_1750hz(self) -> &'a mut W {
        self.variant(HPFSW::HIGH_PASS_1750HZ)
    }
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    #[inline(always)]
    pub fn high_pass_215hz(self) -> &'a mut W {
        self.variant(HPFSW::HIGH_PASS_215HZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - High pass filter"]
    #[inline(always)]
    pub fn hpfs(&self) -> HPFS_R {
        HPFS_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - High pass filter"]
    #[inline(always)]
    pub fn hpfs(&mut self) -> _HPFSW {
        _HPFSW { w: self }
    }
}
