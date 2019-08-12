#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCFG {
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
        0x0d1a
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `FETCHCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FETCHCFGR {
    #[doc = "Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    NO_BUFFER,
    #[doc = "One buffer is used for all instruction fetches."]
    ONE_BUFFER,
    #[doc = "All buffers may be used for instruction fetches."]
    ALL_BUFFERS,
}
impl crate::ToBits<u8> for FETCHCFGR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FETCHCFGR::NO_BUFFER => 0,
            FETCHCFGR::ONE_BUFFER => 1,
            FETCHCFGR::ALL_BUFFERS => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FETCHCFG_R = crate::FR<u8, FETCHCFGR>;
impl FETCHCFG_R {
    #[doc = "Checks if the value of the field is `NO_BUFFER`"]
    #[inline(always)]
    pub fn is_no_buffer(&self) -> bool {
        *self == FETCHCFGR::NO_BUFFER
    }
    #[doc = "Checks if the value of the field is `ONE_BUFFER`"]
    #[inline(always)]
    pub fn is_one_buffer(&self) -> bool {
        *self == FETCHCFGR::ONE_BUFFER
    }
    #[doc = "Checks if the value of the field is `ALL_BUFFERS`"]
    #[inline(always)]
    pub fn is_all_buffers(&self) -> bool {
        *self == FETCHCFGR::ALL_BUFFERS
    }
}
#[doc = "Values that can be written to the field `FETCHCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FETCHCFGW {
    #[doc = "Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    NO_BUFFER,
    #[doc = "One buffer is used for all instruction fetches."]
    ONE_BUFFER,
    #[doc = "All buffers may be used for instruction fetches."]
    ALL_BUFFERS,
}
impl FETCHCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FETCHCFGW::NO_BUFFER => 0,
            FETCHCFGW::ONE_BUFFER => 1,
            FETCHCFGW::ALL_BUFFERS => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FETCHCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _FETCHCFGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FETCHCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Instruction fetches from flash are not buffered. Every fetch request from the CPU results in a read of the flash memory. This setting may use significantly more power than when buffering is enabled."]
    #[inline(always)]
    pub fn no_buffer(self) -> &'a mut W {
        self.variant(FETCHCFGW::NO_BUFFER)
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn one_buffer(self) -> &'a mut W {
        self.variant(FETCHCFGW::ONE_BUFFER)
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn all_buffers(self) -> &'a mut W {
        self.variant(FETCHCFGW::ALL_BUFFERS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `DATACFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATACFGR {
    #[doc = "Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    NOT_BUFFERED,
    #[doc = "One buffer is used for all data accesses."]
    ONE_BUFFER,
    #[doc = "All buffers may be used for data accesses."]
    ALL_BUFFERS,
}
impl crate::ToBits<u8> for DATACFGR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DATACFGR::NOT_BUFFERED => 0,
            DATACFGR::ONE_BUFFER => 1,
            DATACFGR::ALL_BUFFERS => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DATACFG_R = crate::FR<u8, DATACFGR>;
impl DATACFG_R {
    #[doc = "Checks if the value of the field is `NOT_BUFFERED`"]
    #[inline(always)]
    pub fn is_not_buffered(&self) -> bool {
        *self == DATACFGR::NOT_BUFFERED
    }
    #[doc = "Checks if the value of the field is `ONE_BUFFER`"]
    #[inline(always)]
    pub fn is_one_buffer(&self) -> bool {
        *self == DATACFGR::ONE_BUFFER
    }
    #[doc = "Checks if the value of the field is `ALL_BUFFERS`"]
    #[inline(always)]
    pub fn is_all_buffers(&self) -> bool {
        *self == DATACFGR::ALL_BUFFERS
    }
}
#[doc = "Values that can be written to the field `DATACFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATACFGW {
    #[doc = "Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    NOT_BUFFERED,
    #[doc = "One buffer is used for all data accesses."]
    ONE_BUFFER,
    #[doc = "All buffers may be used for data accesses."]
    ALL_BUFFERS,
}
impl DATACFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATACFGW::NOT_BUFFERED => 0,
            DATACFGW::ONE_BUFFER => 1,
            DATACFGW::ALL_BUFFERS => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DATACFGW<'a> {
    w: &'a mut W,
}
impl<'a> _DATACFGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATACFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Data accesses from flash are not buffered. Every data access from the CPU results in a read of the flash memory."]
    #[inline(always)]
    pub fn not_buffered(self) -> &'a mut W {
        self.variant(DATACFGW::NOT_BUFFERED)
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn one_buffer(self) -> &'a mut W {
        self.variant(DATACFGW::ONE_BUFFER)
    }
    #[doc = "All buffers may be used for data accesses."]
    #[inline(always)]
    pub fn all_buffers(self) -> &'a mut W {
        self.variant(DATACFGW::ALL_BUFFERS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `ACCEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCELR {
    #[doc = "Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    DISABLED,
    #[doc = "Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    ENABLED,
}
impl crate::ToBits<bool> for ACCELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACCELR::DISABLED => false,
            ACCELR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ACCEL_R = crate::FR<bool, ACCELR>;
impl ACCEL_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACCELR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACCELR::ENABLED
    }
}
#[doc = "Values that can be written to the field `ACCEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCELW {
    #[doc = "Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    DISABLED,
    #[doc = "Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    ENABLED,
}
impl ACCELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ACCELW::DISABLED => false,
            ACCELW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ACCELW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Flash acceleration is disabled. Every flash read (including those fulfilled from a buffer) takes FLASHTIM + 1 system clocks. This allows more determinism at a cost of performance."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACCELW::DISABLED)
    }
    #[doc = "Flash acceleration is enabled. Performance is enhanced, dependent on other FLASHCFG settings."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACCELW::ENABLED)
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
#[doc = "Possible values of the field `PREFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFENR {
    #[doc = "No instruction prefetch is performed."]
    NO_PREFETCH,
    #[doc = "If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    PREFETCH,
}
impl crate::ToBits<bool> for PREFENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PREFENR::NO_PREFETCH => false,
            PREFENR::PREFETCH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PREFEN_R = crate::FR<bool, PREFENR>;
