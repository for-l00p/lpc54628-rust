#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIRCR {
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
        0xfa05_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Proxy"]
pub struct _VECTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTRESETW<'a> {
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
#[doc = r"Proxy"]
pub struct _VECTCLRACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTCLRACTIVEW<'a> {
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
#[doc = "Values that can be written to the field `SYSRESETREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRESETREQW {
    #[doc = "no system reset request"]
    SYSRESETREQ_0,
    #[doc = "asserts a signal to the outer system that requests a reset"]
    SYSRESETREQ_1,
}
impl SYSRESETREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSRESETREQW::SYSRESETREQ_0 => false,
            SYSRESETREQW::SYSRESETREQ_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSRESETREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRESETREQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRESETREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no system reset request"]
    #[inline(always)]
    pub fn sysresetreq_0(self) -> &'a mut W {
        self.variant(SYSRESETREQW::SYSRESETREQ_0)
    }
    #[doc = "asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn sysresetreq_1(self) -> &'a mut W {
        self.variant(SYSRESETREQW::SYSRESETREQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRIGROUP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRIGROUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIGROUPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `ENDIANNESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESSR {
    #[doc = "Little-endian"]
    ENDIANNESS_0,
    #[doc = "Big-endian"]
    ENDIANNESS_1,
}
impl crate::ToBits<bool> for ENDIANNESSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENDIANNESSR::ENDIANNESS_0 => false,
            ENDIANNESSR::ENDIANNESS_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENDIANNESS_R = crate::FR<bool, ENDIANNESSR>;
impl ENDIANNESS_R {
    #[doc = "Checks if the value of the field is `ENDIANNESS_0`"]
    #[inline(always)]
    pub fn is_endianness_0(&self) -> bool {
        *self == ENDIANNESSR::ENDIANNESS_0
    }
    #[doc = "Checks if the value of the field is `ENDIANNESS_1`"]
    #[inline(always)]
    pub fn is_endianness_1(&self) -> bool {
        *self == ENDIANNESSR::ENDIANNESS_1
    }
}
#[doc = r"Reader of the field"]
pub type VECTKEY_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _VECTKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTKEYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn vectreset(&mut self) -> _VECTRESETW {
        _VECTRESETW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn vectclractive(&mut self) -> _VECTCLRACTIVEW {
        _VECTCLRACTIVEW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> _SYSRESETREQW {
        _SYSRESETREQW { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub fn prigroup(&mut self) -> _PRIGROUPW {
        _PRIGROUPW { w: self }
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&mut self) -> _VECTKEYW {
        _VECTKEYW { w: self }
    }
}
