#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMCFG {
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
#[doc = "Possible values of the field `PROD_ENDPTS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS0R {
    #[doc = "No effect. Slice 0 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS0R::NO_EFFECT => false,
            PROD_ENDPTS0R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS0_R = crate::FR<bool, PROD_ENDPTS0R>;
impl PROD_ENDPTS0_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS0R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS0R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS0W {
    #[doc = "No effect. Slice 0 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS0W::NO_EFFECT => false,
            PROD_ENDPTS0W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 0 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS0W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS0W::ENDPOINT)
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
#[doc = "Possible values of the field `PROD_ENDPTS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS1R {
    #[doc = "No effect. Slice 1 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS1R::NO_EFFECT => false,
            PROD_ENDPTS1R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS1_R = crate::FR<bool, PROD_ENDPTS1R>;
impl PROD_ENDPTS1_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS1R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS1R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS1W {
    #[doc = "No effect. Slice 1 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS1W::NO_EFFECT => false,
            PROD_ENDPTS1W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 1 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS1W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS1W::ENDPOINT)
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
#[doc = "Possible values of the field `PROD_ENDPTS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS2R {
    #[doc = "No effect. Slice 2 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS2R::NO_EFFECT => false,
            PROD_ENDPTS2R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS2_R = crate::FR<bool, PROD_ENDPTS2R>;
impl PROD_ENDPTS2_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS2R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS2R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS2W {
    #[doc = "No effect. Slice 2 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS2W::NO_EFFECT => false,
            PROD_ENDPTS2W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 2 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS2W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS2W::ENDPOINT)
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
#[doc = "Possible values of the field `PROD_ENDPTS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS3R {
    #[doc = "No effect. Slice 3 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS3R::NO_EFFECT => false,
            PROD_ENDPTS3R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS3_R = crate::FR<bool, PROD_ENDPTS3R>;
impl PROD_ENDPTS3_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS3R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS3R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS3W {
    #[doc = "No effect. Slice 3 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS3W::NO_EFFECT => false,
            PROD_ENDPTS3W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 3 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS3W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS3W::ENDPOINT)
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
#[doc = "Possible values of the field `PROD_ENDPTS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS4R {
    #[doc = "No effect. Slice 4 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS4R::NO_EFFECT => false,
            PROD_ENDPTS4R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS4_R = crate::FR<bool, PROD_ENDPTS4R>;
impl PROD_ENDPTS4_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS4R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS4R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS4W {
    #[doc = "No effect. Slice 4 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS4W::NO_EFFECT => false,
            PROD_ENDPTS4W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS4W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 4 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS4W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS4W::ENDPOINT)
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
#[doc = "Possible values of the field `PROD_ENDPTS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS5R {
    #[doc = "No effect. Slice 5 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS5R::NO_EFFECT => false,
            PROD_ENDPTS5R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS5_R = crate::FR<bool, PROD_ENDPTS5R>;
impl PROD_ENDPTS5_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS5R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS5R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS5W {
    #[doc = "No effect. Slice 5 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS5W::NO_EFFECT => false,
            PROD_ENDPTS5W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS5W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 5 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS5W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS5W::ENDPOINT)
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
#[doc = "Possible values of the field `PROD_ENDPTS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS6R {
    #[doc = "No effect. Slice 6 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl crate::ToBits<bool> for PROD_ENDPTS6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS6R::NO_EFFECT => false,
            PROD_ENDPTS6R::ENDPOINT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROD_ENDPTS6_R = crate::FR<bool, PROD_ENDPTS6R>;
