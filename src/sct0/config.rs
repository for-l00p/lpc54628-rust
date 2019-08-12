#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
        0x1e00
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYR {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER,
}
impl crate::ToBits<bool> for UNIFYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            UNIFYR::DUAL_COUNTER => false,
            UNIFYR::UNIFIED_COUNTER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type UNIFY_R = crate::FR<bool, UNIFYR>;
impl UNIFY_R {
    #[doc = "Checks if the value of the field is `DUAL_COUNTER`"]
    #[inline(always)]
    pub fn is_dual_counter(&self) -> bool {
        *self == UNIFYR::DUAL_COUNTER
    }
    #[doc = "Checks if the value of the field is `UNIFIED_COUNTER`"]
    #[inline(always)]
    pub fn is_unified_counter(&self) -> bool {
        *self == UNIFYR::UNIFIED_COUNTER
    }
}
#[doc = "Values that can be written to the field `UNIFY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNIFYW {
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    DUAL_COUNTER,
    #[doc = "The SCT operates as a unified 32-bit counter."]
    UNIFIED_COUNTER,
}
impl UNIFYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            UNIFYW::DUAL_COUNTER => false,
            UNIFYW::UNIFIED_COUNTER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UNIFYW<'a> {
    w: &'a mut W,
}
impl<'a> _UNIFYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNIFYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SCT operates as two 16-bit counters named COUNTER_L and COUNTER_H."]
    #[inline(always)]
    pub fn dual_counter(self) -> &'a mut W {
        self.variant(UNIFYW::DUAL_COUNTER)
    }
    #[doc = "The SCT operates as a unified 32-bit counter."]
    #[inline(always)]
    pub fn unified_counter(self) -> &'a mut W {
        self.variant(UNIFYW::UNIFIED_COUNTER)
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
#[doc = "Possible values of the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODER {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE,
}
impl crate::ToBits<u8> for CLKMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKMODER::SYSTEM_CLOCK_MODE => 0,
            CLKMODER::SAMPLED_SYSTEM_CLOCK_MODE => 1,
            CLKMODER::SCT_INPUT_CLOCK_MODE => 2,
            CLKMODER::ASYNCHRONOUS_MODE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CLKMODE_R = crate::FR<u8, CLKMODER>;
impl CLKMODE_R {
    #[doc = "Checks if the value of the field is `SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_system_clock_mode(&self) -> bool {
        *self == CLKMODER::SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SAMPLED_SYSTEM_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sampled_system_clock_mode(&self) -> bool {
        *self == CLKMODER::SAMPLED_SYSTEM_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `SCT_INPUT_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_sct_input_clock_mode(&self) -> bool {
        *self == CLKMODER::SCT_INPUT_CLOCK_MODE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == CLKMODER::ASYNCHRONOUS_MODE
    }
}
#[doc = "Values that can be written to the field `CLKMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKMODEW {
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    SYSTEM_CLOCK_MODE,
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    SAMPLED_SYSTEM_CLOCK_MODE,
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    SCT_INPUT_CLOCK_MODE,
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    ASYNCHRONOUS_MODE,
}
impl CLKMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKMODEW::SYSTEM_CLOCK_MODE => 0,
            CLKMODEW::SAMPLED_SYSTEM_CLOCK_MODE => 1,
            CLKMODEW::SCT_INPUT_CLOCK_MODE => 2,
            CLKMODEW::ASYNCHRONOUS_MODE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CLKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "System Clock Mode. The system clock clocks the entire SCT module including the counter(s) and counter prescalers."]
    #[inline(always)]
    pub fn system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::SYSTEM_CLOCK_MODE)
    }
    #[doc = "Sampled System Clock Mode. The system clock clocks the SCT module, but the counter and prescalers are only enabled to count when the designated edge is detected on the input selected by the CKSEL field. The minimum pulse width on the selected clock-gate input is 1 bus clock period. This mode is the high-performance, sampled-clock mode."]
    #[inline(always)]
    pub fn sampled_system_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::SAMPLED_SYSTEM_CLOCK_MODE)
    }
    #[doc = "SCT Input Clock Mode. The input/edge selected by the CKSEL field clocks the SCT module, including the counters and prescalers, after first being synchronized to the system clock. The minimum pulse width on the clock input is 1 bus clock period. This mode is the low-power, sampled-clock mode."]
    #[inline(always)]
    pub fn sct_input_clock_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::SCT_INPUT_CLOCK_MODE)
    }
    #[doc = "Asynchronous Mode. The entire SCT module is clocked directly by the input/edge selected by the CKSEL field. In this mode, the SCT outputs are switched synchronously to the SCT input clock - not the system clock. The input clock rate must be at least half the system clock rate and can be the same or faster than the system clock."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(CLKMODEW::ASYNCHRONOUS_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELR {
    #[doc = "Rising edges on input 0."]
    INPUT_0_RISING_EDGES,
    #[doc = "Falling edges on input 0."]
    INPUT_0_FALLING_EDGE,
    #[doc = "Rising edges on input 1."]
    INPUT_1_RISING_EDGES,
    #[doc = "Falling edges on input 1."]
    INPUT_1_FALLING_EDGE,
    #[doc = "Rising edges on input 2."]
    INPUT_2_RISING_EDGES,
    #[doc = "Falling edges on input 2."]
    INPUT_2_FALLING_EDGE,
    #[doc = "Rising edges on input 3."]
    INPUT_3_RISING_EDGES,
    #[doc = "Falling edges on input 3."]
    INPUT_3_FALLING_EDGE,
}
impl crate::ToBits<u8> for CKSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKSELR::INPUT_0_RISING_EDGES => 0,
            CKSELR::INPUT_0_FALLING_EDGE => 1,
            CKSELR::INPUT_1_RISING_EDGES => 2,
            CKSELR::INPUT_1_FALLING_EDGE => 3,
            CKSELR::INPUT_2_RISING_EDGES => 4,
            CKSELR::INPUT_2_FALLING_EDGE => 5,
            CKSELR::INPUT_3_RISING_EDGES => 6,
            CKSELR::INPUT_3_FALLING_EDGE => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CKSEL_R = crate::FR<u8, CKSELR>;
