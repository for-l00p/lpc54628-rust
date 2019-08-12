#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFSR {
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
#[doc = "Possible values of the field `IACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOLR {
    #[doc = "no instruction access violation fault"]
    IACCVIOL_0,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    IACCVIOL_1,
}
impl crate::ToBits<bool> for IACCVIOLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IACCVIOLR::IACCVIOL_0 => false,
            IACCVIOLR::IACCVIOL_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IACCVIOL_R = crate::FR<bool, IACCVIOLR>;
impl IACCVIOL_R {
    #[doc = "Checks if the value of the field is `IACCVIOL_0`"]
    #[inline(always)]
    pub fn is_iaccviol_0(&self) -> bool {
        *self == IACCVIOLR::IACCVIOL_0
    }
    #[doc = "Checks if the value of the field is `IACCVIOL_1`"]
    #[inline(always)]
    pub fn is_iaccviol_1(&self) -> bool {
        *self == IACCVIOLR::IACCVIOL_1
    }
}
#[doc = "Values that can be written to the field `IACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOLW {
    #[doc = "no instruction access violation fault"]
    IACCVIOL_0,
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    IACCVIOL_1,
}
impl IACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IACCVIOLW::IACCVIOL_0 => false,
            IACCVIOLW::IACCVIOL_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _IACCVIOLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline(always)]
    pub fn iaccviol_0(self) -> &'a mut W {
        self.variant(IACCVIOLW::IACCVIOL_0)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    #[inline(always)]
    pub fn iaccviol_1(self) -> &'a mut W {
        self.variant(IACCVIOLW::IACCVIOL_1)
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
#[doc = "Possible values of the field `DACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOLR {
    #[doc = "no data access violation fault"]
    DACCVIOL_0,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    DACCVIOL_1,
}
impl crate::ToBits<bool> for DACCVIOLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DACCVIOLR::DACCVIOL_0 => false,
            DACCVIOLR::DACCVIOL_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DACCVIOL_R = crate::FR<bool, DACCVIOLR>;
impl DACCVIOL_R {
    #[doc = "Checks if the value of the field is `DACCVIOL_0`"]
    #[inline(always)]
    pub fn is_daccviol_0(&self) -> bool {
        *self == DACCVIOLR::DACCVIOL_0
    }
    #[doc = "Checks if the value of the field is `DACCVIOL_1`"]
    #[inline(always)]
    pub fn is_daccviol_1(&self) -> bool {
        *self == DACCVIOLR::DACCVIOL_1
    }
}
#[doc = "Values that can be written to the field `DACCVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOLW {
    #[doc = "no data access violation fault"]
    DACCVIOL_0,
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    DACCVIOL_1,
}
impl DACCVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DACCVIOLW::DACCVIOL_0 => false,
            DACCVIOLW::DACCVIOL_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DACCVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCVIOLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACCVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no data access violation fault"]
    #[inline(always)]
    pub fn daccviol_0(self) -> &'a mut W {
        self.variant(DACCVIOLW::DACCVIOL_0)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    #[inline(always)]
    pub fn daccviol_1(self) -> &'a mut W {
        self.variant(DACCVIOLW::DACCVIOL_1)
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
#[doc = "Possible values of the field `MUNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERRR {
    #[doc = "no unstacking fault"]
    MUNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more access violations"]
    MUNSTKERR_1,
}
impl crate::ToBits<bool> for MUNSTKERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MUNSTKERRR::MUNSTKERR_0 => false,
            MUNSTKERRR::MUNSTKERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MUNSTKERR_R = crate::FR<bool, MUNSTKERRR>;
