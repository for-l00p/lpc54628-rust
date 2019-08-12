#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PHY_CTRL {
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
#[doc = "Possible values of the field `PHY_FALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_FALLR {
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    RISING_EDGE,
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    FALLING_EDGE,
}
impl crate::ToBits<bool> for PHY_FALLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PHY_FALLR::RISING_EDGE => false,
            PHY_FALLR::FALLING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PHY_FALL_R = crate::FR<bool, PHY_FALLR>;
impl PHY_FALL_R {
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PHY_FALLR::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PHY_FALLR::FALLING_EDGE
    }
}
#[doc = "Values that can be written to the field `PHY_FALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_FALLW {
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    RISING_EDGE,
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    FALLING_EDGE,
}
impl PHY_FALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PHY_FALLW::RISING_EDGE => false,
            PHY_FALLW::FALLING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PHY_FALLW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_FALLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHY_FALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(PHY_FALLW::RISING_EDGE)
    }
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(PHY_FALLW::FALLING_EDGE)
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
#[doc = "Possible values of the field `PHY_HALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_HALFR {
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    STANDARD,
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    HALF_RATE,
}
impl crate::ToBits<bool> for PHY_HALFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PHY_HALFR::STANDARD => false,
            PHY_HALFR::HALF_RATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PHY_HALF_R = crate::FR<bool, PHY_HALFR>;
impl PHY_HALF_R {
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == PHY_HALFR::STANDARD
    }
    #[doc = "Checks if the value of the field is `HALF_RATE`"]
    #[inline(always)]
    pub fn is_half_rate(&self) -> bool {
        *self == PHY_HALFR::HALF_RATE
    }
}
#[doc = "Values that can be written to the field `PHY_HALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_HALFW {
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    STANDARD,
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    HALF_RATE,
}
impl PHY_HALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PHY_HALFW::STANDARD => false,
            PHY_HALFW::HALF_RATE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PHY_HALFW<'a> {
    w: &'a mut W,
}
impl<'a> _PHY_HALFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHY_HALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(PHY_HALFW::STANDARD)
    }
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    #[inline(always)]
    pub fn half_rate(self) -> &'a mut W {
        self.variant(PHY_HALFW::HALF_RATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Capture PDM_DATA"]
    #[inline(always)]
    pub fn phy_fall(&self) -> PHY_FALL_R {
        PHY_FALL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Half rate sampling"]
    #[inline(always)]
    pub fn phy_half(&self) -> PHY_HALF_R {
        PHY_HALF_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Capture PDM_DATA"]
    #[inline(always)]
    pub fn phy_fall(&mut self) -> _PHY_FALLW {
        _PHY_FALLW { w: self }
    }
    #[doc = "Bit 1 - Half rate sampling"]
    #[inline(always)]
    pub fn phy_half(&mut self) -> _PHY_HALFW {
        _PHY_HALFW { w: self }
    }
}