impl CKSEL_R {
    #[doc = "Checks if the value of the field is `INPUT_0_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_0_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_0_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_0_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_0_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_0_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_1_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_1_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_1_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_1_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_1_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_1_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_2_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_2_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_2_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_2_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_2_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_2_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `INPUT_3_RISING_EDGES`"]
    #[inline(always)]
    pub fn is_input_3_rising_edges(&self) -> bool {
        *self == CKSELR::INPUT_3_RISING_EDGES
    }
    #[doc = "Checks if the value of the field is `INPUT_3_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_input_3_falling_edge(&self) -> bool {
        *self == CKSELR::INPUT_3_FALLING_EDGE
    }
}
#[doc = "Values that can be written to the field `CKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKSELW {
    #[doc = "Rising edges on input 0."]
    INPUT_0_RISING_EDGES,
    #[doc = "Falling edges on input 0."]
    INPUT_0_FALLING_EDGE,
    #[doc = "Rising edges on input 1."]
    INPUT_1_RISING_EDGES,
    #[doc = "Falling edges on input 1."]
    INPUT_1_FALLING_EDGE,
    #[doc = "Rising edges on input 2."]
    INPUT_2_RISING_EDGES,
    #[doc = "Falling edges on input 2."]
    INPUT_2_FALLING_EDGE,
    #[doc = "Rising edges on input 3."]
    INPUT_3_RISING_EDGES,
    #[doc = "Falling edges on input 3."]
    INPUT_3_FALLING_EDGE,
}
impl CKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKSELW::INPUT_0_RISING_EDGES => 0,
            CKSELW::INPUT_0_FALLING_EDGE => 1,
            CKSELW::INPUT_1_RISING_EDGES => 2,
            CKSELW::INPUT_1_FALLING_EDGE => 3,
            CKSELW::INPUT_2_RISING_EDGES => 4,
            CKSELW::INPUT_2_FALLING_EDGE => 5,
            CKSELW::INPUT_3_RISING_EDGES => 6,
            CKSELW::INPUT_3_FALLING_EDGE => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CKSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rising edges on input 0."]
    #[inline(always)]
    pub fn input_0_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_0_RISING_EDGES)
    }
    #[doc = "Falling edges on input 0."]
    #[inline(always)]
    pub fn input_0_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_0_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 1."]
    #[inline(always)]
    pub fn input_1_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_1_RISING_EDGES)
    }
    #[doc = "Falling edges on input 1."]
    #[inline(always)]
    pub fn input_1_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_1_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 2."]
    #[inline(always)]
    pub fn input_2_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_2_RISING_EDGES)
    }
    #[doc = "Falling edges on input 2."]
    #[inline(always)]
    pub fn input_2_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_2_FALLING_EDGE)
    }
    #[doc = "Rising edges on input 3."]
    #[inline(always)]
    pub fn input_3_rising_edges(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_3_RISING_EDGES)
    }
    #[doc = "Falling edges on input 3."]
    #[inline(always)]
    pub fn input_3_falling_edge(self) -> &'a mut W {
        self.variant(CKSELW::INPUT_3_FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NORELAOD_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NORELAOD_LW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELAOD_LW<'a> {
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
pub type NORELOAD_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NORELOAD_HW<'a> {
    w: &'a mut W,
}
impl<'a> _NORELOAD_HW<'a> {
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
pub type INSYNC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSYNCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AUTOLIMIT_L_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AUTOLIMIT_LW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_LW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AUTOLIMIT_H_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AUTOLIMIT_HW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOLIMIT_HW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&self) -> UNIFY_R {
        UNIFY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&self) -> CLKMODE_R {
        CLKMODE_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits() >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn norelaod_l(&self) -> NORELAOD_L_R {
        NORELAOD_L_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&self) -> NORELOAD_H_R {
        NORELOAD_H_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&self) -> INSYNC_R {
        INSYNC_R::new(((self.bits() >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&self) -> AUTOLIMIT_L_R {
        AUTOLIMIT_L_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&self) -> AUTOLIMIT_H_R {
        AUTOLIMIT_H_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SCT operation"]
    #[inline(always)]
    pub fn unify(&mut self) -> _UNIFYW {
        _UNIFYW { w: self }
    }
    #[doc = "Bits 1:2 - SCT clock mode"]
    #[inline(always)]
    pub fn clkmode(&mut self) -> _CLKMODEW {
        _CLKMODEW { w: self }
    }
    #[doc = "Bits 3:6 - SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub fn cksel(&mut self) -> _CKSELW {
        _CKSELW { w: self }
    }
    #[doc = "Bit 7 - A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn norelaod_l(&mut self) -> _NORELAOD_LW {
        _NORELAOD_LW { w: self }
    }
    #[doc = "Bit 8 - A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn noreload_h(&mut self) -> _NORELOAD_HW {
        _NORELOAD_HW { w: self }
    }
    #[doc = "Bits 9:12 - Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub fn insync(&mut self) -> _INSYNCW {
        _INSYNCW { w: self }
    }
    #[doc = "Bit 17 - A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_l(&mut self) -> _AUTOLIMIT_LW {
        _AUTOLIMIT_LW { w: self }
    }
    #[doc = "Bit 18 - A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub fn autolimit_h(&mut self) -> _AUTOLIMIT_HW {
        _AUTOLIMIT_HW { w: self }
    }
}
