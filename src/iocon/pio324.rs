#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO324 {
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
        0x0340
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCR {
    #[doc = "Alternative connection 0."]
    ALT0,
    #[doc = "Alternative connection 1."]
    ALT1,
    #[doc = "Alternative connection 2."]
    ALT2,
    #[doc = "Alternative connection 3."]
    ALT3,
    #[doc = "Alternative connection 4."]
    ALT4,
    #[doc = "Alternative connection 5."]
    ALT5,
    #[doc = "Alternative connection 6."]
    ALT6,
    #[doc = "Alternative connection 7."]
    ALT7,
}
impl crate::ToBits<u8> for FUNCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FUNCR::ALT0 => 0,
            FUNCR::ALT1 => 1,
            FUNCR::ALT2 => 2,
            FUNCR::ALT3 => 3,
            FUNCR::ALT4 => 4,
            FUNCR::ALT5 => 5,
            FUNCR::ALT6 => 6,
            FUNCR::ALT7 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FUNC_R = crate::FR<u8, FUNCR>;
impl FUNC_R {
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == FUNCR::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == FUNCR::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == FUNCR::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == FUNCR::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == FUNCR::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == FUNCR::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == FUNCR::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == FUNCR::ALT7
    }
}
#[doc = "Values that can be written to the field `FUNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUNCW {
    #[doc = "Alternative connection 0."]
    ALT0,
    #[doc = "Alternative connection 1."]
    ALT1,
    #[doc = "Alternative connection 2."]
    ALT2,
    #[doc = "Alternative connection 3."]
    ALT3,
    #[doc = "Alternative connection 4."]
    ALT4,
    #[doc = "Alternative connection 5."]
    ALT5,
    #[doc = "Alternative connection 6."]
    ALT6,
    #[doc = "Alternative connection 7."]
    ALT7,
}
impl FUNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FUNCW::ALT0 => 0,
            FUNCW::ALT1 => 1,
            FUNCW::ALT2 => 2,
            FUNCW::ALT3 => 3,
            FUNCW::ALT4 => 4,
            FUNCW::ALT5 => 5,
            FUNCW::ALT6 => 6,
            FUNCW::ALT7 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Alternative connection 0."]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(FUNCW::ALT0)
    }
    #[doc = "Alternative connection 1."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(FUNCW::ALT1)
    }
    #[doc = "Alternative connection 2."]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(FUNCW::ALT2)
    }
    #[doc = "Alternative connection 3."]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(FUNCW::ALT3)
    }
    #[doc = "Alternative connection 4."]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(FUNCW::ALT4)
    }
    #[doc = "Alternative connection 5."]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(FUNCW::ALT5)
    }
    #[doc = "Alternative connection 6."]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(FUNCW::ALT6)
    }
    #[doc = "Alternative connection 7."]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(FUNCW::ALT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `I2CSLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CSLEWR {
    #[doc = "I2C mode."]
    I2C_MODE,
    #[doc = "GPIO mode."]
    GPIO_MODE,
}
impl crate::ToBits<bool> for I2CSLEWR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2CSLEWR::I2C_MODE => false,
            I2CSLEWR::GPIO_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2CSLEW_R = crate::FR<bool, I2CSLEWR>;
impl I2CSLEW_R {
    #[doc = "Checks if the value of the field is `I2C_MODE`"]
    #[inline(always)]
    pub fn is_i2c_mode(&self) -> bool {
        *self == I2CSLEWR::I2C_MODE
    }
    #[doc = "Checks if the value of the field is `GPIO_MODE`"]
    #[inline(always)]
    pub fn is_gpio_mode(&self) -> bool {
        *self == I2CSLEWR::GPIO_MODE
    }
}
#[doc = "Values that can be written to the field `I2CSLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CSLEWW {
    #[doc = "I2C mode."]
    I2C_MODE,
    #[doc = "GPIO mode."]
    GPIO_MODE,
}
impl I2CSLEWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2CSLEWW::I2C_MODE => false,
            I2CSLEWW::GPIO_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2CSLEWW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CSLEWW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CSLEWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C mode."]
    #[inline(always)]
    pub fn i2c_mode(self) -> &'a mut W {
        self.variant(I2CSLEWW::I2C_MODE)
    }
    #[doc = "GPIO mode."]
    #[inline(always)]
    pub fn gpio_mode(self) -> &'a mut W {
        self.variant(I2CSLEWW::GPIO_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `INVERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERTR {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED,
}
impl crate::ToBits<bool> for INVERTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INVERTR::DISABLED => false,
            INVERTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INVERT_R = crate::FR<bool, INVERTR>;
impl INVERT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INVERTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVERTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `INVERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERTW {
    #[doc = "Disabled. Input function is not inverted."]
    DISABLED,
    #[doc = "Enabled. Input is function inverted."]
    ENABLED,
}
impl INVERTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INVERTW::DISABLED => false,
            INVERTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _INVERTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVERTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Input function is not inverted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INVERTW::DISABLED)
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INVERTW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `DIGIMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODER {
    #[doc = "Analog mode."]
    ANALOG,
    #[doc = "Digital mode."]
    DIGITAL,
}
impl crate::ToBits<bool> for DIGIMODER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIGIMODER::ANALOG => false,
            DIGIMODER::DIGITAL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIGIMODE_R = crate::FR<bool, DIGIMODER>;
impl DIGIMODE_R {
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == DIGIMODER::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == DIGIMODER::DIGITAL
    }
}
#[doc = "Values that can be written to the field `DIGIMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODEW {
    #[doc = "Analog mode."]
    ANALOG,
    #[doc = "Digital mode."]
    DIGITAL,
}
impl DIGIMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIGIMODEW::ANALOG => false,
            DIGIMODEW::DIGITAL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIGIMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DIGIMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIGIMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog mode."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODEW::ANALOG)
    }
    #[doc = "Digital mode."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(DIGIMODEW::DIGITAL)
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
#[doc = "Possible values of the field `FILTEROFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEROFFR {
    #[doc = "Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    ENABLED,
    #[doc = "Filter disabled. No input filtering is done."]
    DISABLED,
}
impl crate::ToBits<bool> for FILTEROFFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FILTEROFFR::ENABLED => false,
            FILTEROFFR::DISABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FILTEROFF_R = crate::FR<bool, FILTEROFFR>;
