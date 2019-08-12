#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBCLKCTRLCLR {
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
#[doc = r"Proxy"]
pub struct _CLK_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_CLRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn clk_clr(&mut self) -> _CLK_CLRW {
        _CLK_CLRW { w: self }
    }
}
