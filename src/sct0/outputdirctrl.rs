#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTPUTDIRCTRL {
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
#[doc = "Possible values of the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR0R::INDEPENDENT => 0,
            SETCLR0R::L_REVERSED => 1,
            SETCLR0R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR0_R = crate::FR<u8, SETCLR0R>;
impl SETCLR0_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR0R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR0R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR0R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR0W::INDEPENDENT => 0,
            SETCLR0W::L_REVERSED => 1,
            SETCLR0W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR0W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR0W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR0W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR1R::INDEPENDENT => 0,
            SETCLR1R::L_REVERSED => 1,
            SETCLR1R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR1_R = crate::FR<u8, SETCLR1R>;
impl SETCLR1_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR1R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR1R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR1R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR1W::INDEPENDENT => 0,
            SETCLR1W::L_REVERSED => 1,
            SETCLR1W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR1W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR1W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR1W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR2R::INDEPENDENT => 0,
            SETCLR2R::L_REVERSED => 1,
            SETCLR2R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR2_R = crate::FR<u8, SETCLR2R>;
impl SETCLR2_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR2R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR2R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR2R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR2W::INDEPENDENT => 0,
            SETCLR2W::L_REVERSED => 1,
            SETCLR2W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR2W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR2W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR2W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR3R::INDEPENDENT => 0,
            SETCLR3R::L_REVERSED => 1,
            SETCLR3R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR3_R = crate::FR<u8, SETCLR3R>;
impl SETCLR3_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR3R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR3R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR3R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR3W::INDEPENDENT => 0,
            SETCLR3W::L_REVERSED => 1,
            SETCLR3W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR3W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR3W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR3W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR4R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR4R::INDEPENDENT => 0,
            SETCLR4R::L_REVERSED => 1,
            SETCLR4R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR4_R = crate::FR<u8, SETCLR4R>;
impl SETCLR4_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR4R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR4R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR4R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR4W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR4W::INDEPENDENT => 0,
            SETCLR4W::L_REVERSED => 1,
            SETCLR4W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR4W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR4W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR4W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR5R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR5R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR5R::INDEPENDENT => 0,
            SETCLR5R::L_REVERSED => 1,
            SETCLR5R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR5_R = crate::FR<u8, SETCLR5R>;
impl SETCLR5_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR5R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR5R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR5R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR5W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR5W::INDEPENDENT => 0,
            SETCLR5W::L_REVERSED => 1,
            SETCLR5W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR5W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR5W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR5W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR6R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR6R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR6R::INDEPENDENT => 0,
            SETCLR6R::L_REVERSED => 1,
            SETCLR6R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR6_R = crate::FR<u8, SETCLR6R>;
impl SETCLR6_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR6R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR6R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR6R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR6W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR6W::INDEPENDENT => 0,
            SETCLR6W::L_REVERSED => 1,
            SETCLR6W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR6W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR6W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR6W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR7R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR7R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR7R::INDEPENDENT => 0,
            SETCLR7R::L_REVERSED => 1,
            SETCLR7R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR7_R = crate::FR<u8, SETCLR7R>;
impl SETCLR7_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR7R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR7R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR7R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR7W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR7W::INDEPENDENT => 0,
            SETCLR7W::L_REVERSED => 1,
            SETCLR7W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR7W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR7W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR7W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR8R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR8R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR8R::INDEPENDENT => 0,
            SETCLR8R::L_REVERSED => 1,
            SETCLR8R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR8_R = crate::FR<u8, SETCLR8R>;
impl SETCLR8_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR8R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR8R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR8R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR8W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR8W::INDEPENDENT => 0,
            SETCLR8W::L_REVERSED => 1,
            SETCLR8W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR8W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR8W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR8W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR9R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR9R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR9R::INDEPENDENT => 0,
            SETCLR9R::L_REVERSED => 1,
            SETCLR9R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR9_R = crate::FR<u8, SETCLR9R>;
impl SETCLR9_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR9R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR9R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR9R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR9W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR9W::INDEPENDENT => 0,
            SETCLR9W::L_REVERSED => 1,
            SETCLR9W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR9W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR9W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR9W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR10R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR10R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR10R::INDEPENDENT => 0,
            SETCLR10R::L_REVERSED => 1,
            SETCLR10R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR10_R = crate::FR<u8, SETCLR10R>;
impl SETCLR10_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR10R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR10R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR10R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR10W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR10W::INDEPENDENT => 0,
            SETCLR10W::L_REVERSED => 1,
            SETCLR10W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR10W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR10W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR10W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR11R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR11R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR11R::INDEPENDENT => 0,
            SETCLR11R::L_REVERSED => 1,
            SETCLR11R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR11_R = crate::FR<u8, SETCLR11R>;
