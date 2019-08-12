#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `SSAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAENR {
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    ENABLED,
}
impl crate::ToBits<bool> for SSAENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSAENR::DISABLED => false,
            SSAENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSAEN_R = crate::FR<bool, SSAENR>;
impl SSAEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSAENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SSAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAENW {
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    ENABLED,
}
impl SSAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSAENW::DISABLED => false,
            SSAENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSAENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSAENW::DISABLED)
    }
    #[doc = "Enabled. An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSAENW::ENABLED)
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
#[doc = "Possible values of the field `SSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDENR {
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    ENABLED,
}
impl crate::ToBits<bool> for SSDENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSDENR::DISABLED => false,
            SSDENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSDEN_R = crate::FR<bool, SSDENR>;
impl SSDEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSDENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDENW {
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    DISABLED,
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    ENABLED,
}
impl SSDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSDENW::DISABLED => false,
            SSDENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSDENW::DISABLED)
    }
    #[doc = "Enabled. An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSDENW::ENABLED)
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
#[doc = "Possible values of the field `MSTIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTIDLEENR {
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    DISABLED,
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    ENABLED,
}
impl crate::ToBits<bool> for MSTIDLEENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTIDLEENR::DISABLED => false,
            MSTIDLEENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTIDLEEN_R = crate::FR<bool, MSTIDLEENR>;
impl MSTIDLEEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTIDLEENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTIDLEENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTIDLEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTIDLEENW {
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    DISABLED,
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    ENABLED,
}
impl MSTIDLEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTIDLEENW::DISABLED => false,
            MSTIDLEENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTIDLEENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTIDLEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when the SPI master function is idle."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTIDLEENW::DISABLED)
    }
    #[doc = "An interrupt will be generated when the SPI master function is fully idle."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTIDLEENW::ENABLED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&self) -> SSAEN_R {
        SSAEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&self) -> SSDEN_R {
        SSDEN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline(always)]
    pub fn mstidleen(&self) -> MSTIDLEEN_R {
        MSTIDLEEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn ssaen(&mut self) -> _SSAENW {
        _SSAENW { w: self }
    }
    #[doc = "Bit 5 - Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn ssden(&mut self) -> _SSDENW {
        _SSDENW { w: self }
    }
    #[doc = "Bit 8 - Master idle interrupt enable."]
    #[inline(always)]
    pub fn mstidleen(&mut self) -> _MSTIDLEENW {
        _MSTIDLEENW { w: self }
    }
}
