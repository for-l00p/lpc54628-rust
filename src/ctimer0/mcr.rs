#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
pub type MR0I_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR0IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0IW<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR0R_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR0RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0RW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR0S_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR0SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0SW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR1I_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR1IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1IW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR1R_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR1RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1RW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR1S_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR1SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1SW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR2I_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR2IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2IW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR2R_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR2RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2RW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR2S_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR2SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2SW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR3I_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR3IW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3IW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR3R_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR3RW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3RW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR3S_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR3SW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3SW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR0RL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR0RLW<'a> {
    w: &'a mut W,
}
impl<'a> _MR0RLW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR1RL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR1RLW<'a> {
    w: &'a mut W,
}
impl<'a> _MR1RLW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR2RL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR2RLW<'a> {
    w: &'a mut W,
}
impl<'a> _MR2RLW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MR3RL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MR3RLW<'a> {
    w: &'a mut W,
}
impl<'a> _MR3RLW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr0rl(&self) -> MR0RL_R {
        MR0RL_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr1rl(&self) -> MR1RL_R {
        MR1RL_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr2rl(&self) -> MR2RL_R {
        MR2RL_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr3rl(&self) -> MR3RL_R {
        MR3RL_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&mut self) -> _MR0IW {
        _MR0IW { w: self }
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&mut self) -> _MR0RW {
        _MR0RW { w: self }
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&mut self) -> _MR0SW {
        _MR0SW { w: self }
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&mut self) -> _MR1IW {
        _MR1IW { w: self }
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&mut self) -> _MR1RW {
        _MR1RW { w: self }
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&mut self) -> _MR1SW {
        _MR1SW { w: self }
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&mut self) -> _MR2IW {
        _MR2IW { w: self }
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&mut self) -> _MR2RW {
        _MR2RW { w: self }
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> _MR2SW {
        _MR2SW { w: self }
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&mut self) -> _MR3IW {
        _MR3IW { w: self }
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&mut self) -> _MR3RW {
        _MR3RW { w: self }
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&mut self) -> _MR3SW {
        _MR3SW { w: self }
    }
    #[doc = "Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr0rl(&mut self) -> _MR0RLW {
        _MR0RLW { w: self }
    }
    #[doc = "Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr1rl(&mut self) -> _MR1RLW {
        _MR1RLW { w: self }
    }
    #[doc = "Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr2rl(&mut self) -> _MR2RLW {
        _MR2RLW { w: self }
    }
    #[doc = "Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr3rl(&mut self) -> _MR3RLW {
        _MR3RLW { w: self }
    }
}
