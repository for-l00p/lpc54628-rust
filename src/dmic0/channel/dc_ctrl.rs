#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DC_CTRL {
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
#[doc = "Possible values of the field `DCPOLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCPOLER {
    #[doc = "Flat response, no filter."]
    FLAT_RESPONSE,
    #[doc = "155 Hz."]
    HZ_155,
    #[doc = "78 Hz."]
    HZ_78,
    #[doc = "39 Hz"]
    HZ_39,
}
impl crate::ToBits<u8> for DCPOLER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DCPOLER::FLAT_RESPONSE => 0,
            DCPOLER::HZ_155 => 1,
            DCPOLER::HZ_78 => 2,
            DCPOLER::HZ_39 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DCPOLE_R = crate::FR<u8, DCPOLER>;
impl DCPOLE_R {
    #[doc = "Checks if the value of the field is `FLAT_RESPONSE`"]
    #[inline(always)]
    pub fn is_flat_response(&self) -> bool {
        *self == DCPOLER::FLAT_RESPONSE
    }
    #[doc = "Checks if the value of the field is `HZ_155`"]
    #[inline(always)]
    pub fn is_hz_155(&self) -> bool {
        *self == DCPOLER::HZ_155
    }
    #[doc = "Checks if the value of the field is `HZ_78`"]
    #[inline(always)]
    pub fn is_hz_78(&self) -> bool {
        *self == DCPOLER::HZ_78
    }
    #[doc = "Checks if the value of the field is `HZ_39`"]
    #[inline(always)]
    pub fn is_hz_39(&self) -> bool {
        *self == DCPOLER::HZ_39
    }
}
#[doc = "Values that can be written to the field `DCPOLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCPOLEW {
    #[doc = "Flat response, no filter."]
    FLAT_RESPONSE,
    #[doc = "155 Hz."]
    HZ_155,
    #[doc = "78 Hz."]
    HZ_78,
    #[doc = "39 Hz"]
    HZ_39,
}
impl DCPOLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCPOLEW::FLAT_RESPONSE => 0,
            DCPOLEW::HZ_155 => 1,
            DCPOLEW::HZ_78 => 2,
            DCPOLEW::HZ_39 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DCPOLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCPOLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCPOLEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Flat response, no filter."]
    #[inline(always)]
    pub fn flat_response(self) -> &'a mut W {
        self.variant(DCPOLEW::FLAT_RESPONSE)
    }
    #[doc = "155 Hz."]
    #[inline(always)]
    pub fn hz_155(self) -> &'a mut W {
        self.variant(DCPOLEW::HZ_155)
    }
    #[doc = "78 Hz."]
    #[inline(always)]
    pub fn hz_78(self) -> &'a mut W {
        self.variant(DCPOLEW::HZ_78)
    }
    #[doc = "39 Hz"]
    #[inline(always)]
    pub fn hz_39(self) -> &'a mut W {
        self.variant(DCPOLEW::HZ_39)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DCGAIN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DCGAINW<'a> {
    w: &'a mut W,
}
impl<'a> _DCGAINW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SATURATEAT16BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SATURATEAT16BITR {
    #[doc = "Results roll over if out range and do not saturate."]
    DO_NOT_SATURATE,
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    SATURATE,
}
impl crate::ToBits<bool> for SATURATEAT16BITR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SATURATEAT16BITR::DO_NOT_SATURATE => false,
            SATURATEAT16BITR::SATURATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SATURATEAT16BIT_R = crate::FR<bool, SATURATEAT16BITR>;
impl SATURATEAT16BIT_R {
    #[doc = "Checks if the value of the field is `DO_NOT_SATURATE`"]
    #[inline(always)]
    pub fn is_do_not_saturate(&self) -> bool {
        *self == SATURATEAT16BITR::DO_NOT_SATURATE
    }
    #[doc = "Checks if the value of the field is `SATURATE`"]
    #[inline(always)]
    pub fn is_saturate(&self) -> bool {
        *self == SATURATEAT16BITR::SATURATE
    }
}
#[doc = "Values that can be written to the field `SATURATEAT16BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SATURATEAT16BITW {
    #[doc = "Results roll over if out range and do not saturate."]
    DO_NOT_SATURATE,
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    SATURATE,
}
impl SATURATEAT16BITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SATURATEAT16BITW::DO_NOT_SATURATE => false,
            SATURATEAT16BITW::SATURATE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SATURATEAT16BITW<'a> {
    w: &'a mut W,
}
impl<'a> _SATURATEAT16BITW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SATURATEAT16BITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Results roll over if out range and do not saturate."]
    #[inline(always)]
    pub fn do_not_saturate(self) -> &'a mut W {
        self.variant(SATURATEAT16BITW::DO_NOT_SATURATE)
    }
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    #[inline(always)]
    pub fn saturate(self) -> &'a mut W {
        self.variant(SATURATEAT16BITW::SATURATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - DC block filter"]
    #[inline(always)]
    pub fn dcpole(&self) -> DCPOLE_R {
        DCPOLE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub fn dcgain(&self) -> DCGAIN_R {
        DCGAIN_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Selects 16-bit saturation."]
    #[inline(always)]
    pub fn saturateat16bit(&self) -> SATURATEAT16BIT_R {
        SATURATEAT16BIT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - DC block filter"]
    #[inline(always)]
    pub fn dcpole(&mut self) -> _DCPOLEW {
        _DCPOLEW { w: self }
    }
    #[doc = "Bits 4:7 - Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub fn dcgain(&mut self) -> _DCGAINW {
        _DCGAINW { w: self }
    }
    #[doc = "Bit 8 - Selects 16-bit saturation."]
    #[inline(always)]
    pub fn saturateat16bit(&mut self) -> _SATURATEAT16BITW {
        _SATURATEAT16BITW { w: self }
    }
}
