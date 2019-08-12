#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FPCCR {
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
        0xc000_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `LSPACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPACTR {
    #[doc = "Lazy state preservation is not active."]
    LSPACT_0,
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    LSPACT_1,
}
impl crate::ToBits<bool> for LSPACTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LSPACTR::LSPACT_0 => false,
            LSPACTR::LSPACT_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LSPACT_R = crate::FR<bool, LSPACTR>;
impl LSPACT_R {
    #[doc = "Checks if the value of the field is `LSPACT_0`"]
    #[inline(always)]
    pub fn is_lspact_0(&self) -> bool {
        *self == LSPACTR::LSPACT_0
    }
    #[doc = "Checks if the value of the field is `LSPACT_1`"]
    #[inline(always)]
    pub fn is_lspact_1(&self) -> bool {
        *self == LSPACTR::LSPACT_1
    }
}
#[doc = "Values that can be written to the field `LSPACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPACTW {
    #[doc = "Lazy state preservation is not active."]
    LSPACT_0,
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    LSPACT_1,
}
impl LSPACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPACTW::LSPACT_0 => false,
            LSPACTW::LSPACT_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LSPACTW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPACTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPACTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lazy state preservation is not active."]
    #[inline(always)]
    pub fn lspact_0(self) -> &'a mut W {
        self.variant(LSPACTW::LSPACT_0)
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn lspact_1(self) -> &'a mut W {
        self.variant(LSPACTW::LSPACT_1)
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
#[doc = "Possible values of the field `USER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERR {
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    USER_0,
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    USER_1,
}
impl crate::ToBits<bool> for USERR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USERR::USER_0 => false,
            USERR::USER_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USER_R = crate::FR<bool, USERR>;
impl USER_R {
    #[doc = "Checks if the value of the field is `USER_0`"]
    #[inline(always)]
    pub fn is_user_0(&self) -> bool {
        *self == USERR::USER_0
    }
    #[doc = "Checks if the value of the field is `USER_1`"]
    #[inline(always)]
    pub fn is_user_1(&self) -> bool {
        *self == USERR::USER_1
    }
}
#[doc = "Values that can be written to the field `USER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERW {
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    USER_0,
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    USER_1,
}
impl USERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USERW::USER_0 => false,
            USERW::USER_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USERW<'a> {
    w: &'a mut W,
}
impl<'a> _USERW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user_0(self) -> &'a mut W {
        self.variant(USERW::USER_0)
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user_1(self) -> &'a mut W {
        self.variant(USERW::USER_1)
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
#[doc = "Possible values of the field `THREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREADR {
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    THREAD_0,
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    THREAD_1,
}
impl crate::ToBits<bool> for THREADR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            THREADR::THREAD_0 => false,
            THREADR::THREAD_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type THREAD_R = crate::FR<bool, THREADR>;
impl THREAD_R {
    #[doc = "Checks if the value of the field is `THREAD_0`"]
    #[inline(always)]
    pub fn is_thread_0(&self) -> bool {
        *self == THREADR::THREAD_0
    }
    #[doc = "Checks if the value of the field is `THREAD_1`"]
    #[inline(always)]
    pub fn is_thread_1(&self) -> bool {
        *self == THREADR::THREAD_1
    }
}
#[doc = "Values that can be written to the field `THREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREADW {
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    THREAD_0,
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    THREAD_1,
}
impl THREADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            THREADW::THREAD_0 => false,
            THREADW::THREAD_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _THREADW<'a> {
    w: &'a mut W,
}
impl<'a> _THREADW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread_0(self) -> &'a mut W {
        self.variant(THREADW::THREAD_0)
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread_1(self) -> &'a mut W {
        self.variant(THREADW::THREAD_1)
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
#[doc = "Possible values of the field `HFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFRDYR {
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    HFRDY_0,
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    HFRDY_1,
}
impl crate::ToBits<bool> for HFRDYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HFRDYR::HFRDY_0 => false,
            HFRDYR::HFRDY_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HFRDY_R = crate::FR<bool, HFRDYR>;
