#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
        0x400f_ffff
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type TIMEOUT_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CSHIGH_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CSHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _CSHIGHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type D_PRFTCH_DIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _D_PRFTCH_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _D_PRFTCH_DISW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type INTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `MODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3R {
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    SCK_LOW,
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    SCK_HIGH,
}
impl crate::ToBits<bool> for MODE3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MODE3R::SCK_LOW => false,
            MODE3R::SCK_HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MODE3_R = crate::FR<bool, MODE3R>;
impl MODE3_R {
    #[doc = "Checks if the value of the field is `SCK_LOW`"]
    #[inline(always)]
    pub fn is_sck_low(&self) -> bool {
        *self == MODE3R::SCK_LOW
    }
    #[doc = "Checks if the value of the field is `SCK_HIGH`"]
    #[inline(always)]
    pub fn is_sck_high(&self) -> bool {
        *self == MODE3R::SCK_HIGH
    }
}
#[doc = "Values that can be written to the field `MODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3W {
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    SCK_LOW,
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    SCK_HIGH,
}
impl MODE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MODE3W::SCK_LOW => false,
            MODE3W::SCK_HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCK LOW. The SPIFI drives SCK low after the rising edge at which the last bit of each command is captured, and keeps it low while CS is HIGH."]
    #[inline(always)]
    pub fn sck_low(self) -> &'a mut W {
        self.variant(MODE3W::SCK_LOW)
    }
    #[doc = "SCK HIGH. the SPIFI keeps SCK high after the rising edge for the last bit of each command and while CS is HIGH, and drives it low after it drives CS LOW. (Known serial flash devices can handle either mode, but some devices may require a particular mode for proper operation.) MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    #[inline(always)]
    pub fn sck_high(self) -> &'a mut W {
        self.variant(MODE3W::SCK_HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `PRFTCH_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTCH_DISR {
    #[doc = "Enable. Cache prefetching enabled."]
    ENABLE,
    #[doc = "Disable. Disables prefetching of cache lines."]
    DISABLE,
}
impl crate::ToBits<bool> for PRFTCH_DISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PRFTCH_DISR::ENABLE => false,
            PRFTCH_DISR::DISABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PRFTCH_DIS_R = crate::FR<bool, PRFTCH_DISR>;
impl PRFTCH_DIS_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PRFTCH_DISR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRFTCH_DISR::DISABLE
    }
}
#[doc = "Values that can be written to the field `PRFTCH_DIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTCH_DISW {
    #[doc = "Enable. Cache prefetching enabled."]
    ENABLE,
    #[doc = "Disable. Disables prefetching of cache lines."]
    DISABLE,
}
impl PRFTCH_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PRFTCH_DISW::ENABLE => false,
            PRFTCH_DISW::DISABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PRFTCH_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _PRFTCH_DISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRFTCH_DISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable. Cache prefetching enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PRFTCH_DISW::ENABLE)
    }
    #[doc = "Disable. Disables prefetching of cache lines."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PRFTCH_DISW::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `DUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALR {
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    QUAD,
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    DUAL,
}
impl crate::ToBits<bool> for DUALR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DUALR::QUAD => false,
            DUALR::DUAL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DUAL_R = crate::FR<bool, DUALR>;
impl DUAL_R {
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DUALR::QUAD
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DUALR::DUAL
    }
}
#[doc = "Values that can be written to the field `DUAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALW {
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    QUAD,
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    DUAL,
}
impl DUALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DUALW::QUAD => false,
            DUALW::DUAL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DUALW<'a> {
    w: &'a mut W,
}
impl<'a> _DUALW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quad protocol. This protocol uses IO3:0."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(DUALW::QUAD)
    }
    #[doc = "Dual protocol. This protocol uses IO1:0."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(DUALW::DUAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `RFCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCLKR {
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    RISING_EDGE,
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    FALLING_EDGE,
}
impl crate::ToBits<bool> for RFCLKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RFCLKR::RISING_EDGE => false,
            RFCLKR::FALLING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RFCLK_R = crate::FR<bool, RFCLKR>;
impl RFCLK_R {
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == RFCLKR::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == RFCLKR::FALLING_EDGE
    }
}
#[doc = "Values that can be written to the field `RFCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFCLKW {
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    RISING_EDGE,
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    FALLING_EDGE,
}
impl RFCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RFCLKW::RISING_EDGE => false,
            RFCLKW::FALLING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RFCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _RFCLKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rising edge. Read data is sampled on rising edges on the clock, as in classic SPI operation."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(RFCLKW::RISING_EDGE)
    }
    #[doc = "Falling edge. Read data is sampled on falling edges of the clock, allowing a full serial clock of of time in order to maximize the serial clock frequency. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(RFCLKW::FALLING_EDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Possible values of the field `FBCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLKR {
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    INTERNAL_CLOCK,
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    FEEDBACK_CLOCK,
}
impl crate::ToBits<bool> for FBCLKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FBCLKR::INTERNAL_CLOCK => false,
            FBCLKR::FEEDBACK_CLOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FBCLK_R = crate::FR<bool, FBCLKR>;
