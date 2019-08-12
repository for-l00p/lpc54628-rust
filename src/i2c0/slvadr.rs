#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVADR {
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
        0x01
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `SADISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADISABLER {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED,
}
impl crate::ToBits<bool> for SADISABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SADISABLER::ENABLED => false,
            SADISABLER::DISABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SADISABLE_R = crate::FR<bool, SADISABLER>;
impl SADISABLE_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SADISABLER::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SADISABLER::DISABLED
    }
}
#[doc = "Values that can be written to the field `SADISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SADISABLEW {
    #[doc = "Enabled. Slave Address n is enabled."]
    ENABLED,
    #[doc = "Ignored Slave Address n is ignored."]
    DISABLED,
}
impl SADISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SADISABLEW::ENABLED => false,
            SADISABLEW::DISABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SADISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SADISABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SADISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SADISABLEW::ENABLED)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SADISABLEW::DISABLED)
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
pub type SLVADR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SLVADRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVADRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `AUTONACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTONACKR {
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    NORMAL,
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC,
}
impl crate::ToBits<bool> for AUTONACKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AUTONACKR::NORMAL => false,
            AUTONACKR::AUTOMATIC => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AUTONACK_R = crate::FR<bool, AUTONACKR>;
impl AUTONACK_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == AUTONACKR::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTONACKR::AUTOMATIC
    }
}
#[doc = "Values that can be written to the field `AUTONACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTONACKW {
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    NORMAL,
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    AUTOMATIC,
}
impl AUTONACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTONACKW::NORMAL => false,
            AUTONACKW::AUTOMATIC => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AUTONACKW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTONACKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTONACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation, matching I2C addresses are not ignored."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AUTONACKW::NORMAL)
    }
    #[doc = "Automatic-only mode. All incoming addresses are ignored (NACKed), unless AUTOACK is set, it matches SLVADRn, and AUTOMATCHREAD matches the direction."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTONACKW::AUTOMATIC)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SADISABLE_R {
        SADISABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SLVADR_R {
        SLVADR_R::new(((self.bits() >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub fn autonack(&self) -> AUTONACK_R {
        AUTONACK_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&mut self) -> _SADISABLEW {
        _SADISABLEW { w: self }
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&mut self) -> _SLVADRW {
        _SLVADRW { w: self }
    }
    #[doc = "Bit 15 - Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub fn autonack(&mut self) -> _AUTONACKW {
        _AUTONACKW { w: self }
    }
}