impl PROD_ENDPTS6_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS6R::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS6R::ENDPOINT
    }
}
#[doc = "Values that can be written to the field `PROD_ENDPTS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROD_ENDPTS6W {
    #[doc = "No effect. Slice 6 is not an endpoint."]
    NO_EFFECT,
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT,
}
impl PROD_ENDPTS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROD_ENDPTS6W::NO_EFFECT => false,
            PROD_ENDPTS6W::ENDPOINT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROD_ENDPTS6W<'a> {
    w: &'a mut W,
}
impl<'a> _PROD_ENDPTS6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROD_ENDPTS6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Slice 6 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS6W::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS6W::ENDPOINT)
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
#[doc = "Possible values of the field `CFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG0R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG0R::CONSTANT_HIGH => 0,
            CFG0R::STICKY_RISING_EDGE => 1,
            CFG0R::STICKY_FALLING_EDGE => 2,
            CFG0R::STICKY_RISING_FALLING_EDGE => 3,
            CFG0R::HIGH_LEVEL => 4,
            CFG0R::LOW_LEVEL => 5,
            CFG0R::CONSTANT_ZERO => 6,
            CFG0R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG0_R = crate::FR<u8, CFG0R>;
impl CFG0_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG0R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG0R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG0R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG0R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG0R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG0R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG0R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG0R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG0W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG0W::CONSTANT_HIGH => 0,
            CFG0W::STICKY_RISING_EDGE => 1,
            CFG0W::STICKY_FALLING_EDGE => 2,
            CFG0W::STICKY_RISING_FALLING_EDGE => 3,
            CFG0W::HIGH_LEVEL => 4,
            CFG0W::LOW_LEVEL => 5,
            CFG0W::CONSTANT_ZERO => 6,
            CFG0W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG0W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG0W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG0W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG0W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG0W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG0W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG0W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG0W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG0W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG1R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG1R::CONSTANT_HIGH => 0,
            CFG1R::STICKY_RISING_EDGE => 1,
            CFG1R::STICKY_FALLING_EDGE => 2,
            CFG1R::STICKY_RISING_FALLING_EDGE => 3,
            CFG1R::HIGH_LEVEL => 4,
            CFG1R::LOW_LEVEL => 5,
            CFG1R::CONSTANT_ZERO => 6,
            CFG1R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG1_R = crate::FR<u8, CFG1R>;
impl CFG1_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG1R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG1R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG1R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG1R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG1R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG1R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG1R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG1R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG1W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG1W::CONSTANT_HIGH => 0,
            CFG1W::STICKY_RISING_EDGE => 1,
            CFG1W::STICKY_FALLING_EDGE => 2,
            CFG1W::STICKY_RISING_FALLING_EDGE => 3,
            CFG1W::HIGH_LEVEL => 4,
            CFG1W::LOW_LEVEL => 5,
            CFG1W::CONSTANT_ZERO => 6,
            CFG1W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG1W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG1W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG1W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG1W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG1W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG1W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG1W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG1W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG1W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `CFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG2R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG2R::CONSTANT_HIGH => 0,
            CFG2R::STICKY_RISING_EDGE => 1,
            CFG2R::STICKY_FALLING_EDGE => 2,
            CFG2R::STICKY_RISING_FALLING_EDGE => 3,
            CFG2R::HIGH_LEVEL => 4,
            CFG2R::LOW_LEVEL => 5,
            CFG2R::CONSTANT_ZERO => 6,
            CFG2R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG2_R = crate::FR<u8, CFG2R>;
impl CFG2_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG2R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG2R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG2R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG2R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG2R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG2R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG2R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG2R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG2W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG2W::CONSTANT_HIGH => 0,
            CFG2W::STICKY_RISING_EDGE => 1,
            CFG2W::STICKY_FALLING_EDGE => 2,
            CFG2W::STICKY_RISING_FALLING_EDGE => 3,
            CFG2W::HIGH_LEVEL => 4,
            CFG2W::LOW_LEVEL => 5,
            CFG2W::CONSTANT_ZERO => 6,
            CFG2W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG2W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG2W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG2W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG2W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG2W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG2W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG2W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG2W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG2W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `CFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG3R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG3R::CONSTANT_HIGH => 0,
            CFG3R::STICKY_RISING_EDGE => 1,
            CFG3R::STICKY_FALLING_EDGE => 2,
            CFG3R::STICKY_RISING_FALLING_EDGE => 3,
            CFG3R::HIGH_LEVEL => 4,
            CFG3R::LOW_LEVEL => 5,
            CFG3R::CONSTANT_ZERO => 6,
            CFG3R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG3_R = crate::FR<u8, CFG3R>;