impl MUNSTKERR_R {
    #[doc = "Checks if the value of the field is `MUNSTKERR_0`"]
    #[inline(always)]
    pub fn is_munstkerr_0(&self) -> bool {
        *self == MUNSTKERRR::MUNSTKERR_0
    }
    #[doc = "Checks if the value of the field is `MUNSTKERR_1`"]
    #[inline(always)]
    pub fn is_munstkerr_1(&self) -> bool {
        *self == MUNSTKERRR::MUNSTKERR_1
    }
}
#[doc = "Values that can be written to the field `MUNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERRW {
    #[doc = "no unstacking fault"]
    MUNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more access violations"]
    MUNSTKERR_1,
}
impl MUNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MUNSTKERRW::MUNSTKERR_0 => false,
            MUNSTKERRW::MUNSTKERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MUNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MUNSTKERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn munstkerr_0(self) -> &'a mut W {
        self.variant(MUNSTKERRW::MUNSTKERR_0)
    }
    #[doc = "unstack for an exception return has caused one or more access violations"]
    #[inline(always)]
    pub fn munstkerr_1(self) -> &'a mut W {
        self.variant(MUNSTKERRW::MUNSTKERR_1)
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
#[doc = "Possible values of the field `MSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERRR {
    #[doc = "no stacking fault"]
    MSTKERR_0,
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    MSTKERR_1,
}
impl crate::ToBits<bool> for MSTKERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTKERRR::MSTKERR_0 => false,
            MSTKERRR::MSTKERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTKERR_R = crate::FR<bool, MSTKERRR>;
impl MSTKERR_R {
    #[doc = "Checks if the value of the field is `MSTKERR_0`"]
    #[inline(always)]
    pub fn is_mstkerr_0(&self) -> bool {
        *self == MSTKERRR::MSTKERR_0
    }
    #[doc = "Checks if the value of the field is `MSTKERR_1`"]
    #[inline(always)]
    pub fn is_mstkerr_1(&self) -> bool {
        *self == MSTKERRR::MSTKERR_1
    }
}
#[doc = "Values that can be written to the field `MSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERRW {
    #[doc = "no stacking fault"]
    MSTKERR_0,
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    MSTKERR_1,
}
impl MSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTKERRW::MSTKERR_0 => false,
            MSTKERRW::MSTKERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTKERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn mstkerr_0(self) -> &'a mut W {
        self.variant(MSTKERRW::MSTKERR_0)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    #[inline(always)]
    pub fn mstkerr_1(self) -> &'a mut W {
        self.variant(MSTKERRW::MSTKERR_1)
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
#[doc = "Possible values of the field `MLSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERRR {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_0,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_1,
}
impl crate::ToBits<bool> for MLSPERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MLSPERRR::MLSPERR_0 => false,
            MLSPERRR::MLSPERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MLSPERR_R = crate::FR<bool, MLSPERRR>;
impl MLSPERR_R {
    #[doc = "Checks if the value of the field is `MLSPERR_0`"]
    #[inline(always)]
    pub fn is_mlsperr_0(&self) -> bool {
        *self == MLSPERRR::MLSPERR_0
    }
    #[doc = "Checks if the value of the field is `MLSPERR_1`"]
    #[inline(always)]
    pub fn is_mlsperr_1(&self) -> bool {
        *self == MLSPERRR::MLSPERR_1
    }
}
#[doc = "Values that can be written to the field `MLSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERRW {
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_0,
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_1,
}
impl MLSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MLSPERRW::MLSPERR_0 => false,
            MLSPERRW::MLSPERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MLSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MLSPERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MLSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr_0(self) -> &'a mut W {
        self.variant(MLSPERRW::MLSPERR_0)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr_1(self) -> &'a mut W {
        self.variant(MLSPERRW::MLSPERR_1)
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
#[doc = "Possible values of the field `MMARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALIDR {
    #[doc = "value in MMAR is not a valid fault address"]
    MMARVALID_0,
    #[doc = "MMAR holds a valid fault address"]
    MMARVALID_1,
}
impl crate::ToBits<bool> for MMARVALIDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MMARVALIDR::MMARVALID_0 => false,
            MMARVALIDR::MMARVALID_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MMARVALID_R = crate::FR<bool, MMARVALIDR>;
