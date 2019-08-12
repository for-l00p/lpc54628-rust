#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WSTATE {
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
        0x0004_0802
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type PHASE3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHASE3W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHASE2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHASE2W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PHASE1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PHASE1W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LCK_PARWEP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LCK_PARWEPW<'a> {
    w: &'a mut W,
}
impl<'a> _LCK_PARWEPW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&self) -> LCK_PARWEP_R {
        LCK_PARWEP_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&mut self) -> _PHASE3W {
        _PHASE3W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&mut self) -> _PHASE2W {
        _PHASE2W { w: self }
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&mut self) -> _PHASE1W {
        _PHASE1W { w: self }
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&mut self) -> _LCK_PARWEPW {
        _LCK_PARWEPW { w: self }
    }
}
