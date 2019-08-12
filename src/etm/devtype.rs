#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVTYPE {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MajorType`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJORTYPER {
    #[doc = "Trace source"]
    MAJORTYPE_3,
}
impl crate::ToBits<u8> for MAJORTYPER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MAJORTYPER::MAJORTYPE_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MAJORTYPE_R = crate::FR<u8, MAJORTYPER>;
impl MAJORTYPE_R {
    #[doc = "Checks if the value of the field is `MAJORTYPE_3`"]
    #[inline(always)]
    pub fn is_major_type_3(&self) -> bool {
        *self == MAJORTYPER::MAJORTYPE_3
    }
}
#[doc = "Possible values of the field `SubType`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBTYPER {
    #[doc = "Processor trace"]
    SUBTYPE_1,
}
impl crate::ToBits<u8> for SUBTYPER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SUBTYPER::SUBTYPE_1 => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SUBTYPE_R = crate::FR<u8, SUBTYPER>;
impl SUBTYPE_R {
    #[doc = "Checks if the value of the field is `SUBTYPE_1`"]
    #[inline(always)]
    pub fn is_sub_type_1(&self) -> bool {
        *self == SUBTYPER::SUBTYPE_1
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Major Type and Class"]
    #[inline(always)]
    pub fn major_type(&self) -> MAJORTYPE_R {
        MAJORTYPE_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub Type"]
    #[inline(always)]
    pub fn sub_type(&self) -> SUBTYPE_R {
        SUBTYPE_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
