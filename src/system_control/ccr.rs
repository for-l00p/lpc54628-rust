#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `NONBASETHRDENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENAR {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    NONBASETHRDENA_0,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    NONBASETHRDENA_1,
}
impl crate::ToBits<bool> for NONBASETHRDENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NONBASETHRDENAR::NONBASETHRDENA_0 => false,
            NONBASETHRDENAR::NONBASETHRDENA_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NONBASETHRDENA_R = crate::FR<bool, NONBASETHRDENAR>;
impl NONBASETHRDENA_R {
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_0`"]
    #[inline(always)]
    pub fn is_nonbasethrdena_0(&self) -> bool {
        *self == NONBASETHRDENAR::NONBASETHRDENA_0
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_1`"]
    #[inline(always)]
    pub fn is_nonbasethrdena_1(&self) -> bool {
        *self == NONBASETHRDENAR::NONBASETHRDENA_1
    }
}
#[doc = "Values that can be written to the field `NONBASETHRDENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENAW {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    NONBASETHRDENA_0,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    NONBASETHRDENA_1,
}
impl NONBASETHRDENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NONBASETHRDENAW::NONBASETHRDENA_0 => false,
            NONBASETHRDENAW::NONBASETHRDENA_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NONBASETHRDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _NONBASETHRDENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NONBASETHRDENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline(always)]
    pub fn nonbasethrdena_0(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::NONBASETHRDENA_0)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline(always)]
    pub fn nonbasethrdena_1(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::NONBASETHRDENA_1)
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
#[doc = "Possible values of the field `USERSETMPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPENDR {
    #[doc = "disable"]
    USERSETMPEND_0,
    #[doc = "enable"]
    USERSETMPEND_1,
}
impl crate::ToBits<bool> for USERSETMPENDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USERSETMPENDR::USERSETMPEND_0 => false,
            USERSETMPENDR::USERSETMPEND_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USERSETMPEND_R = crate::FR<bool, USERSETMPENDR>;
impl USERSETMPEND_R {
    #[doc = "Checks if the value of the field is `USERSETMPEND_0`"]
    #[inline(always)]
    pub fn is_usersetmpend_0(&self) -> bool {
        *self == USERSETMPENDR::USERSETMPEND_0
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_1`"]
    #[inline(always)]
    pub fn is_usersetmpend_1(&self) -> bool {
        *self == USERSETMPENDR::USERSETMPEND_1
    }
}
#[doc = "Values that can be written to the field `USERSETMPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPENDW {
    #[doc = "disable"]
    USERSETMPEND_0,
    #[doc = "enable"]
    USERSETMPEND_1,
}
impl USERSETMPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USERSETMPENDW::USERSETMPEND_0 => false,
            USERSETMPENDW::USERSETMPEND_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USERSETMPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USERSETMPENDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USERSETMPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn usersetmpend_0(self) -> &'a mut W {
        self.variant(USERSETMPENDW::USERSETMPEND_0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn usersetmpend_1(self) -> &'a mut W {
        self.variant(USERSETMPENDW::USERSETMPEND_1)
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
#[doc = "Possible values of the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPR {
    #[doc = "do not trap unaligned halfword and word accesses"]
    UNALIGN_TRP_0,
    #[doc = "trap unaligned halfword and word accesses"]
    UNALIGN_TRP_1,
}
impl crate::ToBits<bool> for UNALIGN_TRPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRPR::UNALIGN_TRP_0 => false,
            UNALIGN_TRPR::UNALIGN_TRP_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNALIGN_TRP_R = crate::FR<bool, UNALIGN_TRPR>;
impl UNALIGN_TRP_R {
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_0`"]
    #[inline(always)]
    pub fn is_unalign_trp_0(&self) -> bool {
        *self == UNALIGN_TRPR::UNALIGN_TRP_0
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_1`"]
    #[inline(always)]
    pub fn is_unalign_trp_1(&self) -> bool {
        *self == UNALIGN_TRPR::UNALIGN_TRP_1
    }
}
#[doc = "Values that can be written to the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPW {
    #[doc = "do not trap unaligned halfword and word accesses"]
    UNALIGN_TRP_0,
    #[doc = "trap unaligned halfword and word accesses"]
    UNALIGN_TRP_1,
}
impl UNALIGN_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRPW::UNALIGN_TRP_0 => false,
            UNALIGN_TRPW::UNALIGN_TRP_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNALIGN_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGN_TRPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGN_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn unalign_trp_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::UNALIGN_TRP_0)
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn unalign_trp_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::UNALIGN_TRP_1)
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
#[doc = "Possible values of the field `DIV_0_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRPR {
    #[doc = "do not trap divide by 0"]
    DIV_0_TRP_0,
    #[doc = "trap divide by 0"]
    DIV_0_TRP_1,
}
impl crate::ToBits<bool> for DIV_0_TRPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIV_0_TRPR::DIV_0_TRP_0 => false,
            DIV_0_TRPR::DIV_0_TRP_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIV_0_TRP_R = crate::FR<bool, DIV_0_TRPR>;