impl MMARVALID_R {
    #[doc = "Checks if the value of the field is `MMARVALID_0`"]
    #[inline(always)]
    pub fn is_mmarvalid_0(&self) -> bool {
        *self == MMARVALIDR::MMARVALID_0
    }
    #[doc = "Checks if the value of the field is `MMARVALID_1`"]
    #[inline(always)]
    pub fn is_mmarvalid_1(&self) -> bool {
        *self == MMARVALIDR::MMARVALID_1
    }
}
#[doc = "Values that can be written to the field `MMARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALIDW {
    #[doc = "value in MMAR is not a valid fault address"]
    MMARVALID_0,
    #[doc = "MMAR holds a valid fault address"]
    MMARVALID_1,
}
impl MMARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MMARVALIDW::MMARVALID_0 => false,
            MMARVALIDW::MMARVALID_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MMARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _MMARVALIDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline(always)]
    pub fn mmarvalid_0(self) -> &'a mut W {
        self.variant(MMARVALIDW::MMARVALID_0)
    }
    #[doc = "MMAR holds a valid fault address"]
    #[inline(always)]
    pub fn mmarvalid_1(self) -> &'a mut W {
        self.variant(MMARVALIDW::MMARVALID_1)
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
#[doc = "Possible values of the field `IBUSERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERRR {
    #[doc = "no instruction bus error"]
    IBUSERR_0,
    #[doc = "instruction bus error"]
    IBUSERR_1,
}
impl crate::ToBits<bool> for IBUSERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IBUSERRR::IBUSERR_0 => false,
            IBUSERRR::IBUSERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IBUSERR_R = crate::FR<bool, IBUSERRR>;
impl IBUSERR_R {
    #[doc = "Checks if the value of the field is `IBUSERR_0`"]
    #[inline(always)]
    pub fn is_ibuserr_0(&self) -> bool {
        *self == IBUSERRR::IBUSERR_0
    }
    #[doc = "Checks if the value of the field is `IBUSERR_1`"]
    #[inline(always)]
    pub fn is_ibuserr_1(&self) -> bool {
        *self == IBUSERRR::IBUSERR_1
    }
}
#[doc = "Values that can be written to the field `IBUSERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERRW {
    #[doc = "no instruction bus error"]
    IBUSERR_0,
    #[doc = "instruction bus error"]
    IBUSERR_1,
}
impl IBUSERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IBUSERRW::IBUSERR_0 => false,
            IBUSERRW::IBUSERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IBUSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IBUSERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBUSERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no instruction bus error"]
    #[inline(always)]
    pub fn ibuserr_0(self) -> &'a mut W {
        self.variant(IBUSERRW::IBUSERR_0)
    }
    #[doc = "instruction bus error"]
    #[inline(always)]
    pub fn ibuserr_1(self) -> &'a mut W {
        self.variant(IBUSERRW::IBUSERR_1)
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
#[doc = "Possible values of the field `PRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERRR {
    #[doc = "no precise data bus error"]
    PRECISERR_0,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    PRECISERR_1,
}
impl crate::ToBits<bool> for PRECISERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PRECISERRR::PRECISERR_0 => false,
            PRECISERRR::PRECISERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PRECISERR_R = crate::FR<bool, PRECISERRR>;
impl PRECISERR_R {
    #[doc = "Checks if the value of the field is `PRECISERR_0`"]
    #[inline(always)]
    pub fn is_preciserr_0(&self) -> bool {
        *self == PRECISERRR::PRECISERR_0
    }
    #[doc = "Checks if the value of the field is `PRECISERR_1`"]
    #[inline(always)]
    pub fn is_preciserr_1(&self) -> bool {
        *self == PRECISERRR::PRECISERR_1
    }
}
#[doc = "Values that can be written to the field `PRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERRW {
    #[doc = "no precise data bus error"]
    PRECISERR_0,
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    PRECISERR_1,
}
impl PRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PRECISERRW::PRECISERR_0 => false,
            PRECISERRW::PRECISERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRECISERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no precise data bus error"]
    #[inline(always)]
    pub fn preciserr_0(self) -> &'a mut W {
        self.variant(PRECISERRW::PRECISERR_0)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    #[inline(always)]
    pub fn preciserr_1(self) -> &'a mut W {
        self.variant(PRECISERRW::PRECISERR_1)
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
#[doc = "Possible values of the field `IMPRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERRR {
    #[doc = "no imprecise data bus error"]
    IMPRECISERR_0,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    IMPRECISERR_1,
}
impl crate::ToBits<bool> for IMPRECISERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            IMPRECISERRR::IMPRECISERR_0 => false,
            IMPRECISERRR::IMPRECISERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMPRECISERR_R = crate::FR<bool, IMPRECISERRR>;