impl CFG3_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG3R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG3R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG3R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG3R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG3R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG3R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG3R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG3R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG3W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG3W::CONSTANT_HIGH => 0,
            CFG3W::STICKY_RISING_EDGE => 1,
            CFG3W::STICKY_FALLING_EDGE => 2,
            CFG3W::STICKY_RISING_FALLING_EDGE => 3,
            CFG3W::HIGH_LEVEL => 4,
            CFG3W::LOW_LEVEL => 5,
            CFG3W::CONSTANT_ZERO => 6,
            CFG3W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG3W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG3W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG3W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG3W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG3W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG3W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG3W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG3W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG3W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `CFG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG4R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG4R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG4R::CONSTANT_HIGH => 0,
            CFG4R::STICKY_RISING_EDGE => 1,
            CFG4R::STICKY_FALLING_EDGE => 2,
            CFG4R::STICKY_RISING_FALLING_EDGE => 3,
            CFG4R::HIGH_LEVEL => 4,
            CFG4R::LOW_LEVEL => 5,
            CFG4R::CONSTANT_ZERO => 6,
            CFG4R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG4_R = crate::FR<u8, CFG4R>;
impl CFG4_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG4R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG4R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG4R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG4R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG4R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG4R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG4R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG4R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG4W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG4W::CONSTANT_HIGH => 0,
            CFG4W::STICKY_RISING_EDGE => 1,
            CFG4W::STICKY_FALLING_EDGE => 2,
            CFG4W::STICKY_RISING_FALLING_EDGE => 3,
            CFG4W::HIGH_LEVEL => 4,
            CFG4W::LOW_LEVEL => 5,
            CFG4W::CONSTANT_ZERO => 6,
            CFG4W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG4W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG4W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG4W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG4W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG4W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG4W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG4W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG4W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG4W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `CFG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG5R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG5R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG5R::CONSTANT_HIGH => 0,
            CFG5R::STICKY_RISING_EDGE => 1,
            CFG5R::STICKY_FALLING_EDGE => 2,
            CFG5R::STICKY_RISING_FALLING_EDGE => 3,
            CFG5R::HIGH_LEVEL => 4,
            CFG5R::LOW_LEVEL => 5,
            CFG5R::CONSTANT_ZERO => 6,
            CFG5R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG5_R = crate::FR<u8, CFG5R>;
