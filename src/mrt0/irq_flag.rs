#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQ_FLAG {
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
#[doc = "Possible values of the field `GFLAG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG0R {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT,
}
impl crate::ToBits<bool> for GFLAG0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            GFLAG0R::NO_PENDING_INTERRUPT => false,
            GFLAG0R::PENDING_INTERRUPT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type GFLAG0_R = crate::FR<bool, GFLAG0R>;
impl GFLAG0_R {
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == GFLAG0R::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == GFLAG0R::PENDING_INTERRUPT
    }
}
#[doc = "Values that can be written to the field `GFLAG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GFLAG0W {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT,
}
impl GFLAG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            GFLAG0W::NO_PENDING_INTERRUPT => false,
            GFLAG0W::PENDING_INTERRUPT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _GFLAG0W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GFLAG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0W::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMER0 has reached the end of the time interval. If the INTEN bit in the CONTROL0 register is also set to 1, the interrupt for timer channel 0 and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(GFLAG0W::PENDING_INTERRUPT)
    }
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
pub type GFLAG1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GFLAG1W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG1W<'a> {
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
pub type GFLAG2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GFLAG2W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG2W<'a> {
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
pub type GFLAG3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GFLAG3W<'a> {
    w: &'a mut W,
}
impl<'a> _GFLAG3W<'a> {
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
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&self) -> GFLAG0_R {
        GFLAG0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&self) -> GFLAG1_R {
        GFLAG1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&self) -> GFLAG2_R {
        GFLAG2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&self) -> GFLAG3_R {
        GFLAG3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn gflag0(&mut self) -> _GFLAG0W {
        _GFLAG0W { w: self }
    }
    #[doc = "Bit 1 - Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn gflag1(&mut self) -> _GFLAG1W {
        _GFLAG1W { w: self }
    }
    #[doc = "Bit 2 - Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn gflag2(&mut self) -> _GFLAG2W {
        _GFLAG2W { w: self }
    }
    #[doc = "Bit 3 - Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn gflag3(&mut self) -> _GFLAG3W {
        _GFLAG3W { w: self }
    }
}