impl FILTEROFF_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FILTEROFFR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FILTEROFFR::DISABLED
    }
}
#[doc = "Values that can be written to the field `FILTEROFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEROFFW {
    #[doc = "Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    ENABLED,
    #[doc = "Filter disabled. No input filtering is done."]
    DISABLED,
}
impl FILTEROFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTEROFFW::ENABLED => false,
            FILTEROFFW::DISABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FILTEROFFW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEROFFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTEROFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter enabled. Noise pulses below approximately 10 ns are filtered out."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTEROFFW::ENABLED)
    }
    #[doc = "Filter disabled. No input filtering is done."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTEROFFW::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `I2CDRIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CDRIVER {
    #[doc = "Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    LOW,
    #[doc = "High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details."]
    HIGH,
}
impl crate::ToBits<bool> for I2CDRIVER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2CDRIVER::LOW => false,
            I2CDRIVER::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2CDRIVE_R = crate::FR<bool, I2CDRIVER>;
impl I2CDRIVE_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == I2CDRIVER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == I2CDRIVER::HIGH
    }
}
#[doc = "Values that can be written to the field `I2CDRIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CDRIVEW {
    #[doc = "Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    LOW,
    #[doc = "High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details."]
    HIGH,
}
impl I2CDRIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2CDRIVEW::LOW => false,
            I2CDRIVEW::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2CDRIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CDRIVEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CDRIVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(I2CDRIVEW::LOW)
    }
    #[doc = "High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(I2CDRIVEW::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `I2CFILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CFILTERR {
    #[doc = "Enabled. I2C 50 ns glitch filter enabled."]
    ENABLED,
    #[doc = "Disabled. I2C 50 ns glitch filter disabled."]
    DISABLED,
}
impl crate::ToBits<bool> for I2CFILTERR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2CFILTERR::ENABLED => false,
            I2CFILTERR::DISABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2CFILTER_R = crate::FR<bool, I2CFILTERR>;
impl I2CFILTER_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2CFILTERR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2CFILTERR::DISABLED
    }
}
#[doc = "Values that can be written to the field `I2CFILTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CFILTERW {
    #[doc = "Enabled. I2C 50 ns glitch filter enabled."]
    ENABLED,
    #[doc = "Disabled. I2C 50 ns glitch filter disabled."]
    DISABLED,
}
impl I2CFILTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2CFILTERW::ENABLED => false,
            I2CFILTERW::DISABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2CFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CFILTERW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CFILTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. I2C 50 ns glitch filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2CFILTERW::ENABLED)
    }
    #[doc = "Disabled. I2C 50 ns glitch filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2CFILTERW::DISABLED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Controls slew rate of I2C pad."]
    #[inline(always)]
    pub fn i2cslew(&self) -> I2CSLEW_R {
        I2CSLEW_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select Analog/Digital mode."]
    #[inline(always)]
    pub fn digimode(&self) -> DIGIMODE_R {
        DIGIMODE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&self) -> FILTEROFF_R {
        FILTEROFF_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Controls the current sink capability of the pin."]
    #[inline(always)]
    pub fn i2cdrive(&self) -> I2CDRIVE_R {
        I2CDRIVE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn i2cfilter(&self) -> I2CFILTER_R {
        I2CFILTER_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&mut self) -> _FUNCW {
        _FUNCW { w: self }
    }
    #[doc = "Bit 6 - Controls slew rate of I2C pad."]
    #[inline(always)]
    pub fn i2cslew(&mut self) -> _I2CSLEWW {
        _I2CSLEWW { w: self }
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&mut self) -> _INVERTW {
        _INVERTW { w: self }
    }
    #[doc = "Bit 8 - Select Analog/Digital mode."]
    #[inline(always)]
    pub fn digimode(&mut self) -> _DIGIMODEW {
        _DIGIMODEW { w: self }
    }
    #[doc = "Bit 9 - Controls input glitch filter."]
    #[inline(always)]
    pub fn filteroff(&mut self) -> _FILTEROFFW {
        _FILTEROFFW { w: self }
    }
    #[doc = "Bit 10 - Controls the current sink capability of the pin."]
    #[inline(always)]
    pub fn i2cdrive(&mut self) -> _I2CDRIVEW {
        _I2CDRIVEW { w: self }
    }
    #[doc = "Bit 11 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn i2cfilter(&mut self) -> _I2CFILTERW {
        _I2CFILTERW { w: self }
    }
}
