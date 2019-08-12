#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `INTFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAGR {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT,
}
impl crate::ToBits<bool> for INTFLAGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTFLAGR::NO_PENDING_INTERRUPT => false,
            INTFLAGR::PENDING_INTERRUPT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTFLAG_R = crate::FR<bool, INTFLAGR>;
impl INTFLAG_R {
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == INTFLAGR::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == INTFLAGR::PENDING_INTERRUPT
    }
}
#[doc = "Values that can be written to the field `INTFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAGW {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT,
}
impl INTFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTFLAGW::NO_PENDING_INTERRUPT => false,
            INTFLAGW::PENDING_INTERRUPT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _INTFLAGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAGW::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAGW::PENDING_INTERRUPT)
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
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "Idle state. TIMERn is stopped."]
    IDLE_STATE,
    #[doc = "Running. TIMERn is running."]
    RUNNING,
}
impl crate::ToBits<bool> for RUNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RUNR::IDLE_STATE => false,
            RUNR::RUNNING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RUN_R = crate::FR<bool, RUNR>;
impl RUN_R {
    #[doc = "Checks if the value of the field is `IDLE_STATE`"]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == RUNR::IDLE_STATE
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUNR::RUNNING
    }
}
#[doc = "Values that can be written to the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNW {
    #[doc = "Idle state. TIMERn is stopped."]
    IDLE_STATE,
    #[doc = "Running. TIMERn is running."]
    RUNNING,
}
impl RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RUNW::IDLE_STATE => false,
            RUNW::RUNNING => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle state. TIMERn is stopped."]
    #[inline(always)]
    pub fn idle_state(self) -> &'a mut W {
        self.variant(RUNW::IDLE_STATE)
    }
    #[doc = "Running. TIMERn is running."]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(RUNW::RUNNING)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `INUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INUSER {
    #[doc = "This channel is not in use."]
    NO,
    #[doc = "This channel is in use."]
    YES,
}
impl crate::ToBits<bool> for INUSER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INUSER::NO => false,
            INUSER::YES => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INUSE_R = crate::FR<bool, INUSER>;
impl INUSE_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == INUSER::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == INUSER::YES
    }
}
#[doc = "Values that can be written to the field `INUSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INUSEW {
    #[doc = "This channel is not in use."]
    NO,
    #[doc = "This channel is in use."]
    YES,
}
impl INUSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INUSEW::NO => false,
            INUSEW::YES => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INUSEW<'a> {
    w: &'a mut W,
}
impl<'a> _INUSEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INUSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This channel is not in use."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(INUSEW::NO)
    }
    #[doc = "This channel is in use."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(INUSEW::YES)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&self) -> INTFLAG_R {
        INTFLAG_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub fn inuse(&self) -> INUSE_R {
        INUSE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&mut self) -> _INTFLAGW {
        _INTFLAGW { w: self }
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
    #[doc = "Bit 2 - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub fn inuse(&mut self) -> _INUSEW {
        _INUSEW { w: self }
    }
}
