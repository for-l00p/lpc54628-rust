#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPUID {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type REVISION_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PARTNO_R = crate::FR<u16, u16>;
#[doc = r"Reader of the field"]
pub type VARIANT_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type IMPLEMENTER_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Indicates patch release: 0x0 = Patch 0"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Indicates part number"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits() >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - Indicates processor revision: 0x2 = Revision 2"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits() >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