impl PREFEN_R {
    #[doc = "Checks if the value of the field is `NO_PREFETCH`"]
    #[inline(always)]
    pub fn is_no_prefetch(&self) -> bool {
        *self == PREFENR::NO_PREFETCH
    }
    #[doc = "Checks if the value of the field is `PREFETCH`"]
    #[inline(always)]
    pub fn is_prefetch(&self) -> bool {
        *self == PREFENR::PREFETCH
    }
}
#[doc = "Values that can be written to the field `PREFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFENW {
    #[doc = "No instruction prefetch is performed."]
    NO_PREFETCH,
    #[doc = "If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    PREFETCH,
}
impl PREFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PREFENW::NO_PREFETCH => false,
            PREFENW::PREFETCH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PREFENW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn no_prefetch(self) -> &'a mut W {
        self.variant(PREFENW::NO_PREFETCH)
    }
    #[doc = "If the FETCHCFG field is not 0, the next flash line following the current execution address is automatically prefetched if it is not already buffered."]
    #[inline(always)]
    pub fn prefetch(self) -> &'a mut W {
        self.variant(PREFENW::PREFETCH)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `PREFOVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFOVRR {
    #[doc = "Any previously initiated prefetch will be completed."]
    PREFETCH_COMPLETED,
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    PREFETCH_ABORT,
}
impl crate::ToBits<bool> for PREFOVRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PREFOVRR::PREFETCH_COMPLETED => false,
            PREFOVRR::PREFETCH_ABORT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PREFOVR_R = crate::FR<bool, PREFOVRR>;
impl PREFOVR_R {
    #[doc = "Checks if the value of the field is `PREFETCH_COMPLETED`"]
    #[inline(always)]
    pub fn is_prefetch_completed(&self) -> bool {
        *self == PREFOVRR::PREFETCH_COMPLETED
    }
    #[doc = "Checks if the value of the field is `PREFETCH_ABORT`"]
    #[inline(always)]
    pub fn is_prefetch_abort(&self) -> bool {
        *self == PREFOVRR::PREFETCH_ABORT
    }
}
#[doc = "Values that can be written to the field `PREFOVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREFOVRW {
    #[doc = "Any previously initiated prefetch will be completed."]
    PREFETCH_COMPLETED,
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    PREFETCH_ABORT,
}
impl PREFOVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PREFOVRW::PREFETCH_COMPLETED => false,
            PREFOVRW::PREFETCH_ABORT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PREFOVRW<'a> {
    w: &'a mut W,
}
impl<'a> _PREFOVRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREFOVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn prefetch_completed(self) -> &'a mut W {
        self.variant(PREFOVRW::PREFETCH_COMPLETED)
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn prefetch_abort(self) -> &'a mut W {
        self.variant(PREFOVRW::PREFETCH_ABORT)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMR {
    #[doc = "1 system clock flash access time (for system clock rates up to 12 MHz)."]
    N_1_CLOCK_CYCLE,
    #[doc = "2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    N_2_CLOCK_CYCLES,
    #[doc = "3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    N_3_CLOCK_CYCLES,
    #[doc = "4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    N_4_CLOCK_CYCLES,
    #[doc = "5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    N_5_CLOCK_CYCLES,
}
impl crate::ToBits<u8> for FLASHTIMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMR::N_1_CLOCK_CYCLE => 0,
            FLASHTIMR::N_2_CLOCK_CYCLES => 1,
            FLASHTIMR::N_3_CLOCK_CYCLES => 2,
            FLASHTIMR::N_4_CLOCK_CYCLES => 3,
            FLASHTIMR::N_5_CLOCK_CYCLES => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FLASHTIM_R = crate::FR<u8, FLASHTIMR>;
impl FLASHTIM_R {
    #[doc = "Checks if the value of the field is `N_1_CLOCK_CYCLE`"]
    #[inline(always)]
    pub fn is_n_1_clock_cycle(&self) -> bool {
        *self == FLASHTIMR::N_1_CLOCK_CYCLE
    }
    #[doc = "Checks if the value of the field is `N_2_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_2_clock_cycles(&self) -> bool {
        *self == FLASHTIMR::N_2_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_3_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_3_clock_cycles(&self) -> bool {
        *self == FLASHTIMR::N_3_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_4_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_4_clock_cycles(&self) -> bool {
        *self == FLASHTIMR::N_4_CLOCK_CYCLES
    }
    #[doc = "Checks if the value of the field is `N_5_CLOCK_CYCLES`"]
    #[inline(always)]
    pub fn is_n_5_clock_cycles(&self) -> bool {
        *self == FLASHTIMR::N_5_CLOCK_CYCLES
    }
}
#[doc = "Values that can be written to the field `FLASHTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHTIMW {
    #[doc = "1 system clock flash access time (for system clock rates up to 12 MHz)."]
    N_1_CLOCK_CYCLE,
    #[doc = "2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    N_2_CLOCK_CYCLES,
    #[doc = "3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    N_3_CLOCK_CYCLES,
    #[doc = "4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    N_4_CLOCK_CYCLES,
    #[doc = "5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    N_5_CLOCK_CYCLES,
}
impl FLASHTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHTIMW::N_1_CLOCK_CYCLE => 0,
            FLASHTIMW::N_2_CLOCK_CYCLES => 1,
            FLASHTIMW::N_3_CLOCK_CYCLES => 2,
            FLASHTIMW::N_4_CLOCK_CYCLES => 3,
            FLASHTIMW::N_5_CLOCK_CYCLES => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASHTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHTIMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 system clock flash access time (for system clock rates up to 12 MHz)."]
    #[inline(always)]
    pub fn n_1_clock_cycle(self) -> &'a mut W {
        self.variant(FLASHTIMW::N_1_CLOCK_CYCLE)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 30 MHz)."]
    #[inline(always)]
    pub fn n_2_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIMW::N_2_CLOCK_CYCLES)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 60 MHz)."]
    #[inline(always)]
    pub fn n_3_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIMW::N_3_CLOCK_CYCLES)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 85 MHz)."]
    #[inline(always)]
    pub fn n_4_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIMW::N_4_CLOCK_CYCLES)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 100 MHz)."]
    #[inline(always)]
    pub fn n_5_clock_cycles(self) -> &'a mut W {
        self.variant(FLASHTIMW::N_5_CLOCK_CYCLES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
    #[inline(always)]
    pub fn fetchcfg(&self) -> FETCHCFG_R {
        FETCHCFG_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
    #[inline(always)]
    pub fn datacfg(&self) -> DATACFG_R {
        DATACFG_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&self) -> ACCEL_R {
        ACCEL_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
    #[inline(always)]
    pub fn prefovr(&self) -> PREFOVR_R {
        PREFOVR_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Instruction fetch configuration. This field determines how flash accelerator buffers are used for instruction fetches."]
    #[inline(always)]
    pub fn fetchcfg(&mut self) -> _FETCHCFGW {
        _FETCHCFGW { w: self }
    }
    #[doc = "Bits 2:3 - Data read configuration. This field determines how flash accelerator buffers are used for data accesses."]
    #[inline(always)]
    pub fn datacfg(&mut self) -> _DATACFGW {
        _DATACFGW { w: self }
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&mut self) -> _ACCELW {
        _ACCELW { w: self }
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&mut self) -> _PREFENW {
        _PREFENW { w: self }
    }
    #[doc = "Bit 6 - Prefetch override. This bit only applies when PREFEN = 1 and a buffered instruction is completing for which the next flash line is not already buffered or being prefetched."]
    #[inline(always)]
    pub fn prefovr(&mut self) -> _PREFOVRW {
        _PREFOVRW { w: self }
    }
    #[doc = "Bits 12:15 - Flash memory access time. The number of system clocks used for flash accesses is equal to FLASHTIM +1."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> _FLASHTIMW {
        _FLASHTIMW { w: self }
    }
}
