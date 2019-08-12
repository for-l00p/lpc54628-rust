#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CIDR1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PREAMBLE_R = crate::FR<u8, u8>;
#[doc = "Possible values of the field `ComponentClass`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPONENTCLASSR {
    #[doc = "ROM table."]
    COMPONENTCLASS_1,
    #[doc = "CoreSight component."]
    COMPONENTCLASS_9,
    #[doc = "PrimeCell of system component with no standardized register layout, for backward compatibility."]
    COMPONENTCLASS_15,
}
impl crate::ToBits<u8> for COMPONENTCLASSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            COMPONENTCLASSR::COMPONENTCLASS_1 => 1,
            COMPONENTCLASSR::COMPONENTCLASS_9 => 9,
            COMPONENTCLASSR::COMPONENTCLASS_15 => 15,
        }
    }
}
#[doc = r"Reader of the field"]
pub type COMPONENTCLASS_R = crate::FR<u8, COMPONENTCLASSR>;
impl COMPONENTCLASS_R {
    #[doc = "Checks if the value of the field is `COMPONENTCLASS_1`"]
    #[inline(always)]
    pub fn is_component_class_1(&self) -> bool {
        *self == COMPONENTCLASSR::COMPONENTCLASS_1
    }
    #[doc = "Checks if the value of the field is `COMPONENTCLASS_9`"]
    #[inline(always)]
    pub fn is_component_class_9(&self) -> bool {
        *self == COMPONENTCLASSR::COMPONENTCLASS_9
    }
    #[doc = "Checks if the value of the field is `COMPONENTCLASS_15`"]
    #[inline(always)]
    pub fn is_component_class_15(&self) -> bool {
        *self == COMPONENTCLASSR::COMPONENTCLASS_15
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Preamble"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component class"]
    #[inline(always)]
    pub fn component_class(&self) -> COMPONENTCLASS_R {
        COMPONENTCLASS_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
