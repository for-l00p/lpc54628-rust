#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVQUAL0 {
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
#[doc = "Possible values of the field `QUALMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALMODE0R {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK,
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND,
}
impl crate::ToBits<bool> for QUALMODE0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            QUALMODE0R::MASK => false,
            QUALMODE0R::EXTEND => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type QUALMODE0_R = crate::FR<bool, QUALMODE0R>;
impl QUALMODE0_R {
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == QUALMODE0R::MASK
    }
    #[doc = "Checks if the value of the field is `EXTEND`"]
    #[inline(always)]
    pub fn is_extend(&self) -> bool {
        *self == QUALMODE0R::EXTEND
    }
}
#[doc = "Values that can be written to the field `QUALMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALMODE0W {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK,
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND,
}
impl QUALMODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            QUALMODE0W::MASK => false,
            QUALMODE0W::EXTEND => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _QUALMODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _QUALMODE0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUALMODE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(QUALMODE0W::MASK)
    }
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    #[inline(always)]
    pub fn extend(self) -> &'a mut W {
        self.variant(QUALMODE0W::EXTEND)
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
#[doc = r"Reader of the field"]
pub type SLVQUAL0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SLVQUAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _SLVQUAL0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn qualmode0(&self) -> QUALMODE0_R {
        QUALMODE0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn slvqual0(&self) -> SLVQUAL0_R {
        SLVQUAL0_R::new(((self.bits() >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn qualmode0(&mut self) -> _QUALMODE0W {
        _QUALMODE0W { w: self }
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn slvqual0(&mut self) -> _SLVQUAL0W {
        _SLVQUAL0W { w: self }
    }
}