impl IMPRECISERR_R {
    #[doc = "Checks if the value of the field is `IMPRECISERR_0`"]
    #[inline(always)]
    pub fn is_impreciserr_0(&self) -> bool {
        *self == IMPRECISERRR::IMPRECISERR_0
    }
    #[doc = "Checks if the value of the field is `IMPRECISERR_1`"]
    #[inline(always)]
    pub fn is_impreciserr_1(&self) -> bool {
        *self == IMPRECISERRR::IMPRECISERR_1
    }
}
#[doc = "Values that can be written to the field `IMPRECISERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERRW {
    #[doc = "no imprecise data bus error"]
    IMPRECISERR_0,
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    IMPRECISERR_1,
}
impl IMPRECISERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            IMPRECISERRW::IMPRECISERR_0 => false,
            IMPRECISERRW::IMPRECISERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IMPRECISERRW<'a> {
    w: &'a mut W,
}
impl<'a> _IMPRECISERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMPRECISERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr_0(self) -> &'a mut W {
        self.variant(IMPRECISERRW::IMPRECISERR_0)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    #[inline(always)]
    pub fn impreciserr_1(self) -> &'a mut W {
        self.variant(IMPRECISERRW::IMPRECISERR_1)
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
#[doc = "Possible values of the field `UNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERRR {
    #[doc = "no unstacking fault"]
    UNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    UNSTKERR_1,
}
impl crate::ToBits<bool> for UNSTKERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNSTKERRR::UNSTKERR_0 => false,
            UNSTKERRR::UNSTKERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNSTKERR_R = crate::FR<bool, UNSTKERRR>;
impl UNSTKERR_R {
    #[doc = "Checks if the value of the field is `UNSTKERR_0`"]
    #[inline(always)]
    pub fn is_unstkerr_0(&self) -> bool {
        *self == UNSTKERRR::UNSTKERR_0
    }
    #[doc = "Checks if the value of the field is `UNSTKERR_1`"]
    #[inline(always)]
    pub fn is_unstkerr_1(&self) -> bool {
        *self == UNSTKERRR::UNSTKERR_1
    }
}
#[doc = "Values that can be written to the field `UNSTKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERRW {
    #[doc = "no unstacking fault"]
    UNSTKERR_0,
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    UNSTKERR_1,
}
impl UNSTKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNSTKERRW::UNSTKERR_0 => false,
            UNSTKERRW::UNSTKERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNSTKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNSTKERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNSTKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn unstkerr_0(self) -> &'a mut W {
        self.variant(UNSTKERRW::UNSTKERR_0)
    }
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    #[inline(always)]
    pub fn unstkerr_1(self) -> &'a mut W {
        self.variant(UNSTKERRW::UNSTKERR_1)
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
#[doc = "Possible values of the field `STKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERRR {
    #[doc = "no stacking fault"]
    STKERR_0,
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    STKERR_1,
}
impl crate::ToBits<bool> for STKERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STKERRR::STKERR_0 => false,
            STKERRR::STKERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type STKERR_R = crate::FR<bool, STKERRR>;
