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
#[doc = "Possible values of the field `MSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTENR {
    #[doc = "Disabled. The I2C Master function is disabled."]
    DISABLED,
    #[doc = "Enabled. The I2C Master function is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MSTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTENR::DISABLED => false,
            MSTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTEN_R = crate::FR<bool, MSTENR>;
impl MSTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTENW {
    #[doc = "Disabled. The I2C Master function is disabled."]
    DISABLED,
    #[doc = "Enabled. The I2C Master function is enabled."]
    ENABLED,
}
impl MSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTENW::DISABLED => false,
            MSTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The I2C Master function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTENW::DISABLED)
    }
    #[doc = "Enabled. The I2C Master function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTENW::ENABLED)
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
#[doc = "Possible values of the field `SLVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVENR {
    #[doc = "Disabled. The I2C slave function is disabled."]
    DISABLED,
    #[doc = "Enabled. The I2C slave function is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for SLVENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVENR::DISABLED => false,
            SLVENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVEN_R = crate::FR<bool, SLVENR>;
impl SLVEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SLVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVENW {
    #[doc = "Disabled. The I2C slave function is disabled."]
    DISABLED,
    #[doc = "Enabled. The I2C slave function is enabled."]
    ENABLED,
}
impl SLVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVENW::DISABLED => false,
            SLVENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVENW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The I2C slave function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVENW::DISABLED)
    }
    #[doc = "Enabled. The I2C slave function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVENW::ENABLED)
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
#[doc = "Possible values of the field `MONEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONENR {
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    DISABLED,
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for MONENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONENR::DISABLED => false,
            MONENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONEN_R = crate::FR<bool, MONENR>;
impl MONEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MONEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONENW {
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    DISABLED,
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    ENABLED,
}
impl MONENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONENW::DISABLED => false,
            MONENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONENW<'a> {
    w: &'a mut W,
}
impl<'a> _MONENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONENW::DISABLED)
    }
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONENW::ENABLED)
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
#[doc = "Possible values of the field `TIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTENR {
    #[doc = "Disabled. Time-out function is disabled."]
    DISABLED,
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    ENABLED,
}
impl crate::ToBits<bool> for TIMEOUTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TIMEOUTENR::DISABLED => false,
            TIMEOUTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TIMEOUTEN_R = crate::FR<bool, TIMEOUTENR>;
impl TIMEOUTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMEOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMEOUTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `TIMEOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTENW {
    #[doc = "Disabled. Time-out function is disabled."]
    DISABLED,
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    ENABLED,
}
impl TIMEOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMEOUTENW::DISABLED => false,
            TIMEOUTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMEOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Time-out function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMEOUTENW::DISABLED)
    }
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMEOUTENW::ENABLED)
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
#[doc = "Possible values of the field `MONCLKSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONCLKSTRR {
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    DISABLED,
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    ENABLED,
}
impl crate::ToBits<bool> for MONCLKSTRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONCLKSTRR::DISABLED => false,
            MONCLKSTRR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONCLKSTR_R = crate::FR<bool, MONCLKSTRR>;
impl MONCLKSTR_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONCLKSTRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONCLKSTRR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MONCLKSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONCLKSTRW {
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    DISABLED,
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    ENABLED,
}
impl MONCLKSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONCLKSTRW::DISABLED => false,
            MONCLKSTRW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONCLKSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MONCLKSTRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONCLKSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONCLKSTRW::DISABLED)
    }
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONCLKSTRW::ENABLED)
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
#[doc = "Possible values of the field `HSCAPABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSCAPABLER {
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    FAST_MODE_PLUS,
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HIGH_SPEED,
}
impl crate::ToBits<bool> for HSCAPABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HSCAPABLER::FAST_MODE_PLUS => false,
            HSCAPABLER::HIGH_SPEED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HSCAPABLE_R = crate::FR<bool, HSCAPABLER>;
impl HSCAPABLE_R {
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == HSCAPABLER::FAST_MODE_PLUS
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == HSCAPABLER::HIGH_SPEED
    }
}
#[doc = "Values that can be written to the field `HSCAPABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSCAPABLEW {
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    FAST_MODE_PLUS,
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HIGH_SPEED,
}
impl HSCAPABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HSCAPABLEW::FAST_MODE_PLUS => false,
            HSCAPABLEW::HIGH_SPEED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HSCAPABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _HSCAPABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSCAPABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(HSCAPABLEW::FAST_MODE_PLUS)
    }
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(HSCAPABLEW::HIGH_SPEED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&self) -> TIMEOUTEN_R {
        TIMEOUTEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&self) -> MONCLKSTR_R {
        MONCLKSTR_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&self) -> HSCAPABLE_R {
        HSCAPABLE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&mut self) -> _MSTENW {
        _MSTENW { w: self }
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&mut self) -> _SLVENW {
        _SLVENW { w: self }
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&mut self) -> _MONENW {
        _MONENW { w: self }
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&mut self) -> _TIMEOUTENW {
        _TIMEOUTENW { w: self }
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&mut self) -> _MONCLKSTRW {
        _MONCLKSTRW { w: self }
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&mut self) -> _HSCAPABLEW {
        _HSCAPABLEW { w: self }
    }
}
