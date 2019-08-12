#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOD {
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
#[doc = "Possible values of the field `WDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDENR {
    #[doc = "Stop. The watchdog timer is stopped."]
    STOP,
    #[doc = "Run. The watchdog timer is running."]
    RUN,
}
impl crate::ToBits<bool> for WDENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDENR::STOP => false,
            WDENR::RUN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDEN_R = crate::FR<bool, WDENR>;
impl WDEN_R {
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WDENR::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WDENR::RUN
    }
}
#[doc = "Values that can be written to the field `WDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDENW {
    #[doc = "Stop. The watchdog timer is stopped."]
    STOP,
    #[doc = "Run. The watchdog timer is running."]
    RUN,
}
impl WDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDENW::STOP => false,
            WDENW::RUN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDENW<'a> {
    w: &'a mut W,
}
impl<'a> _WDENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop. The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(WDENW::STOP)
    }
    #[doc = "Run. The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(WDENW::RUN)
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
#[doc = "Possible values of the field `WDRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESETR {
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    INTERRUPT,
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    RESET,
}
impl crate::ToBits<bool> for WDRESETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDRESETR::INTERRUPT => false,
            WDRESETR::RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDRESET_R = crate::FR<bool, WDRESETR>;
impl WDRESET_R {
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDRESETR::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDRESETR::RESET
    }
}
#[doc = "Values that can be written to the field `WDRESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRESETW {
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    INTERRUPT,
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    RESET,
}
impl WDRESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDRESETW::INTERRUPT => false,
            WDRESETW::RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRESETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDRESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt. A watchdog time-out will not cause a chip reset."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDRESETW::INTERRUPT)
    }
    #[doc = "Reset. A watchdog time-out will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDRESETW::RESET)
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
#[doc = r"Reader of the field"]
pub type WDTOF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDTOFW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOFW<'a> {
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
pub type WDINT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDINTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDINTW<'a> {
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
#[doc = "Possible values of the field `WDPROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECTR {
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    FLEXIBLE,
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    THRESHOLD,
}
impl crate::ToBits<bool> for WDPROTECTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDPROTECTR::FLEXIBLE => false,
            WDPROTECTR::THRESHOLD => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDPROTECT_R = crate::FR<bool, WDPROTECTR>;
impl WDPROTECT_R {
    #[doc = "Checks if the value of the field is `FLEXIBLE`"]
    #[inline(always)]
    pub fn is_flexible(&self) -> bool {
        *self == WDPROTECTR::FLEXIBLE
    }
    #[doc = "Checks if the value of the field is `THRESHOLD`"]
    #[inline(always)]
    pub fn is_threshold(&self) -> bool {
        *self == WDPROTECTR::THRESHOLD
    }
}
#[doc = "Values that can be written to the field `WDPROTECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDPROTECTW {
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    FLEXIBLE,
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    THRESHOLD,
}
impl WDPROTECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDPROTECTW::FLEXIBLE => false,
            WDPROTECTW::THRESHOLD => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDPROTECTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDPROTECTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDPROTECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flexible. The watchdog time-out value (TC) can be changed at any time."]
    #[inline(always)]
    pub fn flexible(self) -> &'a mut W {
        self.variant(WDPROTECTW::FLEXIBLE)
    }
    #[doc = "Threshold. The watchdog time-out value (TC) can be changed only after the counter is below the value of WDWARNINT and WDWINDOW."]
    #[inline(always)]
    pub fn threshold(self) -> &'a mut W {
        self.variant(WDPROTECTW::THRESHOLD)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&self) -> WDPROTECT_R {
        WDPROTECT_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Watchdog enable bit. Once this bit is set to one and a watchdog feed is performed, the watchdog timer will run permanently."]
    #[inline(always)]
    pub fn wden(&mut self) -> _WDENW {
        _WDENW { w: self }
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. Once this bit has been written with a 1 it cannot be re-written with a 0."]
    #[inline(always)]
    pub fn wdreset(&mut self) -> _WDRESETW {
        _WDRESETW { w: self }
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, by a feed error, or by events associated with WDPROTECT. Cleared by software writing a 0 to this bit position. Causes a chip reset if WDRESET = 1."]
    #[inline(always)]
    pub fn wdtof(&mut self) -> _WDTOFW {
        _WDTOFW { w: self }
    }
    #[doc = "Bit 3 - Warning interrupt flag. Set when the timer is at or below the value in WDWARNINT. Cleared by software writing a 1 to this bit position. Note that this bit cannot be cleared while the WARNINT value is equal to the value of the TV register. This can occur if the value of WARNINT is 0 and the WDRESET bit is 0 when TV decrements to 0."]
    #[inline(always)]
    pub fn wdint(&mut self) -> _WDINTW {
        _WDINTW { w: self }
    }
    #[doc = "Bit 4 - Watchdog update mode. This bit can be set once by software and is only cleared by a reset."]
    #[inline(always)]
    pub fn wdprotect(&mut self) -> _WDPROTECTW {
        _WDPROTECTW { w: self }
    }
    #[doc = "Bit 5 - Once this bit is set to one and a watchdog feed is performed, disabling or powering down the watchdog oscillator is prevented by hardware. This bit can be set once by software and is only cleared by any reset."]
    #[inline(always)]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
