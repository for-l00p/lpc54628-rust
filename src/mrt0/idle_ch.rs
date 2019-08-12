#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IDLE_CH {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CHAN_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:7 - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub fn chan(&self) -> CHAN_R {
        CHAN_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
}
