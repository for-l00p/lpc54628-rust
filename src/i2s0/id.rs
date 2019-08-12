#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ID {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type APERTURE_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MINOR_REV_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MAJOR_REV_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type ID_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub fn aperture(&self) -> APERTURE_R {
        APERTURE_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Minor revision of module implementation, starting at 0."]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Major revision of module implementation, starting at 0."]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Unique module identifier for this IP block."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
