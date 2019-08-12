#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INSEL {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "ADC0_IN0 function."]
    ADC0_IN0,
    #[doc = "Internal temperature sensor."]
    TEMPERATURE_SENSOR,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::ADC0_IN0 => 0,
            SELR::TEMPERATURE_SENSOR => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<u8, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `ADC0_IN0`"]
    #[inline(always)]
    pub fn is_adc0_in0(&self) -> bool {
        *self == SELR::ADC0_IN0
    }
    #[doc = "Checks if the value of the field is `TEMPERATURE_SENSOR`"]
    #[inline(always)]
    pub fn is_temperature_sensor(&self) -> bool {
        *self == SELR::TEMPERATURE_SENSOR
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "ADC0_IN0 function."]
    ADC0_IN0,
    #[doc = "Internal temperature sensor."]
    TEMPERATURE_SENSOR,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::ADC0_IN0 => 0,
            SELW::TEMPERATURE_SENSOR => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ADC0_IN0 function."]
    #[inline(always)]
    pub fn adc0_in0(self) -> &'a mut W {
        self.variant(SELW::ADC0_IN0)
    }
    #[doc = "Internal temperature sensor."]
    #[inline(always)]
    pub fn temperature_sensor(self) -> &'a mut W {
        self.variant(SELW::TEMPERATURE_SENSOR)
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
    #[doc = "Bits 0:1 - Selects the input source for channel 0. All other values are reserved."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Selects the input source for channel 0. All other values are reserved."]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
