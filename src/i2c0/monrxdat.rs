#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MONRXDAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONRXDAT_R = crate::FR<u8, u8>;
#[doc = "Possible values of the field `MONSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONSTARTR {
    #[doc = "No start detected. The Monitor function has not detected a Start event on the I2C bus."]
    NO_START_DETECTED,
    #[doc = "Start detected. The Monitor function has detected a Start event on the I2C bus."]
    START_DETECTED,
}
impl crate::ToBits<bool> for MONSTARTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONSTARTR::NO_START_DETECTED => false,
            MONSTARTR::START_DETECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONSTART_R = crate::FR<bool, MONSTARTR>;
impl MONSTART_R {
    #[doc = "Checks if the value of the field is `NO_START_DETECTED`"]
    #[inline(always)]
    pub fn is_no_start_detected(&self) -> bool {
        *self == MONSTARTR::NO_START_DETECTED
    }
    #[doc = "Checks if the value of the field is `START_DETECTED`"]
    #[inline(always)]
    pub fn is_start_detected(&self) -> bool {
        *self == MONSTARTR::START_DETECTED
    }
}
#[doc = "Possible values of the field `MONRESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRESTARTR {
    #[doc = "No repeated start detected. The Monitor function has not detected a Repeated Start event on the I2C bus."]
    NOT_DETECTED,
    #[doc = "Repeated start detected. The Monitor function has detected a Repeated Start event on the I2C bus."]
    DETECTED,
}
impl crate::ToBits<bool> for MONRESTARTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONRESTARTR::NOT_DETECTED => false,
            MONRESTARTR::DETECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONRESTART_R = crate::FR<bool, MONRESTARTR>;
impl MONRESTART_R {
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == MONRESTARTR::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == MONRESTARTR::DETECTED
    }
}
#[doc = "Possible values of the field `MONNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONNACKR {
    #[doc = "Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    ACKNOWLEDGED,
    #[doc = "Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    NOT_ACKNOWLEDGED,
}
impl crate::ToBits<bool> for MONNACKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONNACKR::ACKNOWLEDGED => false,
            MONNACKR::NOT_ACKNOWLEDGED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONNACK_R = crate::FR<bool, MONNACKR>;
impl MONNACK_R {
    #[doc = "Checks if the value of the field is `ACKNOWLEDGED`"]
    #[inline(always)]
    pub fn is_acknowledged(&self) -> bool {
        *self == MONNACKR::ACKNOWLEDGED
    }
    #[doc = "Checks if the value of the field is `NOT_ACKNOWLEDGED`"]
    #[inline(always)]
    pub fn is_not_acknowledged(&self) -> bool {
        *self == MONNACKR::NOT_ACKNOWLEDGED
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub fn monrxdat(&self) -> MONRXDAT_R {
        MONRXDAT_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bit 8 - Monitor Received Start."]
    #[inline(always)]
    pub fn monstart(&self) -> MONSTART_R {
        MONSTART_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Monitor Received Repeated Start."]
    #[inline(always)]
    pub fn monrestart(&self) -> MONRESTART_R {
        MONRESTART_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Monitor Received NACK."]
    #[inline(always)]
    pub fn monnack(&self) -> MONNACK_R {
        MONNACK_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
}
