#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MAC_SYS_TIME_SCND {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TSS_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Time stamp second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits() & 0xffff_ffff) as u32)
    }
}
