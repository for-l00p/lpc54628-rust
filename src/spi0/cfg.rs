#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    DISABLED,
    #[doc = "Enabled. The SPI is enabled for operation."]
    ENABLED,
}
impl crate::ToBits<bool> for ENABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENABLER::DISABLED => false,
            ENABLER::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLE_R = crate::FR<bool, ENABLER>;
impl ENABLE_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLER::ENABLED
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLEW {
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    DISABLED,
    #[doc = "Enabled. The SPI is enabled for operation."]
    ENABLED,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::DISABLED => false,
            ENABLEW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED)
    }
    #[doc = "Enabled. The SPI is enabled for operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED)
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
#[doc = "Possible values of the field `MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERR {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE,
}
impl crate::ToBits<bool> for MASTERR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MASTERR::SLAVE_MODE => false,
            MASTERR::MASTER_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MASTER_R = crate::FR<bool, MASTERR>;
impl MASTER_R {
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MASTERR::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MASTERR::MASTER_MODE
    }
}
#[doc = "Values that can be written to the field `MASTER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTERW {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE,
}
impl MASTERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MASTERW::SLAVE_MODE => false,
            MASTERW::MASTER_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MASTERW<'a> {
    w: &'a mut W,
}
impl<'a> _MASTERW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MASTERW::SLAVE_MODE)
    }
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MASTERW::MASTER_MODE)
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
#[doc = "Possible values of the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFR {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE,
}
impl crate::ToBits<bool> for LSBFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LSBFR::STANDARD => false,
            LSBFR::REVERSE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LSBF_R = crate::FR<bool, LSBFR>;
impl LSBF_R {
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == LSBFR::STANDARD
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LSBFR::REVERSE
    }
}
#[doc = "Values that can be written to the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFW {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE,
}
impl LSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFW::STANDARD => false,
            LSBFW::REVERSE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(LSBFW::STANDARD)
    }
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut W {
        self.variant(LSBFW::REVERSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE,
}
impl crate::ToBits<bool> for CPHAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CPHAR::CHANGE => false,
            CPHAR::CAPTURE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CPHA_R = crate::FR<bool, CPHAR>;
impl CPHA_R {
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == CPHAR::CHANGE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == CPHAR::CAPTURE
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAW {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::CHANGE => false,
            CPHAW::CAPTURE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    #[inline(always)]
    pub fn change(self) -> &'a mut W {
        self.variant(CPHAW::CHANGE)
    }
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(CPHAW::CAPTURE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    LOW,
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    HIGH,
}
impl crate::ToBits<bool> for CPOLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CPOLR::LOW => false,
            CPOLR::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CPOL_R = crate::FR<bool, CPOLR>;
impl CPOL_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOLR::HIGH
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLW {
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    LOW,
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::LOW => false,
            CPOLW::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOLW::LOW)
    }
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOLW::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPR {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for LOOPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LOOPR::DISABLED => false,
            LOOPR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LOOP_R = crate::FR<bool, LOOPR>;
impl LOOP_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Enabled."]
    ENABLED,
}
impl LOOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPW::DISABLED => false,
            LOOPW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LOOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPW::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPW::ENABLED)
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
#[doc = "Possible values of the field `SPOL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL0R {
    #[doc = "Low. The SSEL0 pin is active low."]
    LOW,
    #[doc = "High. The SSEL0 pin is active high."]
    HIGH,
}
impl crate::ToBits<bool> for SPOL0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SPOL0R::LOW => false,
            SPOL0R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SPOL0_R = crate::FR<bool, SPOL0R>;
impl SPOL0_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL0R::HIGH
    }
}
#[doc = "Values that can be written to the field `SPOL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL0W {
    #[doc = "Low. The SSEL0 pin is active low."]
    LOW,
    #[doc = "High. The SSEL0 pin is active high."]
    HIGH,
}
impl SPOL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOL0W::LOW => false,
            SPOL0W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SPOL0W<'a> {
    w: &'a mut W,
}
impl<'a> _SPOL0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The SSEL0 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL0W::LOW)
    }
    #[doc = "High. The SSEL0 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL0W::HIGH)
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
#[doc = "Possible values of the field `SPOL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL1R {
    #[doc = "Low. The SSEL1 pin is active low."]
    LOW,
    #[doc = "High. The SSEL1 pin is active high."]
    HIGH,
}
impl crate::ToBits<bool> for SPOL1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SPOL1R::LOW => false,
            SPOL1R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SPOL1_R = crate::FR<bool, SPOL1R>;
