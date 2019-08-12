#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWVADGAIN {
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
        0x05
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type INPUTGAIN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INPUTGAINW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTGAINW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
    #[inline(always)]
    pub fn inputgain(&self) -> INPUTGAIN_R {
        INPUTGAIN_R::new((self.bits() & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Shift value for input bits 0x00 -10 bits 0x01 -8 bits 0x02 -6 bits 0x03 -4 bits 0x04 -2 bits 0x05 0 bits (default) 0x06 +2 bits 0x07 +4 bits 0x08 +6 bits 0x09 +8 bits 0x0A +10 bits 0x0B +12 bits 0x0C +14 bits 0x0D to 0x0F Reserved."]
    #[inline(always)]
    pub fn inputgain(&mut self) -> _INPUTGAINW {
        _INPUTGAINW { w: self }
    }
}