impl CFG5_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG5R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG5R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG5R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG5R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG5R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG5R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG5R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG5R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG5W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG5W::CONSTANT_HIGH => 0,
            CFG5W::STICKY_RISING_EDGE => 1,
            CFG5W::STICKY_FALLING_EDGE => 2,
            CFG5W::STICKY_RISING_FALLING_EDGE => 3,
            CFG5W::HIGH_LEVEL => 4,
            CFG5W::LOW_LEVEL => 5,
            CFG5W::CONSTANT_ZERO => 6,
            CFG5W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG5W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG5W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG5W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG5W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG5W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG5W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG5W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG5W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG5W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `CFG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG6R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG6R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG6R::CONSTANT_HIGH => 0,
            CFG6R::STICKY_RISING_EDGE => 1,
            CFG6R::STICKY_FALLING_EDGE => 2,
            CFG6R::STICKY_RISING_FALLING_EDGE => 3,
            CFG6R::HIGH_LEVEL => 4,
            CFG6R::LOW_LEVEL => 5,
            CFG6R::CONSTANT_ZERO => 6,
            CFG6R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG6_R = crate::FR<u8, CFG6R>;
impl CFG6_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG6R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG6R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG6R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG6R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG6R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG6R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG6R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG6R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG6W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG6W::CONSTANT_HIGH => 0,
            CFG6W::STICKY_RISING_EDGE => 1,
            CFG6W::STICKY_FALLING_EDGE => 2,
            CFG6W::STICKY_RISING_FALLING_EDGE => 3,
            CFG6W::HIGH_LEVEL => 4,
            CFG6W::LOW_LEVEL => 5,
            CFG6W::CONSTANT_ZERO => 6,
            CFG6W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG6W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG6W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG6W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG6W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG6W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG6W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG6W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG6W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG6W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `CFG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG7R {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl crate::ToBits<u8> for CFG7R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFG7R::CONSTANT_HIGH => 0,
            CFG7R::STICKY_RISING_EDGE => 1,
            CFG7R::STICKY_FALLING_EDGE => 2,
            CFG7R::STICKY_RISING_FALLING_EDGE => 3,
            CFG7R::HIGH_LEVEL => 4,
            CFG7R::LOW_LEVEL => 5,
            CFG7R::CONSTANT_ZERO => 6,
            CFG7R::EVENT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFG7_R = crate::FR<u8, CFG7R>;
impl CFG7_R {
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG7R::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG7R::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG7R::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG7R::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG7R::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG7R::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG7R::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG7R::EVENT
    }
}
#[doc = "Values that can be written to the field `CFG7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFG7W {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT,
}
impl CFG7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFG7W::CONSTANT_HIGH => 0,
            CFG7W::STICKY_RISING_EDGE => 1,
            CFG7W::STICKY_FALLING_EDGE => 2,
            CFG7W::STICKY_RISING_FALLING_EDGE => 3,
            CFG7W::HIGH_LEVEL => 4,
            CFG7W::LOW_LEVEL => 5,
            CFG7W::CONSTANT_ZERO => 6,
            CFG7W::EVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFG7W<'a> {
    w: &'a mut W,
}
impl<'a> _CFG7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG7W::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG7W::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG7W::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG7W::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG7W::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG7W::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG7W::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG7W::EVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&self) -> PROD_ENDPTS0_R {
        PROD_ENDPTS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&self) -> PROD_ENDPTS1_R {
        PROD_ENDPTS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&self) -> PROD_ENDPTS2_R {
        PROD_ENDPTS2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&self) -> PROD_ENDPTS3_R {
        PROD_ENDPTS3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&self) -> PROD_ENDPTS4_R {
        PROD_ENDPTS4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&self) -> PROD_ENDPTS5_R {
        PROD_ENDPTS5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&self) -> PROD_ENDPTS6_R {
        PROD_ENDPTS6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits() >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits() >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits() >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits() >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits() >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits() >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&mut self) -> _PROD_ENDPTS0W {
        _PROD_ENDPTS0W { w: self }
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&mut self) -> _PROD_ENDPTS1W {
        _PROD_ENDPTS1W { w: self }
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&mut self) -> _PROD_ENDPTS2W {
        _PROD_ENDPTS2W { w: self }
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&mut self) -> _PROD_ENDPTS3W {
        _PROD_ENDPTS3W { w: self }
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&mut self) -> _PROD_ENDPTS4W {
        _PROD_ENDPTS4W { w: self }
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&mut self) -> _PROD_ENDPTS5W {
        _PROD_ENDPTS5W { w: self }
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&mut self) -> _PROD_ENDPTS6W {
        _PROD_ENDPTS6W { w: self }
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&mut self) -> _CFG0W {
        _CFG0W { w: self }
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&mut self) -> _CFG1W {
        _CFG1W { w: self }
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&mut self) -> _CFG2W {
        _CFG2W { w: self }
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&mut self) -> _CFG3W {
        _CFG3W { w: self }
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&mut self) -> _CFG4W {
        _CFG4W { w: self }
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&mut self) -> _CFG5W {
        _CFG5W { w: self }
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&mut self) -> _CFG6W {
        _CFG6W { w: self }
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&mut self) -> _CFG7W {
        _CFG7W { w: self }
    }
}
