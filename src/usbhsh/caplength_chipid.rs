#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAPLENGTH_CHIPID {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAPLENGTH_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type CHIPID_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Capability Length: This is used as an offset."]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Chip identification: indicates major and minor revision of the IP: \\[31:24\\] = Major revision \\[23:16\\] = Minor revision Major revisions used: 0x01: USB2."]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
