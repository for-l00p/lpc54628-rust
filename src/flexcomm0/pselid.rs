#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSELID {
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
        0x0010_1000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `PERSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERSELR {
    #[doc = "No peripheral selected."]
    NO_PERIPH_SELECTED,
    #[doc = "USART function selected."]
    USART,
    #[doc = "SPI function selected."]
    SPI,
    #[doc = "I2C function selected."]
    I2C,
    #[doc = "I2S transmit function selected."]
    I2S_TRANSMIT,
    #[doc = "I2S receive function selected."]
    I2S_RECEIVE,
}
impl crate::ToBits<u8> for PERSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PERSELR::NO_PERIPH_SELECTED => 0,
            PERSELR::USART => 1,
            PERSELR::SPI => 2,
            PERSELR::I2C => 3,
            PERSELR::I2S_TRANSMIT => 4,
            PERSELR::I2S_RECEIVE => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PERSEL_R = crate::FR<u8, PERSELR>;
impl PERSEL_R {
    #[doc = "Checks if the value of the field is `NO_PERIPH_SELECTED`"]
    #[inline(always)]
    pub fn is_no_periph_selected(&self) -> bool {
        *self == PERSELR::NO_PERIPH_SELECTED
    }
    #[doc = "Checks if the value of the field is `USART`"]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == PERSELR::USART
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == PERSELR::SPI
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == PERSELR::I2C
    }
    #[doc = "Checks if the value of the field is `I2S_TRANSMIT`"]
    #[inline(always)]
    pub fn is_i2s_transmit(&self) -> bool {
        *self == PERSELR::I2S_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `I2S_RECEIVE`"]
    #[inline(always)]
    pub fn is_i2s_receive(&self) -> bool {
        *self == PERSELR::I2S_RECEIVE
    }
}
#[doc = "Values that can be written to the field `PERSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERSELW {
    #[doc = "No peripheral selected."]
    NO_PERIPH_SELECTED,
    #[doc = "USART function selected."]
    USART,
    #[doc = "SPI function selected."]
    SPI,
    #[doc = "I2C function selected."]
    I2C,
    #[doc = "I2S transmit function selected."]
    I2S_TRANSMIT,
    #[doc = "I2S receive function selected."]
    I2S_RECEIVE,
}
impl PERSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERSELW::NO_PERIPH_SELECTED => 0,
            PERSELW::USART => 1,
            PERSELW::SPI => 2,
            PERSELW::I2C => 3,
            PERSELW::I2S_TRANSMIT => 4,
            PERSELW::I2S_RECEIVE => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PERSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PERSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No peripheral selected."]
    #[inline(always)]
    pub fn no_periph_selected(self) -> &'a mut W {
        self.variant(PERSELW::NO_PERIPH_SELECTED)
    }
    #[doc = "USART function selected."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut W {
        self.variant(PERSELW::USART)
    }
    #[doc = "SPI function selected."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(PERSELW::SPI)
    }
    #[doc = "I2C function selected."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(PERSELW::I2C)
    }
    #[doc = "I2S transmit function selected."]
    #[inline(always)]
    pub fn i2s_transmit(self) -> &'a mut W {
        self.variant(PERSELW::I2S_TRANSMIT)
    }
    #[doc = "I2S receive function selected."]
    #[inline(always)]
    pub fn i2s_receive(self) -> &'a mut W {
        self.variant(PERSELW::I2S_RECEIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "Peripheral select can be changed by software."]
    UNLOCKED,
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED,
}
impl crate::ToBits<bool> for LOCKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LOCKR::UNLOCKED => false,
            LOCKR::LOCKED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_R = crate::FR<bool, LOCKR>;
impl LOCK_R {
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::LOCKED
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKW {
    #[doc = "Peripheral select can be changed by software."]
    UNLOCKED,
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    LOCKED,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::UNLOCKED => false,
            LOCKW::LOCKED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral select can be changed by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKW::UNLOCKED)
    }
    #[doc = "Peripheral select is locked and cannot be changed until this Flexcomm or the entire device is reset."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKW::LOCKED)
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
#[doc = "Possible values of the field `USARTPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USARTPRESENTR {
    #[doc = "This Flexcomm does not include the USART function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the USART function."]
    PRESENT,
}
impl crate::ToBits<bool> for USARTPRESENTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USARTPRESENTR::NOT_PRESENT => false,
            USARTPRESENTR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USARTPRESENT_R = crate::FR<bool, USARTPRESENTR>;
impl USARTPRESENT_R {
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USARTPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == USARTPRESENTR::PRESENT
    }
}
#[doc = "Possible values of the field `SPIPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIPRESENTR {
    #[doc = "This Flexcomm does not include the SPI function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the SPI function."]
    PRESENT,
}
impl crate::ToBits<bool> for SPIPRESENTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SPIPRESENTR::NOT_PRESENT => false,
            SPIPRESENTR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SPIPRESENT_R = crate::FR<bool, SPIPRESENTR>;
impl SPIPRESENT_R {
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == SPIPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == SPIPRESENTR::PRESENT
    }
}
#[doc = "Possible values of the field `I2CPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CPRESENTR {
    #[doc = "This Flexcomm does not include the I2C function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the I2C function."]
    PRESENT,
}
impl crate::ToBits<bool> for I2CPRESENTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2CPRESENTR::NOT_PRESENT => false,
            I2CPRESENTR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2CPRESENT_R = crate::FR<bool, I2CPRESENTR>;
impl I2CPRESENT_R {
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2CPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2CPRESENTR::PRESENT
    }
}
#[doc = "Possible values of the field `I2SPRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SPRESENTR {
    #[doc = "This Flexcomm does not include the I2S function."]
    NOT_PRESENT,
    #[doc = "This Flexcomm includes the I2S function."]
    PRESENT,
}
impl crate::ToBits<bool> for I2SPRESENTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2SPRESENTR::NOT_PRESENT => false,
            I2SPRESENTR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2SPRESENT_R = crate::FR<bool, I2SPRESENTR>;
impl I2SPRESENT_R {
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == I2SPRESENTR::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == I2SPRESENTR::PRESENT
    }
}
#[doc = r"Reader of the field"]
pub type ID_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&self) -> PERSEL_R {
        PERSEL_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn usartpresent(&self) -> USARTPRESENT_R {
        USARTPRESENT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn spipresent(&self) -> SPIPRESENT_R {
        SPIPRESENT_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2cpresent(&self) -> I2CPRESENT_R {
        I2CPRESENT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn i2spresent(&self) -> I2SPRESENT_R {
        I2SPRESENT_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 12:31 - Flexcomm ID."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits() >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn persel(&mut self) -> _PERSELW {
        _PERSELW { w: self }
    }
    #[doc = "Bit 3 - Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
