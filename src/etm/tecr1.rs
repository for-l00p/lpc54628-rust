#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TECR1 {
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
#[doc = "Possible values of the field `TraceControlEnable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECONTROLENABLER {
    #[doc = "Tracing is unaffected by the trace start/stop logic."]
    TRACECONTROLENABLE_0,
    #[doc = "Tracing is controlled by the trace on and off addresses configured for the trace start/stop logic."]
    TRACECONTROLENABLE_1,
}
impl crate::ToBits<bool> for TRACECONTROLENABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRACECONTROLENABLER::TRACECONTROLENABLE_0 => false,
            TRACECONTROLENABLER::TRACECONTROLENABLE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRACECONTROLENABLE_R = crate::FR<bool, TRACECONTROLENABLER>;
impl TRACECONTROLENABLE_R {
    #[doc = "Checks if the value of the field is `TRACECONTROLENABLE_0`"]
    #[inline(always)]
    pub fn is_trace_control_enable_0(&self) -> bool {
        *self == TRACECONTROLENABLER::TRACECONTROLENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRACECONTROLENABLE_1`"]
    #[inline(always)]
    pub fn is_trace_control_enable_1(&self) -> bool {
        *self == TRACECONTROLENABLER::TRACECONTROLENABLE_1
    }
}
#[doc = "Values that can be written to the field `TraceControlEnable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECONTROLENABLEW {
    #[doc = "Tracing is unaffected by the trace start/stop logic."]
    TRACECONTROLENABLE_0,
    #[doc = "Tracing is controlled by the trace on and off addresses configured for the trace start/stop logic."]
    TRACECONTROLENABLE_1,
}
impl TRACECONTROLENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRACECONTROLENABLEW::TRACECONTROLENABLE_0 => false,
            TRACECONTROLENABLEW::TRACECONTROLENABLE_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRACECONTROLENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACECONTROLENABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECONTROLENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tracing is unaffected by the trace start/stop logic."]
    #[inline(always)]
    pub fn trace_control_enable_0(self) -> &'a mut W {
        self.variant(TRACECONTROLENABLEW::TRACECONTROLENABLE_0)
    }
    #[doc = "Tracing is controlled by the trace on and off addresses configured for the trace start/stop logic."]
    #[inline(always)]
    pub fn trace_control_enable_1(self) -> &'a mut W {
        self.variant(TRACECONTROLENABLEW::TRACECONTROLENABLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 25 - Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit."]
    #[inline(always)]
    pub fn trace_control_enable(&self) -> TRACECONTROLENABLE_R {
        TRACECONTROLENABLE_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 25 - Trace start/stop enable. The trace start/stop resource, resource 0x5F, is unaffected by the value of this bit."]
    #[inline(always)]
    pub fn trace_control_enable(&mut self) -> _TRACECONTROLENABLEW {
        _TRACECONTROLENABLEW { w: self }
    }
}