impl STKERR_R {
    #[doc = "Checks if the value of the field is `STKERR_0`"]
    #[inline(always)]
    pub fn is_stkerr_0(&self) -> bool {
        *self == STKERRR::STKERR_0
    }
    #[doc = "Checks if the value of the field is `STKERR_1`"]
    #[inline(always)]
    pub fn is_stkerr_1(&self) -> bool {
        *self == STKERRR::STKERR_1
    }
}
#[doc = "Values that can be written to the field `STKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERRW {
    #[doc = "no stacking fault"]
    STKERR_0,
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    STKERR_1,
}
impl STKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            STKERRW::STKERR_0 => false,
            STKERRW::STKERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _STKERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn stkerr_0(self) -> &'a mut W {
        self.variant(STKERRW::STKERR_0)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    #[inline(always)]
    pub fn stkerr_1(self) -> &'a mut W {
        self.variant(STKERRW::STKERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `LSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERRR {
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    LSPERR_0,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    LSPERR_1,
}
impl crate::ToBits<bool> for LSPERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LSPERRR::LSPERR_0 => false,
            LSPERRR::LSPERR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LSPERR_R = crate::FR<bool, LSPERRR>;
impl LSPERR_R {
    #[doc = "Checks if the value of the field is `LSPERR_0`"]
    #[inline(always)]
    pub fn is_lsperr_0(&self) -> bool {
        *self == LSPERRR::LSPERR_0
    }
    #[doc = "Checks if the value of the field is `LSPERR_1`"]
    #[inline(always)]
    pub fn is_lsperr_1(&self) -> bool {
        *self == LSPERRR::LSPERR_1
    }
}
#[doc = "Values that can be written to the field `LSPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERRW {
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    LSPERR_0,
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    LSPERR_1,
}
impl LSPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPERRW::LSPERR_0 => false,
            LSPERRW::LSPERR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LSPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr_0(self) -> &'a mut W {
        self.variant(LSPERRW::LSPERR_0)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr_1(self) -> &'a mut W {
        self.variant(LSPERRW::LSPERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `BFARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALIDR {
    #[doc = "value in BFAR is not a valid fault address"]
    BFARVALID_0,
    #[doc = "BFAR holds a valid fault address"]
    BFARVALID_1,
}
impl crate::ToBits<bool> for BFARVALIDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BFARVALIDR::BFARVALID_0 => false,
            BFARVALIDR::BFARVALID_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BFARVALID_R = crate::FR<bool, BFARVALIDR>;
impl BFARVALID_R {
    #[doc = "Checks if the value of the field is `BFARVALID_0`"]
    #[inline(always)]
    pub fn is_bfarvalid_0(&self) -> bool {
        *self == BFARVALIDR::BFARVALID_0
    }
    #[doc = "Checks if the value of the field is `BFARVALID_1`"]
    #[inline(always)]
    pub fn is_bfarvalid_1(&self) -> bool {
        *self == BFARVALIDR::BFARVALID_1
    }
}
#[doc = "Values that can be written to the field `BFARVALID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALIDW {
    #[doc = "value in BFAR is not a valid fault address"]
    BFARVALID_0,
    #[doc = "BFAR holds a valid fault address"]
    BFARVALID_1,
}
impl BFARVALIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BFARVALIDW::BFARVALID_0 => false,
            BFARVALIDW::BFARVALID_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BFARVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _BFARVALIDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFARVALIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline(always)]
    pub fn bfarvalid_0(self) -> &'a mut W {
        self.variant(BFARVALIDW::BFARVALID_0)
    }
    #[doc = "BFAR holds a valid fault address"]
    #[inline(always)]
    pub fn bfarvalid_1(self) -> &'a mut W {
        self.variant(BFARVALIDW::BFARVALID_1)
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
#[doc = "Possible values of the field `UNDEFINSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTRR {
    #[doc = "no undefined instruction UsageFault"]
    UNDEFINSTR_0,
    #[doc = "the processor has attempted to execute an undefined instruction"]
    UNDEFINSTR_1,
}
impl crate::ToBits<bool> for UNDEFINSTRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNDEFINSTRR::UNDEFINSTR_0 => false,
            UNDEFINSTRR::UNDEFINSTR_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNDEFINSTR_R = crate::FR<bool, UNDEFINSTRR>;