impl HFRDY_R {
    #[doc = "Checks if the value of the field is `HFRDY_0`"]
    #[inline(always)]
    pub fn is_hfrdy_0(&self) -> bool {
        *self == HFRDYR::HFRDY_0
    }
    #[doc = "Checks if the value of the field is `HFRDY_1`"]
    #[inline(always)]
    pub fn is_hfrdy_1(&self) -> bool {
        *self == HFRDYR::HFRDY_1
    }
}
#[doc = "Values that can be written to the field `HFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFRDYW {
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    HFRDY_0,
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    HFRDY_1,
}
impl HFRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HFRDYW::HFRDY_0 => false,
            HFRDYW::HFRDY_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HFRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _HFRDYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy_0(self) -> &'a mut W {
        self.variant(HFRDYW::HFRDY_0)
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy_1(self) -> &'a mut W {
        self.variant(HFRDYW::HFRDY_1)
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
#[doc = "Possible values of the field `MMRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRDYR {
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    MMRDY_0,
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    MMRDY_1,
}
impl crate::ToBits<bool> for MMRDYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MMRDYR::MMRDY_0 => false,
            MMRDYR::MMRDY_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MMRDY_R = crate::FR<bool, MMRDYR>;
impl MMRDY_R {
    #[doc = "Checks if the value of the field is `MMRDY_0`"]
    #[inline(always)]
    pub fn is_mmrdy_0(&self) -> bool {
        *self == MMRDYR::MMRDY_0
    }
    #[doc = "Checks if the value of the field is `MMRDY_1`"]
    #[inline(always)]
    pub fn is_mmrdy_1(&self) -> bool {
        *self == MMRDYR::MMRDY_1
    }
}
#[doc = "Values that can be written to the field `MMRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRDYW {
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    MMRDY_0,
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    MMRDY_1,
}
impl MMRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MMRDYW::MMRDY_0 => false,
            MMRDYW::MMRDY_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MMRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MMRDYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy_0(self) -> &'a mut W {
        self.variant(MMRDYW::MMRDY_0)
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy_1(self) -> &'a mut W {
        self.variant(MMRDYW::MMRDY_1)
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
#[doc = "Possible values of the field `BFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFRDYR {
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    BFRDY_0,
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    BFRDY_1,
}
impl crate::ToBits<bool> for BFRDYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BFRDYR::BFRDY_0 => false,
            BFRDYR::BFRDY_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BFRDY_R = crate::FR<bool, BFRDYR>;
impl BFRDY_R {
    #[doc = "Checks if the value of the field is `BFRDY_0`"]
    #[inline(always)]
    pub fn is_bfrdy_0(&self) -> bool {
        *self == BFRDYR::BFRDY_0
    }
    #[doc = "Checks if the value of the field is `BFRDY_1`"]
    #[inline(always)]
    pub fn is_bfrdy_1(&self) -> bool {
        *self == BFRDYR::BFRDY_1
    }
}
#[doc = "Values that can be written to the field `BFRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFRDYW {
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    BFRDY_0,
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    BFRDY_1,
}
impl BFRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BFRDYW::BFRDY_0 => false,
            BFRDYW::BFRDY_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BFRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BFRDYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy_0(self) -> &'a mut W {
        self.variant(BFRDYW::BFRDY_0)
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy_1(self) -> &'a mut W {
        self.variant(BFRDYW::BFRDY_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYR {
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    MONRDY_0,
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    MONRDY_1,
}
impl crate::ToBits<bool> for MONRDYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONRDYR::MONRDY_0 => false,
            MONRDYR::MONRDY_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONRDY_R = crate::FR<bool, MONRDYR>;
impl MONRDY_R {
    #[doc = "Checks if the value of the field is `MONRDY_0`"]
    #[inline(always)]
    pub fn is_monrdy_0(&self) -> bool {
        *self == MONRDYR::MONRDY_0
    }
    #[doc = "Checks if the value of the field is `MONRDY_1`"]
    #[inline(always)]
    pub fn is_monrdy_1(&self) -> bool {
        *self == MONRDYR::MONRDY_1
    }
}
#[doc = "Values that can be written to the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYW {
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    MONRDY_0,
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    MONRDY_1,
}
impl MONRDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONRDYW::MONRDY_0 => false,
            MONRDYW::MONRDY_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONRDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy_0(self) -> &'a mut W {
        self.variant(MONRDYW::MONRDY_0)
    }
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy_1(self) -> &'a mut W {
        self.variant(MONRDYW::MONRDY_1)
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
#[doc = "Possible values of the field `LSPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPENR {
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    LSPEN_0,
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    LSPEN_1,
}
impl crate::ToBits<bool> for LSPENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LSPENR::LSPEN_0 => false,
            LSPENR::LSPEN_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LSPEN_R = crate::FR<bool, LSPENR>;
