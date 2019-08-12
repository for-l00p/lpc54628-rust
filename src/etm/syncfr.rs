#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYNCFR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type SYNCFREQUENCY_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - Synchronization frequency. Default value is 1024."]
    #[inline(always)]
    pub fn sync_frequency(&self) -> SYNCFREQUENCY_R {
        SYNCFREQUENCY_R::new((self.bits() & 0x0fff) as u16)
    }
}