impl UNDEFINSTR_R {
    #[doc = "Checks if the value of the field is `UNDEFINSTR_0`"]
    #[inline(always)]
    pub fn is_undefinstr_0(&self) -> bool {
        *self == UNDEFINSTRR::UNDEFINSTR_0
    }
    #[doc = "Checks if the value of the field is `UNDEFINSTR_1`"]
    #[inline(always)]
    pub fn is_undefinstr_1(&self) -> bool {
        *self == UNDEFINSTRR::UNDEFINSTR_1
    }
}
#[doc = "Values that can be written to the field `UNDEFINSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTRW {
    #[doc = "no undefined instruction UsageFault"]
    UNDEFINSTR_0,
    #[doc = "the processor has attempted to execute an undefined instruction"]
    UNDEFINSTR_1,
}
impl UNDEFINSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNDEFINSTRW::UNDEFINSTR_0 => false,
            UNDEFINSTRW::UNDEFINSTR_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNDEFINSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDEFINSTRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDEFINSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr_0(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::UNDEFINSTR_0)
    }
    #[doc = "the processor has attempted to execute an undefined instruction"]
    #[inline(always)]
    pub fn undefinstr_1(self) -> &'a mut W {
        self.variant(UNDEFINSTRW::UNDEFINSTR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `INVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATER {
    #[doc = "no invalid state UsageFault"]
    INVSTATE_0,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    INVSTATE_1,
}
impl crate::ToBits<bool> for INVSTATER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INVSTATER::INVSTATE_0 => false,
            INVSTATER::INVSTATE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INVSTATE_R = crate::FR<bool, INVSTATER>;
impl INVSTATE_R {
    #[doc = "Checks if the value of the field is `INVSTATE_0`"]
    #[inline(always)]
    pub fn is_invstate_0(&self) -> bool {
        *self == INVSTATER::INVSTATE_0
    }
    #[doc = "Checks if the value of the field is `INVSTATE_1`"]
    #[inline(always)]
    pub fn is_invstate_1(&self) -> bool {
        *self == INVSTATER::INVSTATE_1
    }
}
#[doc = "Values that can be written to the field `INVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATEW {
    #[doc = "no invalid state UsageFault"]
    INVSTATE_0,
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    INVSTATE_1,
}
impl INVSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INVSTATEW::INVSTATE_0 => false,
            INVSTATEW::INVSTATE_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INVSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _INVSTATEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVSTATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate_0(self) -> &'a mut W {
        self.variant(INVSTATEW::INVSTATE_0)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    #[inline(always)]
    pub fn invstate_1(self) -> &'a mut W {
        self.variant(INVSTATEW::INVSTATE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `INVPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPCR {
    #[doc = "no invalid PC load UsageFault"]
    INVPC_0,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    INVPC_1,
}
impl crate::ToBits<bool> for INVPCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INVPCR::INVPC_0 => false,
            INVPCR::INVPC_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INVPC_R = crate::FR<bool, INVPCR>;
impl INVPC_R {
    #[doc = "Checks if the value of the field is `INVPC_0`"]
    #[inline(always)]
    pub fn is_invpc_0(&self) -> bool {
        *self == INVPCR::INVPC_0
    }
    #[doc = "Checks if the value of the field is `INVPC_1`"]
    #[inline(always)]
    pub fn is_invpc_1(&self) -> bool {
        *self == INVPCR::INVPC_1
    }
}
#[doc = "Values that can be written to the field `INVPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPCW {
    #[doc = "no invalid PC load UsageFault"]
    INVPC_0,
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    INVPC_1,
}
impl INVPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INVPCW::INVPC_0 => false,
            INVPCW::INVPC_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INVPCW<'a> {
    w: &'a mut W,
}
impl<'a> _INVPCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc_0(self) -> &'a mut W {
        self.variant(INVPCW::INVPC_0)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    #[inline(always)]
    pub fn invpc_1(self) -> &'a mut W {
        self.variant(INVPCW::INVPC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `NOCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCPR {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    NOCP_0,
    #[doc = "the processor has attempted to access a coprocessor"]
    NOCP_1,
}
impl crate::ToBits<bool> for NOCPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NOCPR::NOCP_0 => false,
            NOCPR::NOCP_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NOCP_R = crate::FR<bool, NOCPR>;
impl NOCP_R {
    #[doc = "Checks if the value of the field is `NOCP_0`"]
    #[inline(always)]
    pub fn is_nocp_0(&self) -> bool {
        *self == NOCPR::NOCP_0
    }
    #[doc = "Checks if the value of the field is `NOCP_1`"]
    #[inline(always)]
    pub fn is_nocp_1(&self) -> bool {
        *self == NOCPR::NOCP_1
    }
}
#[doc = "Values that can be written to the field `NOCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCPW {
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    NOCP_0,
    #[doc = "the processor has attempted to access a coprocessor"]
    NOCP_1,
}
impl NOCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NOCPW::NOCP_0 => false,
            NOCPW::NOCP_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NOCPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline(always)]
    pub fn nocp_0(self) -> &'a mut W {
        self.variant(NOCPW::NOCP_0)
    }
    #[doc = "the processor has attempted to access a coprocessor"]
    #[inline(always)]
    pub fn nocp_1(self) -> &'a mut W {
        self.variant(NOCPW::NOCP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `UNALIGNED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNEDR {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    UNALIGNED_0,
    #[doc = "the processor has made an unaligned memory access"]
    UNALIGNED_1,
}
impl crate::ToBits<bool> for UNALIGNEDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNALIGNEDR::UNALIGNED_0 => false,
            UNALIGNEDR::UNALIGNED_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNALIGNED_R = crate::FR<bool, UNALIGNEDR>;
impl UNALIGNED_R {
    #[doc = "Checks if the value of the field is `UNALIGNED_0`"]
    #[inline(always)]
    pub fn is_unaligned_0(&self) -> bool {
        *self == UNALIGNEDR::UNALIGNED_0
    }
    #[doc = "Checks if the value of the field is `UNALIGNED_1`"]
    #[inline(always)]
    pub fn is_unaligned_1(&self) -> bool {
        *self == UNALIGNEDR::UNALIGNED_1
    }
}
#[doc = "Values that can be written to the field `UNALIGNED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNEDW {
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    UNALIGNED_0,
    #[doc = "the processor has made an unaligned memory access"]
    UNALIGNED_1,
}
impl UNALIGNEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGNEDW::UNALIGNED_0 => false,
            UNALIGNEDW::UNALIGNED_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNALIGNEDW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGNEDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGNEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline(always)]
    pub fn unaligned_0(self) -> &'a mut W {
        self.variant(UNALIGNEDW::UNALIGNED_0)
    }
    #[doc = "the processor has made an unaligned memory access"]
    #[inline(always)]
    pub fn unaligned_1(self) -> &'a mut W {
        self.variant(UNALIGNEDW::UNALIGNED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `DIVBYZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZEROR {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    DIVBYZERO_0,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    DIVBYZERO_1,
}
impl crate::ToBits<bool> for DIVBYZEROR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIVBYZEROR::DIVBYZERO_0 => false,
            DIVBYZEROR::DIVBYZERO_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIVBYZERO_R = crate::FR<bool, DIVBYZEROR>;
impl DIVBYZERO_R {
    #[doc = "Checks if the value of the field is `DIVBYZERO_0`"]
    #[inline(always)]
    pub fn is_divbyzero_0(&self) -> bool {
        *self == DIVBYZEROR::DIVBYZERO_0
    }
    #[doc = "Checks if the value of the field is `DIVBYZERO_1`"]
    #[inline(always)]
    pub fn is_divbyzero_1(&self) -> bool {
        *self == DIVBYZEROR::DIVBYZERO_1
    }
}
#[doc = "Values that can be written to the field `DIVBYZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZEROW {
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    DIVBYZERO_0,
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    DIVBYZERO_1,
}
impl DIVBYZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIVBYZEROW::DIVBYZERO_0 => false,
            DIVBYZEROW::DIVBYZERO_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIVBYZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBYZEROW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVBYZEROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline(always)]
    pub fn divbyzero_0(self) -> &'a mut W {
        self.variant(DIVBYZEROW::DIVBYZERO_0)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn divbyzero_1(self) -> &'a mut W {
        self.variant(DIVBYZEROW::DIVBYZERO_1)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MLSPERR_R {
        MLSPERR_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn iaccviol(&mut self) -> _IACCVIOLW {
        _IACCVIOLW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn daccviol(&mut self) -> _DACCVIOLW {
        _DACCVIOLW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn munstkerr(&mut self) -> _MUNSTKERRW {
        _MUNSTKERRW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn mstkerr(&mut self) -> _MSTKERRW {
        _MSTKERRW { w: self }
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn mlsperr(&mut self) -> _MLSPERRW {
        _MLSPERRW { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn mmarvalid(&mut self) -> _MMARVALIDW {
        _MMARVALIDW { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn ibuserr(&mut self) -> _IBUSERRW {
        _IBUSERRW { w: self }
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    pub fn preciserr(&mut self) -> _PRECISERRW {
        _PRECISERRW { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn impreciserr(&mut self) -> _IMPRECISERRW {
        _IMPRECISERRW { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn unstkerr(&mut self) -> _UNSTKERRW {
        _UNSTKERRW { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn stkerr(&mut self) -> _STKERRW {
        _STKERRW { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> _LSPERRW {
        _LSPERRW { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn bfarvalid(&mut self) -> _BFARVALIDW {
        _BFARVALIDW { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn undefinstr(&mut self) -> _UNDEFINSTRW {
        _UNDEFINSTRW { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn invstate(&mut self) -> _INVSTATEW {
        _INVSTATEW { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn invpc(&mut self) -> _INVPCW {
        _INVPCW { w: self }
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    pub fn nocp(&mut self) -> _NOCPW {
        _NOCPW { w: self }
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn unaligned(&mut self) -> _UNALIGNEDW {
        _UNALIGNEDW { w: self }
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    pub fn divbyzero(&mut self) -> _DIVBYZEROW {
        _DIVBYZEROW { w: self }
    }
}
