#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BODCTRL {
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
#[doc = "Possible values of the field `BODRSTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTLEVR {
    #[doc = "Level 0: 1.5 V"]
    LEVEL0,
    #[doc = "Level 1: 1.85 V"]
    LEVEL1,
    #[doc = "Level 2: 2.0 V"]
    LEVEL2,
    #[doc = "Level 3: 2.3 V"]
    LEVEL3,
}
impl crate::ToBits<u8> for BODRSTLEVR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVR::LEVEL0 => 0,
            BODRSTLEVR::LEVEL1 => 1,
            BODRSTLEVR::LEVEL2 => 2,
            BODRSTLEVR::LEVEL3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODRSTLEV_R = crate::FR<u8, BODRSTLEVR>;
impl BODRSTLEV_R {
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODRSTLEVR::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODRSTLEVR::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODRSTLEVR::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODRSTLEVR::LEVEL3
    }
}
#[doc = "Values that can be written to the field `BODRSTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTLEVW {
    #[doc = "Level 0: 1.5 V"]
    LEVEL0,
    #[doc = "Level 1: 1.85 V"]
    LEVEL1,
    #[doc = "Level 2: 2.0 V"]
    LEVEL2,
    #[doc = "Level 3: 2.3 V"]
    LEVEL3,
}
impl BODRSTLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVW::LEVEL0 => 0,
            BODRSTLEVW::LEVEL1 => 1,
            BODRSTLEVW::LEVEL2 => 2,
            BODRSTLEVW::LEVEL3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODRSTLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTLEVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTLEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0: 1.5 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL0)
    }
    #[doc = "Level 1: 1.85 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL1)
    }
    #[doc = "Level 2: 2.0 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL2)
    }
    #[doc = "Level 3: 2.3 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `BODRSTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENAR {
    #[doc = "Disable reset function."]
    DISABLE,
    #[doc = "Enable reset function."]
    ENABLE,
}
impl crate::ToBits<bool> for BODRSTENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODRSTENAR::DISABLE => false,
            BODRSTENAR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODRSTENA_R = crate::FR<bool, BODRSTENAR>;
impl BODRSTENA_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODRSTENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `BODRSTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENAW {
    #[doc = "Disable reset function."]
    DISABLE,
    #[doc = "Enable reset function."]
    ENABLE,
}
impl BODRSTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRSTENAW::DISABLE => false,
            BODRSTENAW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODRSTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODRSTENAW::DISABLE)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENAW::ENABLE)
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
#[doc = "Possible values of the field `BODINTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTLEVR {
    #[doc = "Level 0: 2.05 V"]
    LEVEL0,
    #[doc = "Level 1: 2.45 V"]
    LEVEL1,
    #[doc = "Level 2: 2.75 V"]
    LEVEL2,
    #[doc = "Level 3: 3.05 V"]
    LEVEL3,
}
impl crate::ToBits<u8> for BODINTLEVR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BODINTLEVR::LEVEL0 => 0,
            BODINTLEVR::LEVEL1 => 1,
            BODINTLEVR::LEVEL2 => 2,
            BODINTLEVR::LEVEL3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODINTLEV_R = crate::FR<u8, BODINTLEVR>;
impl BODINTLEV_R {
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODINTLEVR::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODINTLEVR::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODINTLEVR::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODINTLEVR::LEVEL3
    }
}
#[doc = "Values that can be written to the field `BODINTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTLEVW {
    #[doc = "Level 0: 2.05 V"]
    LEVEL0,
    #[doc = "Level 1: 2.45 V"]
    LEVEL1,
    #[doc = "Level 2: 2.75 V"]
    LEVEL2,
    #[doc = "Level 3: 3.05 V"]
    LEVEL3,
}
impl BODINTLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODINTLEVW::LEVEL0 => 0,
            BODINTLEVW::LEVEL1 => 1,
            BODINTLEVW::LEVEL2 => 2,
            BODINTLEVW::LEVEL3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODINTLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _BODINTLEVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTLEVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0: 2.05 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODINTLEVW::LEVEL0)
    }
    #[doc = "Level 1: 2.45 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODINTLEVW::LEVEL1)
    }
    #[doc = "Level 2: 2.75 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODINTLEVW::LEVEL2)
    }
    #[doc = "Level 3: 3.05 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODINTLEVW::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `BODINTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTENAR {
    #[doc = "Disable interrupt function."]
    DISABLE,
    #[doc = "Enable interrupt function."]
    ENABLE,
}
impl crate::ToBits<bool> for BODINTENAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODINTENAR::DISABLE => false,
            BODINTENAR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODINTENA_R = crate::FR<bool, BODINTENAR>;
impl BODINTENA_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODINTENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODINTENAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `BODINTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTENAW {
    #[doc = "Disable interrupt function."]
    DISABLE,
    #[doc = "Enable interrupt function."]
    ENABLE,
}
impl BODINTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BODINTENAW::DISABLE => false,
            BODINTENAW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODINTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BODINTENAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODINTENAW::DISABLE)
    }
    #[doc = "Enable interrupt function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODINTENAW::ENABLE)
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
#[doc = r"Reader of the field"]
pub type BODRSTSTAT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BODRSTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTSTATW<'a> {
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
#[doc = r"Reader of the field"]
pub type BODINTSTAT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BODINTSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _BODINTSTATW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&self) -> BODINTLEV_R {
        BODINTLEV_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&self) -> BODINTENA_R {
        BODINTENA_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&self) -> BODRSTSTAT_R {
        BODRSTSTAT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&self) -> BODINTSTAT_R {
        BODINTSTAT_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> _BODRSTLEVW {
        _BODRSTLEVW { w: self }
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> _BODRSTENAW {
        _BODRSTENAW { w: self }
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&mut self) -> _BODINTLEVW {
        _BODINTLEVW { w: self }
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&mut self) -> _BODINTENAW {
        _BODINTENAW { w: self }
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&mut self) -> _BODRSTSTATW {
        _BODRSTSTATW { w: self }
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&mut self) -> _BODINTSTATW {
        _BODINTSTATW { w: self }
    }
}
