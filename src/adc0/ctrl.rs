#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
        0x0600
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type CLKDIV_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Possible values of the field `ASYNMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNMODER {
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    SYNCHRONOUS_MODE,
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    ASYNCHRONOUS_MODE,
}
impl crate::ToBits<bool> for ASYNMODER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ASYNMODER::SYNCHRONOUS_MODE => false,
            ASYNMODER::ASYNCHRONOUS_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ASYNMODE_R = crate::FR<bool, ASYNMODER>;
impl ASYNMODE_R {
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        *self == ASYNMODER::SYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == ASYNMODER::ASYNCHRONOUS_MODE
    }
}
#[doc = "Values that can be written to the field `ASYNMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASYNMODEW {
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    SYNCHRONOUS_MODE,
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    ASYNCHRONOUS_MODE,
}
impl ASYNMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ASYNMODEW::SYNCHRONOUS_MODE => false,
            ASYNMODEW::ASYNCHRONOUS_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ASYNMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASYNMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Synchronous mode. The ADC clock is derived from the system clock based on the divide value selected in the CLKDIV field. The ADC clock will be started in a controlled fashion in response to a trigger to eliminate any uncertainty in the launching of an ADC conversion in response to any synchronous (on-chip) trigger. In Synchronous mode with the SYNCBYPASS bit (in a sequence control register) set, sampling of the ADC input and start of conversion will initiate 2 system clocks after the leading edge of a (synchronous) trigger pulse."]
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODEW::SYNCHRONOUS_MODE)
    }
    #[doc = "Asynchronous mode. The ADC clock is based on the output of the ADC clock divider ADCCLKSEL in the SYSCON block."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(ASYNMODEW::ASYNCHRONOUS_MODE)
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
#[doc = "Possible values of the field `RESOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLR {
    #[doc = "6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_6_BIT,
    #[doc = "8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_8_BIT,
    #[doc = "10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_10_BIT,
    #[doc = "12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_12_BIT,
}
impl crate::ToBits<u8> for RESOLR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RESOLR::RESOLUTION_6_BIT => 0,
            RESOLR::RESOLUTION_8_BIT => 1,
            RESOLR::RESOLUTION_10_BIT => 2,
            RESOLR::RESOLUTION_12_BIT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RESOL_R = crate::FR<u8, RESOLR>;
