#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHAN_THRSEL {
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
#[doc = "Possible values of the field `CH0_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_THRSELR {
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    THRESHOLD0,
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    THRESHOLD1,
}
impl crate::ToBits<bool> for CH0_THRSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CH0_THRSELR::THRESHOLD0 => false,
            CH0_THRSELR::THRESHOLD1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH0_THRSEL_R = crate::FR<bool, CH0_THRSELR>;
impl CH0_THRSEL_R {
    #[doc = "Checks if the value of the field is `THRESHOLD0`"]
    #[inline(always)]
    pub fn is_threshold0(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD1`"]
    #[inline(always)]
    pub fn is_threshold1(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD1
    }
}
#[doc = "Values that can be written to the field `CH0_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_THRSELW {
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    THRESHOLD0,
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    THRESHOLD1,
}
impl CH0_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0_THRSELW::THRESHOLD0 => false,
            CH0_THRSELW::THRESHOLD1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CH0_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_THRSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Results for this channel will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers."]
    #[inline(always)]
    pub fn threshold0(self) -> &'a mut W {
        self.variant(CH0_THRSELW::THRESHOLD0)
    }
    #[doc = "Threshold 1. Results for this channel will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers."]
    #[inline(always)]
    pub fn threshold1(self) -> &'a mut W {
        self.variant(CH0_THRSELW::THRESHOLD1)
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
pub type CH1_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH1_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_THRSELW<'a> {
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
pub type CH2_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH2_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_THRSELW<'a> {
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
pub type CH3_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH3_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH4_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH4_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH5_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH5_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH6_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH6_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH7_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH7_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH8_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH8_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8_THRSELW<'a> {
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
#[doc = r"Reader of the field"]
pub type CH9_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH9_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH10_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH10_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CH11_THRSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CH11_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11_THRSELW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Threshold select for channel 0."]
    #[inline(always)]
    pub fn ch0_thrsel(&self) -> CH0_THRSEL_R {
        CH0_THRSEL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Threshold select for channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn ch1_thrsel(&self) -> CH1_THRSEL_R {
        CH1_THRSEL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Threshold select for channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn ch2_thrsel(&self) -> CH2_THRSEL_R {
        CH2_THRSEL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Threshold select for channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn ch3_thrsel(&self) -> CH3_THRSEL_R {
        CH3_THRSEL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Threshold select for channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn ch4_thrsel(&self) -> CH4_THRSEL_R {
        CH4_THRSEL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Threshold select for channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn ch5_thrsel(&self) -> CH5_THRSEL_R {
        CH5_THRSEL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Threshold select for channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn ch6_thrsel(&self) -> CH6_THRSEL_R {
        CH6_THRSEL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Threshold select for channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn ch7_thrsel(&self) -> CH7_THRSEL_R {
        CH7_THRSEL_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Threshold select for channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn ch8_thrsel(&self) -> CH8_THRSEL_R {
        CH8_THRSEL_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Threshold select for channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn ch9_thrsel(&self) -> CH9_THRSEL_R {
        CH9_THRSEL_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Threshold select for channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn ch10_thrsel(&self) -> CH10_THRSEL_R {
        CH10_THRSEL_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Threshold select for channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn ch11_thrsel(&self) -> CH11_THRSEL_R {
        CH11_THRSEL_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Threshold select for channel 0."]
    #[inline(always)]
    pub fn ch0_thrsel(&mut self) -> _CH0_THRSELW {
        _CH0_THRSELW { w: self }
    }
    #[doc = "Bit 1 - Threshold select for channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn ch1_thrsel(&mut self) -> _CH1_THRSELW {
        _CH1_THRSELW { w: self }
    }
    #[doc = "Bit 2 - Threshold select for channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn ch2_thrsel(&mut self) -> _CH2_THRSELW {
        _CH2_THRSELW { w: self }
    }
    #[doc = "Bit 3 - Threshold select for channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn ch3_thrsel(&mut self) -> _CH3_THRSELW {
        _CH3_THRSELW { w: self }
    }
    #[doc = "Bit 4 - Threshold select for channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn ch4_thrsel(&mut self) -> _CH4_THRSELW {
        _CH4_THRSELW { w: self }
    }
    #[doc = "Bit 5 - Threshold select for channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn ch5_thrsel(&mut self) -> _CH5_THRSELW {
        _CH5_THRSELW { w: self }
    }
    #[doc = "Bit 6 - Threshold select for channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn ch6_thrsel(&mut self) -> _CH6_THRSELW {
        _CH6_THRSELW { w: self }
    }
    #[doc = "Bit 7 - Threshold select for channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn ch7_thrsel(&mut self) -> _CH7_THRSELW {
        _CH7_THRSELW { w: self }
    }
    #[doc = "Bit 8 - Threshold select for channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn ch8_thrsel(&mut self) -> _CH8_THRSELW {
        _CH8_THRSELW { w: self }
    }
    #[doc = "Bit 9 - Threshold select for channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn ch9_thrsel(&mut self) -> _CH9_THRSELW {
        _CH9_THRSELW { w: self }
    }
    #[doc = "Bit 10 - Threshold select for channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn ch10_thrsel(&mut self) -> _CH10_THRSELW {
        _CH10_THRSELW { w: self }
    }
    #[doc = "Bit 11 - Threshold select for channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn ch11_thrsel(&mut self) -> _CH11_THRSELW {
        _CH11_THRSELW { w: self }
    }
}