impl SETCLR11_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR11R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR11R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR11R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR11W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR11W::INDEPENDENT => 0,
            SETCLR11W::L_REVERSED => 1,
            SETCLR11W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR11W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR11W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR11W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR12R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR12R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR12R::INDEPENDENT => 0,
            SETCLR12R::L_REVERSED => 1,
            SETCLR12R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR12_R = crate::FR<u8, SETCLR12R>;
impl SETCLR12_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR12R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR12R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR12R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR12W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR12W::INDEPENDENT => 0,
            SETCLR12W::L_REVERSED => 1,
            SETCLR12W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR12W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR12W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR12W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR12W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR12W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR13R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR13R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR13R::INDEPENDENT => 0,
            SETCLR13R::L_REVERSED => 1,
            SETCLR13R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR13_R = crate::FR<u8, SETCLR13R>;
impl SETCLR13_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR13R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR13R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR13R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR13W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR13W::INDEPENDENT => 0,
            SETCLR13W::L_REVERSED => 1,
            SETCLR13W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR13W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR13W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR13W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR13W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR13W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR14R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR14R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR14R::INDEPENDENT => 0,
            SETCLR14R::L_REVERSED => 1,
            SETCLR14R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR14_R = crate::FR<u8, SETCLR14R>;
impl SETCLR14_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR14R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR14R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR14R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR14W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR14W::INDEPENDENT => 0,
            SETCLR14W::L_REVERSED => 1,
            SETCLR14W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR14W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR14W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR14W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR14W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR14W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR15R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl crate::ToBits<u8> for SETCLR15R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR15R::INDEPENDENT => 0,
            SETCLR15R::L_REVERSED => 1,
            SETCLR15R::H_REVERSED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR15_R = crate::FR<u8, SETCLR15R>;
impl SETCLR15_R {
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR15R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline(always)]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR15R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline(always)]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR15R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR15W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR15W::INDEPENDENT => 0,
            SETCLR15W::L_REVERSED => 1,
            SETCLR15W::H_REVERSED => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR15W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR15W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR15W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR15W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR15W::H_REVERSED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&self) -> SETCLR0_R {
        SETCLR0_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&self) -> SETCLR1_R {
        SETCLR1_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&self) -> SETCLR2_R {
        SETCLR2_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&self) -> SETCLR3_R {
        SETCLR3_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr4(&self) -> SETCLR4_R {
        SETCLR4_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr5(&self) -> SETCLR5_R {
        SETCLR5_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr6(&self) -> SETCLR6_R {
        SETCLR6_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr7(&self) -> SETCLR7_R {
        SETCLR7_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr8(&self) -> SETCLR8_R {
        SETCLR8_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr9(&self) -> SETCLR9_R {
        SETCLR9_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr10(&self) -> SETCLR10_R {
        SETCLR10_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr11(&self) -> SETCLR11_R {
        SETCLR11_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr12(&self) -> SETCLR12_R {
        SETCLR12_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr13(&self) -> SETCLR13_R {
        SETCLR13_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr14(&self) -> SETCLR14_R {
        SETCLR14_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr15(&self) -> SETCLR15_R {
        SETCLR15_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&mut self) -> _SETCLR0W {
        _SETCLR0W { w: self }
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&mut self) -> _SETCLR1W {
        _SETCLR1W { w: self }
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&mut self) -> _SETCLR2W {
        _SETCLR2W { w: self }
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&mut self) -> _SETCLR3W {
        _SETCLR3W { w: self }
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr4(&mut self) -> _SETCLR4W {
        _SETCLR4W { w: self }
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr5(&mut self) -> _SETCLR5W {
        _SETCLR5W { w: self }
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr6(&mut self) -> _SETCLR6W {
        _SETCLR6W { w: self }
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr7(&mut self) -> _SETCLR7W {
        _SETCLR7W { w: self }
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr8(&mut self) -> _SETCLR8W {
        _SETCLR8W { w: self }
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr9(&mut self) -> _SETCLR9W {
        _SETCLR9W { w: self }
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr10(&mut self) -> _SETCLR10W {
        _SETCLR10W { w: self }
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr11(&mut self) -> _SETCLR11W {
        _SETCLR11W { w: self }
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr12(&mut self) -> _SETCLR12W {
        _SETCLR12W { w: self }
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr13(&mut self) -> _SETCLR13W {
        _SETCLR13W { w: self }
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr14(&mut self) -> _SETCLR14W {
        _SETCLR14W { w: self }
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr15(&mut self) -> _SETCLR15W {
        _SETCLR15W { w: self }
    }
}
