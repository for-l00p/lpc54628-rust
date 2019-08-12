#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_RXQ_CTRL0 {
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
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type RXQ0EN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RXQ0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXQ0ENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RXQ1EN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RXQ1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXQ1ENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Receive Queue 0 Enable."]
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Receive Queue 1 Enable."]
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Receive Queue 0 Enable."]
    #[inline(always)]
    pub fn rxq0en(&mut self) -> _RXQ0ENW {
        _RXQ0ENW { w: self }
    }
    #[doc = "Bits 2:3 - Receive Queue 1 Enable."]
    #[inline(always)]
    pub fn rxq1en(&mut self) -> _RXQ1ENW {
        _RXQ1ENW { w: self }
    }
}
