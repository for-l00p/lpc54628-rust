#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RES {
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
#[doc = "Possible values of the field `O0RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O0RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O0RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O0RESR::NO_CHANGE => 0,
            O0RESR::SET => 1,
            O0RESR::CLEAR => 2,
            O0RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O0RES_R = crate::FR<u8, O0RESR>;
impl O0RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O0RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O0RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O0RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O0RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O0RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O0RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O0RESW::NO_CHANGE => 0,
            O0RESW::SET => 1,
            O0RESW::CLEAR => 2,
            O0RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O0RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O0RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O0RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O0RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O0RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O0RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O0RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `O1RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O1RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O1RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O1RESR::NO_CHANGE => 0,
            O1RESR::SET => 1,
            O1RESR::CLEAR => 2,
            O1RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O1RES_R = crate::FR<u8, O1RESR>;
impl O1RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O1RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O1RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O1RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O1RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O1RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O1RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O1RESW::NO_CHANGE => 0,
            O1RESW::SET => 1,
            O1RESW::CLEAR => 2,
            O1RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O1RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O1RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O1RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O1RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O1RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O1RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O1RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `O2RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O2RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O2RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O2RESR::NO_CHANGE => 0,
            O2RESR::SET => 1,
            O2RESR::CLEAR => 2,
            O2RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O2RES_R = crate::FR<u8, O2RESR>;
impl O2RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O2RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O2RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O2RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O2RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O2RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O2RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O2RESW::NO_CHANGE => 0,
            O2RESW::SET => 1,
            O2RESW::CLEAR => 2,
            O2RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O2RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O2RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O2RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O2RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O2RESW::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O2RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O2RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `O3RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O3RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O3RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O3RESR::NO_CHANGE => 0,
            O3RESR::SET => 1,
            O3RESR::CLEAR => 2,
            O3RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O3RES_R = crate::FR<u8, O3RESR>;
impl O3RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O3RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O3RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O3RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O3RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O3RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O3RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O3RESW::NO_CHANGE => 0,
            O3RESW::SET => 1,
            O3RESW::CLEAR => 2,
            O3RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O3RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O3RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O3RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O3RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O3RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O3RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O3RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `O4RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O4RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O4RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O4RESR::NO_CHANGE => 0,
            O4RESR::SET => 1,
            O4RESR::CLEAR => 2,
            O4RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O4RES_R = crate::FR<u8, O4RESR>;
impl O4RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O4RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O4RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O4RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O4RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O4RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O4RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O4RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O4RESW::NO_CHANGE => 0,
            O4RESW::SET => 1,
            O4RESW::CLEAR => 2,
            O4RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O4RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O4RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O4RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O4RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O4RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O4RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O4RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `O5RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O5RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O5RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O5RESR::NO_CHANGE => 0,
            O5RESR::SET => 1,
            O5RESR::CLEAR => 2,
            O5RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O5RES_R = crate::FR<u8, O5RESR>;
impl O5RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O5RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O5RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O5RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O5RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O5RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O5RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O5RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O5RESW::NO_CHANGE => 0,
            O5RESW::SET => 1,
            O5RESW::CLEAR => 2,
            O5RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O5RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O5RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O5RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O5RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O5RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O5RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O5RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `O6RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O6RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O6RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O6RESR::NO_CHANGE => 0,
            O6RESR::SET => 1,
            O6RESR::CLEAR => 2,
            O6RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O6RES_R = crate::FR<u8, O6RESR>;
impl O6RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O6RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O6RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O6RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O6RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O6RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O6RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O6RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O6RESW::NO_CHANGE => 0,
            O6RESW::SET => 1,
            O6RESW::CLEAR => 2,
            O6RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O6RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O6RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O6RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O6RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O6RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O6RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O6RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `O7RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O7RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O7RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O7RESR::NO_CHANGE => 0,
            O7RESR::SET => 1,
            O7RESR::CLEAR => 2,
            O7RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O7RES_R = crate::FR<u8, O7RESR>;
impl O7RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O7RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O7RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O7RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O7RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O7RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O7RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O7RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O7RESW::NO_CHANGE => 0,
            O7RESW::SET => 1,
            O7RESW::CLEAR => 2,
            O7RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O7RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O7RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O7RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O7RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O7RESW::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O7RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O7RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `O8RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O8RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O8RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O8RESR::NO_CHANGE => 0,
            O8RESR::SET => 1,
            O8RESR::CLEAR => 2,
            O8RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O8RES_R = crate::FR<u8, O8RESR>;
impl O8RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O8RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O8RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O8RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O8RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O8RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O8RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O8RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O8RESW::NO_CHANGE => 0,
            O8RESW::SET => 1,
            O8RESW::CLEAR => 2,
            O8RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O8RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O8RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O8RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O8RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O8RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O8RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O8RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `O9RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O9RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O9RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O9RESR::NO_CHANGE => 0,
            O9RESR::SET => 1,
            O9RESR::CLEAR => 2,
            O9RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O9RES_R = crate::FR<u8, O9RESR>;
impl O9RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O9RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O9RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O9RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O9RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O9RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O9RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O9RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O9RESW::NO_CHANGE => 0,
            O9RESW::SET => 1,
            O9RESW::CLEAR => 2,
            O9RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O9RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O9RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O9RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O9RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O9RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O9RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O9RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `O10RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O10RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O10RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O10RESR::NO_CHANGE => 0,
            O10RESR::SET => 1,
            O10RESR::CLEAR => 2,
            O10RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O10RES_R = crate::FR<u8, O10RESR>;