impl LSPEN_R {
    #[doc = "Checks if the value of the field is `LSPEN_0`"]
    #[inline(always)]
    pub fn is_lspen_0(&self) -> bool {
        *self == LSPENR::LSPEN_0
    }
    #[doc = "Checks if the value of the field is `LSPEN_1`"]
    #[inline(always)]
    pub fn is_lspen_1(&self) -> bool {
        *self == LSPENR::LSPEN_1
    }
}
#[doc = "Values that can be written to the field `LSPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPENW {
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    LSPEN_0,
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    LSPEN_1,
}
impl LSPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LSPENW::LSPEN_0 => false,
            LSPENW::LSPEN_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LSPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LSPENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen_0(self) -> &'a mut W {
        self.variant(LSPENW::LSPEN_0)
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen_1(self) -> &'a mut W {
        self.variant(LSPENW::LSPEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Possible values of the field `ASPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPENR {
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    ASPEN_0,
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    ASPEN_1,
}
impl crate::ToBits<bool> for ASPENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ASPENR::ASPEN_0 => false,
            ASPENR::ASPEN_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ASPEN_R = crate::FR<bool, ASPENR>;
impl ASPEN_R {
    #[doc = "Checks if the value of the field is `ASPEN_0`"]
    #[inline(always)]
    pub fn is_aspen_0(&self) -> bool {
        *self == ASPENR::ASPEN_0
    }
    #[doc = "Checks if the value of the field is `ASPEN_1`"]
    #[inline(always)]
    pub fn is_aspen_1(&self) -> bool {
        *self == ASPENR::ASPEN_1
    }
}
#[doc = "Values that can be written to the field `ASPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPENW {
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    ASPEN_0,
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    ASPEN_1,
}
impl ASPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ASPENW::ASPEN_0 => false,
            ASPENW::ASPEN_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ASPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn aspen_0(self) -> &'a mut W {
        self.variant(ASPENW::ASPEN_0)
    }
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn aspen_1(self) -> &'a mut W {
        self.variant(ASPENW::ASPEN_1)
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
    #[doc = "Bit 0 - Lazy state preservation."]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Privilege level when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Permission to set the MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit."]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Lazy state preservation."]
    #[inline(always)]
    pub fn lspact(&mut self) -> _LSPACTW {
        _LSPACTW { w: self }
    }
    #[doc = "Bit 1 - Privilege level when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&mut self) -> _USERW {
        _USERW { w: self }
    }
    #[doc = "Bit 3 - Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&mut self) -> _THREADW {
        _THREADW { w: self }
    }
    #[doc = "Bit 4 - Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&mut self) -> _HFRDYW {
        _HFRDYW { w: self }
    }
    #[doc = "Bit 5 - Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&mut self) -> _MMRDYW {
        _MMRDYW { w: self }
    }
    #[doc = "Bit 6 - Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&mut self) -> _BFRDYW {
        _BFRDYW { w: self }
    }
    #[doc = "Bit 8 - Permission to set the MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&mut self) -> _MONRDYW {
        _MONRDYW { w: self }
    }
    #[doc = "Bit 30 - Lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&mut self) -> _LSPENW {
        _LSPENW { w: self }
    }
    #[doc = "Bit 31 - Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit."]
    #[inline(always)]
    pub fn aspen(&mut self) -> _ASPENW {
        _ASPENW { w: self }
    }
}