impl RESOL_R {
    #[doc = "Checks if the value of the field is `RESOLUTION_6_BIT`"]
    #[inline(always)]
    pub fn is_resolution_6_bit(&self) -> bool {
        *self == RESOLR::RESOLUTION_6_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_8_BIT`"]
    #[inline(always)]
    pub fn is_resolution_8_bit(&self) -> bool {
        *self == RESOLR::RESOLUTION_8_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_10_BIT`"]
    #[inline(always)]
    pub fn is_resolution_10_bit(&self) -> bool {
        *self == RESOLR::RESOLUTION_10_BIT
    }
    #[doc = "Checks if the value of the field is `RESOLUTION_12_BIT`"]
    #[inline(always)]
    pub fn is_resolution_12_bit(&self) -> bool {
        *self == RESOLR::RESOLUTION_12_BIT
    }
}
#[doc = "Values that can be written to the field `RESOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLW {
    #[doc = "6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_6_BIT,
    #[doc = "8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_8_BIT,
    #[doc = "10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_10_BIT,
    #[doc = "12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    RESOLUTION_12_BIT,
}
impl RESOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESOLW::RESOLUTION_6_BIT => 0,
            RESOLW::RESOLUTION_8_BIT => 1,
            RESOLW::RESOLUTION_10_BIT => 2,
            RESOLW::RESOLUTION_12_BIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RESOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RESOLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESOLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "6-bit resolution. An ADC conversion requires 9 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_6_bit(self) -> &'a mut W {
        self.variant(RESOLW::RESOLUTION_6_BIT)
    }
    #[doc = "8-bit resolution. An ADC conversion requires 11 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_8_bit(self) -> &'a mut W {
        self.variant(RESOLW::RESOLUTION_8_BIT)
    }
    #[doc = "10-bit resolution. An ADC conversion requires 13 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_10_bit(self) -> &'a mut W {
        self.variant(RESOLW::RESOLUTION_10_BIT)
    }
    #[doc = "12-bit resolution. An ADC conversion requires 15 ADC clocks, plus any clocks specified by the TSAMP field."]
    #[inline(always)]
    pub fn resolution_12_bit(self) -> &'a mut W {
        self.variant(RESOLW::RESOLUTION_12_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `BYPASSCAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSCALR {
    #[doc = "Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    CALIBRATE,
    #[doc = "Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    BYPASS_CALIBRATION,
}
impl crate::ToBits<bool> for BYPASSCALR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BYPASSCALR::CALIBRATE => false,
            BYPASSCALR::BYPASS_CALIBRATION => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BYPASSCAL_R = crate::FR<bool, BYPASSCALR>;
impl BYPASSCAL_R {
    #[doc = "Checks if the value of the field is `CALIBRATE`"]
    #[inline(always)]
    pub fn is_calibrate(&self) -> bool {
        *self == BYPASSCALR::CALIBRATE
    }
    #[doc = "Checks if the value of the field is `BYPASS_CALIBRATION`"]
    #[inline(always)]
    pub fn is_bypass_calibration(&self) -> bool {
        *self == BYPASSCALR::BYPASS_CALIBRATION
    }
}
#[doc = "Values that can be written to the field `BYPASSCAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSCALW {
    #[doc = "Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    CALIBRATE,
    #[doc = "Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    BYPASS_CALIBRATION,
}
impl BYPASSCALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSCALW::CALIBRATE => false,
            BYPASSCALW::BYPASS_CALIBRATION => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BYPASSCALW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSCALW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSCALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calibrate. The stored calibration value will be applied to the ADC during conversions to compensated for offset error. A calibration cycle must be performed each time the chip is powered-up. Re-calibration may be warranted periodically - especially if operating conditions have changed."]
    #[inline(always)]
    pub fn calibrate(self) -> &'a mut W {
        self.variant(BYPASSCALW::CALIBRATE)
    }
    #[doc = "Bypass calibration. Calibration is not utilized. Less time is required when enabling the ADC - particularly following chip power-up. Attempts to launch a calibration cycle are blocked when this bit is set."]
    #[inline(always)]
    pub fn bypass_calibration(self) -> &'a mut W {
        self.variant(BYPASSCALW::BYPASS_CALIBRATION)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TSAMP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TSAMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSAMPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline(always)]
    pub fn asynmode(&self) -> ASYNMODE_R {
        ASYNMODE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
    #[inline(always)]
    pub fn resol(&self) -> RESOL_R {
        RESOL_R::new(((self.bits() >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
    #[inline(always)]
    pub fn bypasscal(&self) -> BYPASSCAL_R {
        BYPASSCAL_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
    #[inline(always)]
    pub fn tsamp(&self) -> TSAMP_R {
        TSAMP_R::new(((self.bits() >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - In synchronous mode only, the system clock is divided by this value plus one to produce the clock for the ADC converter, which should be less than or equal to 72 MHz. Typically, software should program the smallest value in this field that yields this maximum clock rate or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable. This field is ignored in the asynchronous operating mode."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> _CLKDIVW {
        _CLKDIVW { w: self }
    }
    #[doc = "Bit 8 - Select clock mode."]
    #[inline(always)]
    pub fn asynmode(&mut self) -> _ASYNMODEW {
        _ASYNMODEW { w: self }
    }
    #[doc = "Bits 9:10 - The number of bits of ADC resolution. Accuracy can be reduced to achieve higher conversion rates. A single conversion (including one conversion in a burst or sequence) requires the selected number of bits of resolution plus 3 ADC clocks. This field must only be altered when the ADC is fully idle. Changing it during any kind of ADC operation may have unpredictable results. ADC clock frequencies for various resolutions must not exceed: - 5x the system clock rate for 12-bit resolution - 4.3x the system clock rate for 10-bit resolution - 3.6x the system clock for 8-bit resolution - 3x the bus clock rate for 6-bit resolution"]
    #[inline(always)]
    pub fn resol(&mut self) -> _RESOLW {
        _RESOLW { w: self }
    }
    #[doc = "Bit 11 - Bypass Calibration. This bit may be set to avoid the need to calibrate if offset error is not a concern in the application."]
    #[inline(always)]
    pub fn bypasscal(&mut self) -> _BYPASSCALW {
        _BYPASSCALW { w: self }
    }
    #[doc = "Bits 12:14 - Sample Time. The default sampling period (TSAMP = '000') at the start of each conversion is 2.5 ADC clock periods. Depending on a variety of factors, including operating conditions and the output impedance of the analog source, longer sampling times may be required. See Section 28.7.10. The TSAMP field specifies the number of additional ADC clock cycles, from zero to seven, by which the sample period will be extended. The total conversion time will increase by the same number of clocks. 000 - The sample period will be the default 2.5 ADC clocks. A complete conversion with 12-bits of accuracy will require 15 clocks. 001- The sample period will be extended by one ADC clock to a total of 3.5 clock periods. A complete 12-bit conversion will require 16 clocks. 010 - The sample period will be extended by two clocks to 4.5 ADC clock cycles. A complete 12-bit conversion will require 17 ADC clocks. 111 - The sample period will be extended by seven clocks to 9.5 ADC clock cycles. A complete 12-bit conversion will require 22 ADC clocks."]
    #[inline(always)]
    pub fn tsamp(&mut self) -> _TSAMPW {
        _TSAMPW { w: self }
    }
}