impl DIV_0_TRP_R {
    #[doc = "Checks if the value of the field is `DIV_0_TRP_0`"]
    #[inline(always)]
    pub fn is_div_0_trp_0(&self) -> bool {
        *self == DIV_0_TRPR::DIV_0_TRP_0
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_1`"]
    #[inline(always)]
    pub fn is_div_0_trp_1(&self) -> bool {
        *self == DIV_0_TRPR::DIV_0_TRP_1
    }
}
#[doc = "Values that can be written to the field `DIV_0_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRPW {
    #[doc = "do not trap divide by 0"]
    DIV_0_TRP_0,
    #[doc = "trap divide by 0"]
    DIV_0_TRP_1,
}
impl DIV_0_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV_0_TRPW::DIV_0_TRP_0 => false,
            DIV_0_TRPW::DIV_0_TRP_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIV_0_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_0_TRPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_0_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn div_0_trp_0(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::DIV_0_TRP_0)
    }
    #[doc = "trap divide by 0"]
    #[inline(always)]
    pub fn div_0_trp_1(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::DIV_0_TRP_1)
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
#[doc = "Possible values of the field `BFHFNMIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGNR {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    BFHFNMIGN_0,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    BFHFNMIGN_1,
}
impl crate::ToBits<bool> for BFHFNMIGNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BFHFNMIGNR::BFHFNMIGN_0 => false,
            BFHFNMIGNR::BFHFNMIGN_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BFHFNMIGN_R = crate::FR<bool, BFHFNMIGNR>;
impl BFHFNMIGN_R {
    #[doc = "Checks if the value of the field is `BFHFNMIGN_0`"]
    #[inline(always)]
    pub fn is_bfhfnmign_0(&self) -> bool {
        *self == BFHFNMIGNR::BFHFNMIGN_0
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_1`"]
    #[inline(always)]
    pub fn is_bfhfnmign_1(&self) -> bool {
        *self == BFHFNMIGNR::BFHFNMIGN_1
    }
}
#[doc = "Values that can be written to the field `BFHFNMIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGNW {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    BFHFNMIGN_0,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    BFHFNMIGN_1,
}
impl BFHFNMIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BFHFNMIGNW::BFHFNMIGN_0 => false,
            BFHFNMIGNW::BFHFNMIGN_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BFHFNMIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _BFHFNMIGNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFHFNMIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn bfhfnmign_0(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::BFHFNMIGN_0)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline(always)]
    pub fn bfhfnmign_1(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::BFHFNMIGN_1)
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
#[doc = "Possible values of the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNR {
    #[doc = "4-byte aligned"]
    STKALIGN_0,
    #[doc = "8-byte aligned"]
    STKALIGN_1,
}
impl crate::ToBits<bool> for STKALIGNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            STKALIGNR::STKALIGN_0 => false,
            STKALIGNR::STKALIGN_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type STKALIGN_R = crate::FR<bool, STKALIGNR>;
impl STKALIGN_R {
    #[doc = "Checks if the value of the field is `STKALIGN_0`"]
    #[inline(always)]
    pub fn is_stkalign_0(&self) -> bool {
        *self == STKALIGNR::STKALIGN_0
    }
    #[doc = "Checks if the value of the field is `STKALIGN_1`"]
    #[inline(always)]
    pub fn is_stkalign_1(&self) -> bool {
        *self == STKALIGNR::STKALIGN_1
    }
}
#[doc = "Values that can be written to the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNW {
    #[doc = "4-byte aligned"]
    STKALIGN_0,
    #[doc = "8-byte aligned"]
    STKALIGN_1,
}
impl STKALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            STKALIGNW::STKALIGN_0 => false,
            STKALIGNW::STKALIGN_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STKALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _STKALIGNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKALIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn stkalign_0(self) -> &'a mut W {
        self.variant(STKALIGNW::STKALIGN_0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn stkalign_1(self) -> &'a mut W {
        self.variant(STKALIGNW::STKALIGN_1)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits() >> 9) & 0x01) != 0)
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
    pub fn nonbasethrdena(&mut self) -> _NONBASETHRDENAW {
        _NONBASETHRDENAW { w: self }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&mut self) -> _USERSETMPENDW {
        _USERSETMPENDW { w: self }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&mut self) -> _UNALIGN_TRPW {
        _UNALIGN_TRPW { w: self }
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn div_0_trp(&mut self) -> _DIV_0_TRPW {
        _DIV_0_TRPW { w: self }
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub fn bfhfnmign(&mut self) -> _BFHFNMIGNW {
        _BFHFNMIGNW { w: self }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&mut self) -> _STKALIGNW {
        _STKALIGNW { w: self }
    }
}