impl SPOL1_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL1R::HIGH
    }
}
#[doc = "Values that can be written to the field `SPOL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL1W {
    #[doc = "Low. The SSEL1 pin is active low."]
    LOW,
    #[doc = "High. The SSEL1 pin is active high."]
    HIGH,
}
impl SPOL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOL1W::LOW => false,
            SPOL1W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SPOL1W<'a> {
    w: &'a mut W,
}
impl<'a> _SPOL1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The SSEL1 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL1W::LOW)
    }
    #[doc = "High. The SSEL1 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL1W::HIGH)
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
#[doc = "Possible values of the field `SPOL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL2R {
    #[doc = "Low. The SSEL2 pin is active low."]
    LOW,
    #[doc = "High. The SSEL2 pin is active high."]
    HIGH,
}
impl crate::ToBits<bool> for SPOL2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SPOL2R::LOW => false,
            SPOL2R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SPOL2_R = crate::FR<bool, SPOL2R>;
impl SPOL2_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL2R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL2R::HIGH
    }
}
#[doc = "Values that can be written to the field `SPOL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL2W {
    #[doc = "Low. The SSEL2 pin is active low."]
    LOW,
    #[doc = "High. The SSEL2 pin is active high."]
    HIGH,
}
impl SPOL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOL2W::LOW => false,
            SPOL2W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SPOL2W<'a> {
    w: &'a mut W,
}
impl<'a> _SPOL2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The SSEL2 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL2W::LOW)
    }
    #[doc = "High. The SSEL2 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL2W::HIGH)
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
#[doc = "Possible values of the field `SPOL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL3R {
    #[doc = "Low. The SSEL3 pin is active low."]
    LOW,
    #[doc = "High. The SSEL3 pin is active high."]
    HIGH,
}
impl crate::ToBits<bool> for SPOL3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SPOL3R::LOW => false,
            SPOL3R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SPOL3_R = crate::FR<bool, SPOL3R>;
impl SPOL3_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL3R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL3R::HIGH
    }
}
#[doc = "Values that can be written to the field `SPOL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL3W {
    #[doc = "Low. The SSEL3 pin is active low."]
    LOW,
    #[doc = "High. The SSEL3 pin is active high."]
    HIGH,
}
impl SPOL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOL3W::LOW => false,
            SPOL3W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SPOL3W<'a> {
    w: &'a mut W,
}
impl<'a> _SPOL3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The SSEL3 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL3W::LOW)
    }
    #[doc = "High. The SSEL3 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL3W::HIGH)
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
    #[doc = "Bit 0 - SPI enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Phase select."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SSEL0 Polarity select."]
    #[inline(always)]
    pub fn spol0(&self) -> SPOL0_R {
        SPOL0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SSEL1 Polarity select."]
    #[inline(always)]
    pub fn spol1(&self) -> SPOL1_R {
        SPOL1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SSEL2 Polarity select."]
    #[inline(always)]
    pub fn spol2(&self) -> SPOL2_R {
        SPOL2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SSEL3 Polarity select."]
    #[inline(always)]
    pub fn spol3(&self) -> SPOL3_R {
        SPOL3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SPI enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline(always)]
    pub fn master(&mut self) -> _MASTERW {
        _MASTERW { w: self }
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline(always)]
    pub fn lsbf(&mut self) -> _LSBFW {
        _LSBFW { w: self }
    }
    #[doc = "Bit 4 - Clock Phase select."]
    #[inline(always)]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline(always)]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub fn loop_(&mut self) -> _LOOPW {
        _LOOPW { w: self }
    }
    #[doc = "Bit 8 - SSEL0 Polarity select."]
    #[inline(always)]
    pub fn spol0(&mut self) -> _SPOL0W {
        _SPOL0W { w: self }
    }
    #[doc = "Bit 9 - SSEL1 Polarity select."]
    #[inline(always)]
    pub fn spol1(&mut self) -> _SPOL1W {
        _SPOL1W { w: self }
    }
    #[doc = "Bit 10 - SSEL2 Polarity select."]
    #[inline(always)]
    pub fn spol2(&mut self) -> _SPOL2W {
        _SPOL2W { w: self }
    }
    #[doc = "Bit 11 - SSEL3 Polarity select."]
    #[inline(always)]
    pub fn spol3(&mut self) -> _SPOL3W {
        _SPOL3W { w: self }
    }
}
