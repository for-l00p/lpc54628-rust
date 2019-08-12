#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DFSR {
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
#[doc = "Possible values of the field `HALTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTEDR {
    #[doc = "No active halt request debug event"]
    HALTED_0,
    #[doc = "Halt request debug event active"]
    HALTED_1,
}
impl crate::ToBits<bool> for HALTEDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            HALTEDR::HALTED_0 => false,
            HALTEDR::HALTED_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type HALTED_R = crate::FR<bool, HALTEDR>;
impl HALTED_R {
    #[doc = "Checks if the value of the field is `HALTED_0`"]
    #[inline(always)]
    pub fn is_halted_0(&self) -> bool {
        *self == HALTEDR::HALTED_0
    }
    #[doc = "Checks if the value of the field is `HALTED_1`"]
    #[inline(always)]
    pub fn is_halted_1(&self) -> bool {
        *self == HALTEDR::HALTED_1
    }
}
#[doc = "Values that can be written to the field `HALTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTEDW {
    #[doc = "No active halt request debug event"]
    HALTED_0,
    #[doc = "Halt request debug event active"]
    HALTED_1,
}
impl HALTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTEDW::HALTED_0 => false,
            HALTEDW::HALTED_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _HALTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTEDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No active halt request debug event"]
    #[inline(always)]
    pub fn halted_0(self) -> &'a mut W {
        self.variant(HALTEDW::HALTED_0)
    }
    #[doc = "Halt request debug event active"]
    #[inline(always)]
    pub fn halted_1(self) -> &'a mut W {
        self.variant(HALTEDW::HALTED_1)
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
#[doc = "Possible values of the field `BKPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPTR {
    #[doc = "No current breakpoint debug event"]
    BKPT_0,
    #[doc = "At least one current breakpoint debug event"]
    BKPT_1,
}
impl crate::ToBits<bool> for BKPTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BKPTR::BKPT_0 => false,
            BKPTR::BKPT_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BKPT_R = crate::FR<bool, BKPTR>;
impl BKPT_R {
    #[doc = "Checks if the value of the field is `BKPT_0`"]
    #[inline(always)]
    pub fn is_bkpt_0(&self) -> bool {
        *self == BKPTR::BKPT_0
    }
    #[doc = "Checks if the value of the field is `BKPT_1`"]
    #[inline(always)]
    pub fn is_bkpt_1(&self) -> bool {
        *self == BKPTR::BKPT_1
    }
}
#[doc = "Values that can be written to the field `BKPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPTW {
    #[doc = "No current breakpoint debug event"]
    BKPT_0,
    #[doc = "At least one current breakpoint debug event"]
    BKPT_1,
}
impl BKPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BKPTW::BKPT_0 => false,
            BKPTW::BKPT_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BKPTW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No current breakpoint debug event"]
    #[inline(always)]
    pub fn bkpt_0(self) -> &'a mut W {
        self.variant(BKPTW::BKPT_0)
    }
    #[doc = "At least one current breakpoint debug event"]
    #[inline(always)]
    pub fn bkpt_1(self) -> &'a mut W {
        self.variant(BKPTW::BKPT_1)
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
#[doc = "Possible values of the field `DWTTRAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTTRAPR {
    #[doc = "No current debug events generated by the DWT"]
    DWTTRAP_0,
    #[doc = "At least one current debug event generated by the DWT"]
    DWTTRAP_1,
}
impl crate::ToBits<bool> for DWTTRAPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DWTTRAPR::DWTTRAP_0 => false,
            DWTTRAPR::DWTTRAP_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DWTTRAP_R = crate::FR<bool, DWTTRAPR>;
