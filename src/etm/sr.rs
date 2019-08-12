#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
pub type UOF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PROGBIT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type STATUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _STATUSW<'a> {
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
pub type TRIGGER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TRIGGERW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Untraced overflow flag. If set to 1, there is an overflow that has not yet been traced. This bit is cleared to 0 when either: - trace is restarted - the ETM Power Down bit, bit \\[0\\] of the ETM Control Register, 0x00, is set to 1. Note: Setting or clearing the ETM programming bit does not cause this bit to be cleared to 0."]
    #[inline(always)]
    pub fn uof(&self) -> UOF_R {
        UOF_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - ETM programming bit value (Progbit). The current effective value of the ETM Programming bit (ETM Control Register bit \\[10\\]). Tou must wait for this bit to go to 1 before you start to program the ETM."]
    #[inline(always)]
    pub fn progbit(&self) -> PROGBIT_R {
        PROGBIT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Holds the current status of the trace start/stop resource. If set to 1, it indicates that a trace on address has been matched, without a corresponding trace off address match."]
    #[inline(always)]
    pub fn status(&mut self) -> _STATUSW {
        _STATUSW { w: self }
    }
    #[doc = "Bit 3 - Trigger bit. Set when the trigger occurs, and prevents the trigger from being output until the ETM is next programmed."]
    #[inline(always)]
    pub fn trigger(&mut self) -> _TRIGGERW {
        _TRIGGERW { w: self }
    }
}
