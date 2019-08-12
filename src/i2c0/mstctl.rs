#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSTCTL {
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
#[doc = "Values that can be written to the field `MSTCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCONTINUEW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE,
}
impl MSTCONTINUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTCONTINUEW::NO_EFFECT => false,
            MSTCONTINUEW::CONTINUE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTCONTINUEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTCONTINUEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCONTINUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCONTINUEW::NO_EFFECT)
    }
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(MSTCONTINUEW::CONTINUE)
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
#[doc = "Possible values of the field `MSTSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTARTR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START,
}
impl crate::ToBits<bool> for MSTSTARTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTSTARTR::NO_EFFECT => false,
            MSTSTARTR::START => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTSTART_R = crate::FR<bool, MSTSTARTR>;
impl MSTSTART_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTARTR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MSTSTARTR::START
    }
}
#[doc = "Values that can be written to the field `MSTSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTARTW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    START,
}
impl MSTSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTARTW::NO_EFFECT => false,
            MSTSTARTW::START => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTARTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTARTW::NO_EFFECT)
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MSTSTARTW::START)
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
#[doc = "Possible values of the field `MSTSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOPR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP,
}
impl crate::ToBits<bool> for MSTSTOPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTSTOPR::NO_EFFECT => false,
            MSTSTOPR::STOP => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTSTOP_R = crate::FR<bool, MSTSTOPR>;
impl MSTSTOP_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTOPR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MSTSTOPR::STOP
    }
}
#[doc = "Values that can be written to the field `MSTSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTOPW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP,
}
impl MSTSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTOPW::NO_EFFECT => false,
            MSTSTOPW::STOP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTOPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTOPW::NO_EFFECT)
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MSTSTOPW::STOP)
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
#[doc = "Possible values of the field `MSTDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTDMAR {
    #[doc = "Disable. No DMA requests are generated for master operation."]
    DISABLED,
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    ENABLED,
}
impl crate::ToBits<bool> for MSTDMAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTDMAR::DISABLED => false,
            MSTDMAR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTDMA_R = crate::FR<bool, MSTDMAR>;
impl MSTDMA_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTDMAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTDMAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MSTDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTDMAW {
    #[doc = "Disable. No DMA requests are generated for master operation."]
    DISABLED,
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    ENABLED,
}
impl MSTDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTDMAW::DISABLED => false,
            MSTDMAW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTDMAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable. No DMA requests are generated for master operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTDMAW::DISABLED)
    }
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTDMAW::ENABLED)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&self) -> MSTSTART_R {
        MSTSTART_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&self) -> MSTSTOP_R {
        MSTSTOP_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&self) -> MSTDMA_R {
        MSTDMA_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline(always)]
    pub fn mstcontinue(&mut self) -> _MSTCONTINUEW {
        _MSTCONTINUEW { w: self }
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&mut self) -> _MSTSTARTW {
        _MSTSTARTW { w: self }
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&mut self) -> _MSTSTOPW {
        _MSTSTOPW { w: self }
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&mut self) -> _MSTDMAW {
        _MSTDMAW { w: self }
    }
}