impl O10RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O10RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O10RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O10RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O10RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O10RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O10RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O10RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O10RESW::NO_CHANGE => 0,
            O10RESW::SET => 1,
            O10RESW::CLEAR => 2,
            O10RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O10RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O10RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O10RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O10RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O10RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O10RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O10RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `O11RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O11RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O11RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O11RESR::NO_CHANGE => 0,
            O11RESR::SET => 1,
            O11RESR::CLEAR => 2,
            O11RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O11RES_R = crate::FR<u8, O11RESR>;
impl O11RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O11RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O11RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O11RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O11RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O11RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O11RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O11RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O11RESW::NO_CHANGE => 0,
            O11RESW::SET => 1,
            O11RESW::CLEAR => 2,
            O11RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O11RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O11RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O11RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O11RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O11RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O11RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O11RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `O12RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O12RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O12RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O12RESR::NO_CHANGE => 0,
            O12RESR::SET => 1,
            O12RESR::CLEAR => 2,
            O12RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O12RES_R = crate::FR<u8, O12RESR>;
impl O12RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O12RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O12RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O12RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O12RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O12RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O12RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O12RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O12RESW::NO_CHANGE => 0,
            O12RESW::SET => 1,
            O12RESW::CLEAR => 2,
            O12RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O12RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O12RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O12RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O12RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O12RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O12RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O12RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `O13RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O13RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O13RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O13RESR::NO_CHANGE => 0,
            O13RESR::SET => 1,
            O13RESR::CLEAR => 2,
            O13RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O13RES_R = crate::FR<u8, O13RESR>;
impl O13RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O13RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O13RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O13RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O13RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O13RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O13RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O13RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O13RESW::NO_CHANGE => 0,
            O13RESW::SET => 1,
            O13RESW::CLEAR => 2,
            O13RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O13RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O13RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O13RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O13RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O13RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O13RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O13RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `O14RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O14RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O14RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O14RESR::NO_CHANGE => 0,
            O14RESR::SET => 1,
            O14RESR::CLEAR => 2,
            O14RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O14RES_R = crate::FR<u8, O14RESR>;
impl O14RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O14RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O14RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O14RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O14RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O14RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O14RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O14RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O14RESW::NO_CHANGE => 0,
            O14RESW::SET => 1,
            O14RESW::CLEAR => 2,
            O14RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O14RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O14RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O14RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O14RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O14RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O14RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O14RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `O15RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O15RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl crate::ToBits<u8> for O15RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O15RESR::NO_CHANGE => 0,
            O15RESR::SET => 1,
            O15RESR::CLEAR => 2,
            O15RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O15RES_R = crate::FR<u8, O15RESR>;
impl O15RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O15RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O15RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O15RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O15RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O15RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O15RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O15RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O15RESW::NO_CHANGE => 0,
            O15RESW::SET => 1,
            O15RESW::CLEAR => 2,
            O15RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O15RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O15RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O15RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O15RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O15RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O15RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O15RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&self) -> O0RES_R {
        O0RES_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&self) -> O1RES_R {
        O1RES_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&self) -> O2RES_R {
        O2RES_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&self) -> O3RES_R {
        O3RES_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub fn o4res(&self) -> O4RES_R {
        O4RES_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub fn o5res(&self) -> O5RES_R {
        O5RES_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub fn o6res(&self) -> O6RES_R {
        O6RES_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub fn o7res(&self) -> O7RES_R {
        O7RES_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub fn o8res(&self) -> O8RES_R {
        O8RES_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub fn o9res(&self) -> O9RES_R {
        O9RES_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub fn o10res(&self) -> O10RES_R {
        O10RES_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub fn o11res(&self) -> O11RES_R {
        O11RES_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub fn o12res(&self) -> O12RES_R {
        O12RES_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub fn o13res(&self) -> O13RES_R {
        O13RES_R::new(((self.bits() >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub fn o14res(&self) -> O14RES_R {
        O14RES_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub fn o15res(&self) -> O15RES_R {
        O15RES_R::new(((self.bits() >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&mut self) -> _O0RESW {
        _O0RESW { w: self }
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&mut self) -> _O1RESW {
        _O1RESW { w: self }
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&mut self) -> _O2RESW {
        _O2RESW { w: self }
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&mut self) -> _O3RESW {
        _O3RESW { w: self }
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub fn o4res(&mut self) -> _O4RESW {
        _O4RESW { w: self }
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub fn o5res(&mut self) -> _O5RESW {
        _O5RESW { w: self }
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub fn o6res(&mut self) -> _O6RESW {
        _O6RESW { w: self }
    }
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub fn o7res(&mut self) -> _O7RESW {
        _O7RESW { w: self }
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub fn o8res(&mut self) -> _O8RESW {
        _O8RESW { w: self }
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub fn o9res(&mut self) -> _O9RESW {
        _O9RESW { w: self }
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub fn o10res(&mut self) -> _O10RESW {
        _O10RESW { w: self }
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub fn o11res(&mut self) -> _O11RESW {
        _O11RESW { w: self }
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub fn o12res(&mut self) -> _O12RESW {
        _O12RESW { w: self }
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub fn o13res(&mut self) -> _O13RESW {
        _O13RESW { w: self }
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub fn o14res(&mut self) -> _O14RESW {
        _O14RESW { w: self }
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub fn o15res(&mut self) -> _O15RESW {
        _O15RESW { w: self }
    }
}
