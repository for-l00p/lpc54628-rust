#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICSR {
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
#[doc = r"Reader of the field"]
pub type VECTACTIVE_R = crate::FR<u16, u16>;
#[doc = "Possible values of the field `RETTOBASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETTOBASER {
    #[doc = "there are preempted active exceptions to execute"]
    RETTOBASE_0,
    #[doc = "there are no active exceptions, or the currently-executing exception is the only active exception"]
    RETTOBASE_1,
}
impl crate::ToBits<bool> for RETTOBASER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RETTOBASER::RETTOBASE_0 => false,
            RETTOBASER::RETTOBASE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RETTOBASE_R = crate::FR<bool, RETTOBASER>;
impl RETTOBASE_R {
    #[doc = "Checks if the value of the field is `RETTOBASE_0`"]
    #[inline(always)]
    pub fn is_rettobase_0(&self) -> bool {
        *self == RETTOBASER::RETTOBASE_0
    }
    #[doc = "Checks if the value of the field is `RETTOBASE_1`"]
    #[inline(always)]
    pub fn is_rettobase_1(&self) -> bool {
        *self == RETTOBASER::RETTOBASE_1
    }
}
#[doc = r"Reader of the field"]
pub type VECTPENDING_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type ISRPENDING_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `ISRPREEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISRPREEMPTR {
    #[doc = "Will not service"]
    ISRPREEMPT_0,
    #[doc = "Will service a pending exception"]
    ISRPREEMPT_1,
}
impl crate::ToBits<bool> for ISRPREEMPTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ISRPREEMPTR::ISRPREEMPT_0 => false,
            ISRPREEMPTR::ISRPREEMPT_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ISRPREEMPT_R = crate::FR<bool, ISRPREEMPTR>;
impl ISRPREEMPT_R {
    #[doc = "Checks if the value of the field is `ISRPREEMPT_0`"]
    #[inline(always)]
    pub fn is_isrpreempt_0(&self) -> bool {
        *self == ISRPREEMPTR::ISRPREEMPT_0
    }
    #[doc = "Checks if the value of the field is `ISRPREEMPT_1`"]
    #[inline(always)]
    pub fn is_isrpreempt_1(&self) -> bool {
        *self == ISRPREEMPTR::ISRPREEMPT_1
    }
}
#[doc = "Values that can be written to the field `PENDSTCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTCLRW {
    #[doc = "no effect"]
    PENDSTCLR_0,
    #[doc = "removes the pending state from the SysTick exception"]
    PENDSTCLR_1,
}
impl PENDSTCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTCLRW::PENDSTCLR_0 => false,
            PENDSTCLRW::PENDSTCLR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PENDSTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn pendstclr_0(self) -> &'a mut W {
        self.variant(PENDSTCLRW::PENDSTCLR_0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn pendstclr_1(self) -> &'a mut W {
        self.variant(PENDSTCLRW::PENDSTCLR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETR {
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    PENDSTSET_0,
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    PENDSTSET_1,
}
impl crate::ToBits<bool> for PENDSTSETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PENDSTSETR::PENDSTSET_0 => false,
            PENDSTSETR::PENDSTSET_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PENDSTSET_R = crate::FR<bool, PENDSTSETR>;
impl PENDSTSET_R {
    #[doc = "Checks if the value of the field is `PENDSTSET_0`"]
    #[inline(always)]
    pub fn is_pendstset_0(&self) -> bool {
        *self == PENDSTSETR::PENDSTSET_0
    }
    #[doc = "Checks if the value of the field is `PENDSTSET_1`"]
    #[inline(always)]
    pub fn is_pendstset_1(&self) -> bool {
        *self == PENDSTSETR::PENDSTSET_1
    }
}
#[doc = "Values that can be written to the field `PENDSTSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSTSETW {
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    PENDSTSET_0,
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    PENDSTSET_1,
}
impl PENDSTSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSTSETW::PENDSTSET_0 => false,
            PENDSTSETW::PENDSTSET_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PENDSTSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSTSETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSTSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn pendstset_0(self) -> &'a mut W {
        self.variant(PENDSTSETW::PENDSTSET_0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn pendstset_1(self) -> &'a mut W {
        self.variant(PENDSTSETW::PENDSTSET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Values that can be written to the field `PENDSVCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVCLRW {
    #[doc = "no effect"]
    PENDSVCLR_0,
    #[doc = "removes the pending state from the PendSV exception"]
    PENDSVCLR_1,
}
impl PENDSVCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVCLRW::PENDSVCLR_0 => false,
            PENDSVCLRW::PENDSVCLR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PENDSVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn pendsvclr_0(self) -> &'a mut W {
        self.variant(PENDSVCLRW::PENDSVCLR_0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn pendsvclr_1(self) -> &'a mut W {
        self.variant(PENDSVCLRW::PENDSVCLR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSETR {
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    PENDSVSET_0,
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    PENDSVSET_1,
}
impl crate::ToBits<bool> for PENDSVSETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PENDSVSETR::PENDSVSET_0 => false,
            PENDSVSETR::PENDSVSET_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PENDSVSET_R = crate::FR<bool, PENDSVSETR>;
impl PENDSVSET_R {
    #[doc = "Checks if the value of the field is `PENDSVSET_0`"]
    #[inline(always)]
    pub fn is_pendsvset_0(&self) -> bool {
        *self == PENDSVSETR::PENDSVSET_0
    }
    #[doc = "Checks if the value of the field is `PENDSVSET_1`"]
    #[inline(always)]
    pub fn is_pendsvset_1(&self) -> bool {
        *self == PENDSVSETR::PENDSVSET_1
    }
}
#[doc = "Values that can be written to the field `PENDSVSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVSETW {
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    PENDSVSET_0,
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    PENDSVSET_1,
}
impl PENDSVSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PENDSVSETW::PENDSVSET_0 => false,
            PENDSVSETW::PENDSVSET_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PENDSVSETW<'a> {
    w: &'a mut W,
}
impl<'a> _PENDSVSETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn pendsvset_0(self) -> &'a mut W {
        self.variant(PENDSVSETW::PENDSVSET_0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn pendsvset_1(self) -> &'a mut W {
        self.variant(PENDSVSETW::PENDSVSET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `NMIPENDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSETR {
    #[doc = "write: no effect; read: NMI exception is not pending"]
    NMIPENDSET_0,
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    NMIPENDSET_1,
}
impl crate::ToBits<bool> for NMIPENDSETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NMIPENDSETR::NMIPENDSET_0 => false,
            NMIPENDSETR::NMIPENDSET_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NMIPENDSET_R = crate::FR<bool, NMIPENDSETR>;
impl NMIPENDSET_R {
    #[doc = "Checks if the value of the field is `NMIPENDSET_0`"]
    #[inline(always)]
    pub fn is_nmipendset_0(&self) -> bool {
        *self == NMIPENDSETR::NMIPENDSET_0
    }
    #[doc = "Checks if the value of the field is `NMIPENDSET_1`"]
    #[inline(always)]
    pub fn is_nmipendset_1(&self) -> bool {
        *self == NMIPENDSETR::NMIPENDSET_1
    }
}
#[doc = "Values that can be written to the field `NMIPENDSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIPENDSETW {
    #[doc = "write: no effect; read: NMI exception is not pending"]
    NMIPENDSET_0,
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    NMIPENDSET_1,
}
impl NMIPENDSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NMIPENDSETW::NMIPENDSET_0 => false,
            NMIPENDSETW::NMIPENDSET_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NMIPENDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _NMIPENDSETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIPENDSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn nmipendset_0(self) -> &'a mut W {
        self.variant(NMIPENDSETW::NMIPENDSET_0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn nmipendset_1(self) -> &'a mut W {
        self.variant(NMIPENDSETW::NMIPENDSET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:8 - Active exception number"]
    #[inline(always)]
    pub fn vectactive(&self) -> VECTACTIVE_R {
        VECTACTIVE_R::new((self.bits() & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn rettobase(&self) -> RETTOBASE_R {
        RETTOBASE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VECTPENDING_R {
        VECTPENDING_R::new(((self.bits() >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    pub fn isrpending(&self) -> ISRPENDING_R {
        ISRPENDING_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    pub fn isrpreempt(&self) -> ISRPREEMPT_R {
        ISRPREEMPT_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pendstset(&self) -> PENDSTSET_R {
        PENDSTSET_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PENDSVSET_R {
        PENDSVSET_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NMIPENDSET_R {
        NMIPENDSET_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn pendstclr(&mut self) -> _PENDSTCLRW {
        _PENDSTCLRW { w: self }
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    pub fn pendstset(&mut self) -> _PENDSTSETW {
        _PENDSTSETW { w: self }
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    pub fn pendsvclr(&mut self) -> _PENDSVCLRW {
        _PENDSVCLRW { w: self }
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    pub fn pendsvset(&mut self) -> _PENDSVSETW {
        _PENDSVSETW { w: self }
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn nmipendset(&mut self) -> _NMIPENDSETW {
        _NMIPENDSETW { w: self }
    }
}