impl FBCLK_R {
    #[doc = "Checks if the value of the field is `INTERNAL_CLOCK`"]
    #[inline(always)]
    pub fn is_internal_clock(&self) -> bool {
        *self == FBCLKR::INTERNAL_CLOCK
    }
    #[doc = "Checks if the value of the field is `FEEDBACK_CLOCK`"]
    #[inline(always)]
    pub fn is_feedback_clock(&self) -> bool {
        *self == FBCLKR::FEEDBACK_CLOCK
    }
}
#[doc = "Values that can be written to the field `FBCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCLKW {
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    INTERNAL_CLOCK,
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    FEEDBACK_CLOCK,
}
impl FBCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FBCLKW::INTERNAL_CLOCK => false,
            FBCLKW::FEEDBACK_CLOCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FBCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCLKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal clock. The SPIFI samples read data using an internal clock."]
    #[inline(always)]
    pub fn internal_clock(self) -> &'a mut W {
        self.variant(FBCLKW::INTERNAL_CLOCK)
    }
    #[doc = "Feedback clock. Read data is sampled using a feedback clock from the SCK pin. This allows slightly more time for each received bit. MODE3, RFCLK, and FBCLK should not all be 1, because in this case there is no final falling edge on SCK on which to sample the last data bit of the frame."]
    #[inline(always)]
    pub fn feedback_clock(self) -> &'a mut W {
        self.variant(FBCLKW::FEEDBACK_CLOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DMAEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
    #[doc = "Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits() & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
    #[inline(always)]
    pub fn cshigh(&self) -> CSHIGH_R {
        CSHIGH_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
    #[inline(always)]
    pub fn d_prftch_dis(&self) -> D_PRFTCH_DIS_R {
        D_PRFTCH_DIS_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SPI Mode 3 select."]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
    #[inline(always)]
    pub fn prftch_dis(&self) -> PRFTCH_DIS_R {
        PRFTCH_DIS_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select dual protocol."]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Select active clock edge for input data."]
    #[inline(always)]
    pub fn rfclk(&self) -> RFCLK_R {
        RFCLK_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Feedback clock select."]
    #[inline(always)]
    pub fn fbclk(&self) -> FBCLK_R {
        FBCLK_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - This field contains the number of serial clock periods without the processor reading data in memory mode, which will cause the SPIFI hardware to terminate the command by driving the CS pin high and negating the CMD bit in the Status register. (This allows the flash memory to enter a lower-power state.) If the processor reads data from the flash region after a time-out, the command in the Memory Command Register is issued again."]
    #[inline(always)]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
    #[doc = "Bits 16:19 - This field controls the minimum CS high time, expressed as a number of serial clock periods minus one."]
    #[inline(always)]
    pub fn cshigh(&mut self) -> _CSHIGHW {
        _CSHIGHW { w: self }
    }
    #[doc = "Bit 21 - This bit allows conditioning of memory mode prefetches based on the AHB HPROT (instruction/data) access information. A 1 in this register means that the SPIFI will not attempt a speculative prefetch when it encounters data accesses."]
    #[inline(always)]
    pub fn d_prftch_dis(&mut self) -> _D_PRFTCH_DISW {
        _D_PRFTCH_DISW { w: self }
    }
    #[doc = "Bit 22 - If this bit is 1 when a command ends, the SPIFI will assert its interrupt request output. See INTRQ in the status register for further details."]
    #[inline(always)]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 23 - SPI Mode 3 select."]
    #[inline(always)]
    pub fn mode3(&mut self) -> _MODE3W {
        _MODE3W { w: self }
    }
    #[doc = "Bit 27 - Cache prefetching enable. The SPIFI includes an internal cache. A 1 in this bit disables prefetching of cache lines."]
    #[inline(always)]
    pub fn prftch_dis(&mut self) -> _PRFTCH_DISW {
        _PRFTCH_DISW { w: self }
    }
    #[doc = "Bit 28 - Select dual protocol."]
    #[inline(always)]
    pub fn dual(&mut self) -> _DUALW {
        _DUALW { w: self }
    }
    #[doc = "Bit 29 - Select active clock edge for input data."]
    #[inline(always)]
    pub fn rfclk(&mut self) -> _RFCLKW {
        _RFCLKW { w: self }
    }
    #[doc = "Bit 30 - Feedback clock select."]
    #[inline(always)]
    pub fn fbclk(&mut self) -> _FBCLKW {
        _FBCLKW { w: self }
    }
    #[doc = "Bit 31 - A 1 in this bit enables the DMA Request output from the SPIFI. Set this bit only when a DMA channel is used to transfer data in peripheral mode. Do not set this bit when a DMA channel is used for memory-to-memory transfers from the SPIFI memory area. DMAEN should only be used in Command mode."]
    #[inline(always)]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
