#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_ITRIG_INMUX {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
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
        0x1f
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type INP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INPW<'a> {
    w: &'a mut W,
}
impl<'a> _INPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 21). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4 = Timer CTIMER0 Match 0 5 = Timer CTIMER0 Match 1 6 = Timer CTIMER1 Match 0 7 = Timer CTIMER2 Match 0 8 = Timer CTIMER2 Match 1 9 = Timer CTIMER3 Match 0 10 = Timer CTIMER4 Match 0 11 = Timer CTIMER4 Match 1 12 = Pin interrupt 0 13 = Pin interrupt 1 14 = Pin interrupt 2 15 = Pin interrupt 3 16 = DMA output trigger mux 0 17 = DMA output trigger mux 1 18 = DMA output trigger mux 2 19 = DMA output trigger mux 3"]
    #[inline(always)]
    pub fn inp(&self) -> INP_R {
        INP_R::new((self.bits() & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Trigger input number (decimal value) for DMA channel n (n = 0 to 21). 0 = ADC0 Sequence A interrupt 1 = ADC0 Sequence B interrupt 2 = SCT0 DMA request 0 3 = SCT0 DMA request 1 4 = Timer CTIMER0 Match 0 5 = Timer CTIMER0 Match 1 6 = Timer CTIMER1 Match 0 7 = Timer CTIMER2 Match 0 8 = Timer CTIMER2 Match 1 9 = Timer CTIMER3 Match 0 10 = Timer CTIMER4 Match 0 11 = Timer CTIMER4 Match 1 12 = Pin interrupt 0 13 = Pin interrupt 1 14 = Pin interrupt 2 15 = Pin interrupt 3 16 = DMA output trigger mux 0 17 = DMA output trigger mux 1 18 = DMA output trigger mux 2 19 = DMA output trigger mux 3"]
    #[inline(always)]
    pub fn inp(&mut self) -> _INPW {
        _INPW { w: self }
    }
}
