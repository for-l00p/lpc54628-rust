#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO219 {
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
        0x0320
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl crate::ToBits<u8> for MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODER::INACTIVE => 0,
            MODER::PULL_DOWN => 1,
            MODER::PULL_UP => 2,
            MODER::REPEATER => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MODE_R = crate::FR<u8, MODER>;
impl MODE_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODER::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODER::REPEATER
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. Repeater mode."]
    REPEATER,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::INACTIVE => 0,
            MODEW::PULL_DOWN => 1,
            MODEW::PULL_UP => 2,
            MODEW::REPEATER => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEW::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEW::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEW::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEW::REPEATER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
#[doc = "Possible values of the field `SLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEWR {
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    FAST,
}
impl crate::ToBits<bool> for SLEWR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLEWR::STANDARD => false,
            SLEWR::FAST => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLEW_R = crate::FR<bool, SLEWR>;
impl SLEW_R {
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SLEWR::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SLEWR::FAST
    }
}
#[doc = "Values that can be written to the field `SLEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEWW {
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    STANDARD,
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    FAST,
}
impl SLEWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEWW::STANDARD => false,
            SLEWW::FAST => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLEWW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEWW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEWW::STANDARD)
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SLEWW::FAST)
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
#[doc = "Possible values of the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODR {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN,
}
impl crate::ToBits<bool> for ODR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ODR::NORMAL => false,
            ODR::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OD_R = crate::FR<bool, ODR>;
impl OD_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODW {
    #[doc = "Normal. Normal push-pull output"]
    NORMAL,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN,
}
impl ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ODW::NORMAL => false,
            ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ODW<'a> {
    w: &'a mut W,
}
impl<'a> _ODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. Normal push-pull output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ODW::NORMAL)
    }
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(ODW::OPEN_DRAIN)
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
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits() >> 4) & 0x03) as u8)
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
    #[doc = "Bit 10 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&self) -> SLEW_R {
        SLEW_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Controls open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits() >> 11) & 0x01) != 0)
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
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
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
    #[doc = "Bit 10 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&mut self) -> _SLEWW {
        _SLEWW { w: self }
    }
    #[doc = "Bit 11 - Controls open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> _ODW {
        _ODW { w: self }
    }
}
