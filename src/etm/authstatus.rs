#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AUTHSTATUS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type NSID_R = crate::FR<u8, u8>;
#[doc = "Possible values of the field `NSNID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSNIDR {
    #[doc = "Non-secure non-invasive debug disabled"]
    NSNID_2,
    #[doc = "Non-secure non-invasive debug enabled"]
    NSNID_3,
}
impl crate::ToBits<u8> for NSNIDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NSNIDR::NSNID_2 => 2,
            NSNIDR::NSNID_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NSNID_R = crate::FR<u8, NSNIDR>;
impl NSNID_R {
    #[doc = "Checks if the value of the field is `NSNID_2`"]
    #[inline(always)]
    pub fn is_nsnid_2(&self) -> bool {
        *self == NSNIDR::NSNID_2
    }
    #[doc = "Checks if the value of the field is `NSNID_3`"]
    #[inline(always)]
    pub fn is_nsnid_3(&self) -> bool {
        *self == NSNIDR::NSNID_3
    }
}
#[doc = r"Reader of the field"]
pub type SID_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type SNID_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Reads as b00, Non-secure invasive debug not supported by the ETM."]
    #[inline(always)]
    pub fn nsid(&self) -> NSID_R {
        NSID_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Permission for Non-secure non-invasive debug."]
    #[inline(always)]
    pub fn nsnid(&self) -> NSNID_R {
        NSNID_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Reads as b00, Secure invasive debug not supported by the ETM."]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Permission for Secure non-invasive debug."]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
}
