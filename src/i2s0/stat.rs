#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    IDLE,
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    BUSY,
}
impl crate::ToBits<bool> for BUSYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BUSYR::IDLE => false,
            BUSYR::BUSY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BUSY_R = crate::FR<bool, BUSYR>;
impl BUSY_R {
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSYR::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSYR::BUSY
    }
}
#[doc = "Values that can be written to the field `SLVFRMERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVFRMERRW {
    #[doc = "No error has been recorded."]
    NO_ERROR,
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    ERROR,
}
impl SLVFRMERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVFRMERRW::NO_ERROR => false,
            SLVFRMERRW::ERROR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVFRMERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVFRMERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVFRMERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No error has been recorded."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(SLVFRMERRW::NO_ERROR)
    }
    #[doc = "An error has been recorded for some channel pair that is operating in slave mode. ERROR is cleared by writing a 1 to this bit position."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(SLVFRMERRW::ERROR)
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
#[doc = "Possible values of the field `LR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRR {
    #[doc = "Left channel."]
    LEFT_CHANNEL,
    #[doc = "Right channel."]
    RIGHT_CHANNEL,
}
impl crate::ToBits<bool> for LRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LRR::LEFT_CHANNEL => false,
            LRR::RIGHT_CHANNEL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LR_R = crate::FR<bool, LRR>;
impl LR_R {
    #[doc = "Checks if the value of the field is `LEFT_CHANNEL`"]
    #[inline(always)]
    pub fn is_left_channel(&self) -> bool {
        *self == LRR::LEFT_CHANNEL
    }
    #[doc = "Checks if the value of the field is `RIGHT_CHANNEL`"]
    #[inline(always)]
    pub fn is_right_channel(&self) -> bool {
        *self == LRR::RIGHT_CHANNEL
    }
}
#[doc = "Possible values of the field `DATAPAUSED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAPAUSEDR {
    #[doc = "Data is not currently paused. A data pause may have been requested but is not yet in force, waiting for an allowed pause point. Refer to the description of the DATAPAUSE control bit in the CFG1 register."]
    NOT_PAUSED,
    #[doc = "A data pause has been requested and is now in force."]
    PAUSED,
}
impl crate::ToBits<bool> for DATAPAUSEDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DATAPAUSEDR::NOT_PAUSED => false,
            DATAPAUSEDR::PAUSED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DATAPAUSED_R = crate::FR<bool, DATAPAUSEDR>;
impl DATAPAUSED_R {
    #[doc = "Checks if the value of the field is `NOT_PAUSED`"]
    #[inline(always)]
    pub fn is_not_paused(&self) -> bool {
        *self == DATAPAUSEDR::NOT_PAUSED
    }
    #[doc = "Checks if the value of the field is `PAUSED`"]
    #[inline(always)]
    pub fn is_paused(&self) -> bool {
        *self == DATAPAUSEDR::PAUSED
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Busy status for the primary channel pair. Other BUSY flags may be found in the STAT register for each channel pair."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - Left/Right indication. This flag is considered to be a debugging aid and is not expected to be used by an I2S driver. Valid when one channel pair is busy. Indicates left or right data being processed for the currently busy channel pair."]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Paused status flag. Applies to all I2S channels"]
    #[inline(always)]
    pub fn datapaused(&self) -> DATAPAUSED_R {
        DATAPAUSED_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Slave Frame Error flag. This applies when at least one channel pair is operating as a slave. An error indicates that the incoming WS signal did not transition as expected due to a mismatch between FRAMELEN and the actual incoming I2S stream."]
    #[inline(always)]
    pub fn slvfrmerr(&mut self) -> _SLVFRMERRW {
        _SLVFRMERRW { w: self }
    }
}