impl DWTTRAP_R {
    #[doc = "Checks if the value of the field is `DWTTRAP_0`"]
    #[inline(always)]
    pub fn is_dwttrap_0(&self) -> bool {
        *self == DWTTRAPR::DWTTRAP_0
    }
    #[doc = "Checks if the value of the field is `DWTTRAP_1`"]
    #[inline(always)]
    pub fn is_dwttrap_1(&self) -> bool {
        *self == DWTTRAPR::DWTTRAP_1
    }
}
#[doc = "Values that can be written to the field `DWTTRAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTTRAPW {
    #[doc = "No current debug events generated by the DWT"]
    DWTTRAP_0,
    #[doc = "At least one current debug event generated by the DWT"]
    DWTTRAP_1,
}
impl DWTTRAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DWTTRAPW::DWTTRAP_0 => false,
            DWTTRAPW::DWTTRAP_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DWTTRAPW<'a> {
    w: &'a mut W,
}
impl<'a> _DWTTRAPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWTTRAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No current debug events generated by the DWT"]
    #[inline(always)]
    pub fn dwttrap_0(self) -> &'a mut W {
        self.variant(DWTTRAPW::DWTTRAP_0)
    }
    #[doc = "At least one current debug event generated by the DWT"]
    #[inline(always)]
    pub fn dwttrap_1(self) -> &'a mut W {
        self.variant(DWTTRAPW::DWTTRAP_1)
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
#[doc = "Possible values of the field `VCATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCATCHR {
    #[doc = "No Vector catch triggered"]
    VCATCH_0,
    #[doc = "Vector catch triggered"]
    VCATCH_1,
}
impl crate::ToBits<bool> for VCATCHR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            VCATCHR::VCATCH_0 => false,
            VCATCHR::VCATCH_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type VCATCH_R = crate::FR<bool, VCATCHR>;
impl VCATCH_R {
    #[doc = "Checks if the value of the field is `VCATCH_0`"]
    #[inline(always)]
    pub fn is_vcatch_0(&self) -> bool {
        *self == VCATCHR::VCATCH_0
    }
    #[doc = "Checks if the value of the field is `VCATCH_1`"]
    #[inline(always)]
    pub fn is_vcatch_1(&self) -> bool {
        *self == VCATCHR::VCATCH_1
    }
}
#[doc = "Values that can be written to the field `VCATCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCATCHW {
    #[doc = "No Vector catch triggered"]
    VCATCH_0,
    #[doc = "Vector catch triggered"]
    VCATCH_1,
}
impl VCATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            VCATCHW::VCATCH_0 => false,
            VCATCHW::VCATCH_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _VCATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _VCATCHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCATCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Vector catch triggered"]
    #[inline(always)]
    pub fn vcatch_0(self) -> &'a mut W {
        self.variant(VCATCHW::VCATCH_0)
    }
    #[doc = "Vector catch triggered"]
    #[inline(always)]
    pub fn vcatch_1(self) -> &'a mut W {
        self.variant(VCATCHW::VCATCH_1)
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
#[doc = "Possible values of the field `EXTERNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNALR {
    #[doc = "No EDBGRQ debug event"]
    EXTERNAL_0,
    #[doc = "EDBGRQ debug event"]
    EXTERNAL_1,
}
impl crate::ToBits<bool> for EXTERNALR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EXTERNALR::EXTERNAL_0 => false,
            EXTERNALR::EXTERNAL_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXTERNAL_R = crate::FR<bool, EXTERNALR>;
impl EXTERNAL_R {
    #[doc = "Checks if the value of the field is `EXTERNAL_0`"]
    #[inline(always)]
    pub fn is_external_0(&self) -> bool {
        *self == EXTERNALR::EXTERNAL_0
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_1`"]
    #[inline(always)]
    pub fn is_external_1(&self) -> bool {
        *self == EXTERNALR::EXTERNAL_1
    }
}
#[doc = "Values that can be written to the field `EXTERNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNALW {
    #[doc = "No EDBGRQ debug event"]
    EXTERNAL_0,
    #[doc = "EDBGRQ debug event"]
    EXTERNAL_1,
}
impl EXTERNALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTERNALW::EXTERNAL_0 => false,
            EXTERNALW::EXTERNAL_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXTERNALW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTERNALW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTERNALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No EDBGRQ debug event"]
    #[inline(always)]
    pub fn external_0(self) -> &'a mut W {
        self.variant(EXTERNALW::EXTERNAL_0)
    }
    #[doc = "EDBGRQ debug event"]
    #[inline(always)]
    pub fn external_1(self) -> &'a mut W {
        self.variant(EXTERNALW::EXTERNAL_1)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits() >> 4) & 0x01) != 0)
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
    pub fn halted(&mut self) -> _HALTEDW {
        _HALTEDW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bkpt(&mut self) -> _BKPTW {
        _BKPTW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn dwttrap(&mut self) -> _DWTTRAPW {
        _DWTTRAPW { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn vcatch(&mut self) -> _VCATCHW {
        _VCATCHW { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn external(&mut self) -> _EXTERNALW {
        _EXTERNALW { w: self }
    }
}
