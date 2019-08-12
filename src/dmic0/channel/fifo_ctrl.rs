#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFO_CTRL {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    DISABLED,
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    ENABLED,
}
impl crate::ToBits<bool> for ENABLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENABLER::DISABLED => false,
            ENABLER::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLE_R = crate::FR<bool, ENABLER>;
impl ENABLE_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLER::ENABLED
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLEW {
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    DISABLED,
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    ENABLED,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::DISABLED => false,
            ENABLEW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED)
    }
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED)
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
#[doc = "Possible values of the field `RESETN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETNR {
    #[doc = "Reset the FIFO."]
    RESET,
    #[doc = "Normal operation"]
    NORMAL,
}
impl crate::ToBits<bool> for RESETNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RESETNR::RESET => false,
            RESETNR::NORMAL => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RESETN_R = crate::FR<bool, RESETNR>;
impl RESETN_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETNR::RESET
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RESETNR::NORMAL
    }
}
#[doc = "Values that can be written to the field `RESETN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETNW {
    #[doc = "Reset the FIFO."]
    RESET,
    #[doc = "Normal operation"]
    NORMAL,
}
impl RESETNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETNW::RESET => false,
            RESETNW::NORMAL => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RESETNW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset the FIFO."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETNW::RESET)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RESETNW::NORMAL)
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
#[doc = "Possible values of the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = "FIFO level interrupts are not enabled."]
    DISABLED,
    #[doc = "FIFO level interrupts are enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTENR::DISABLED => false,
            INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTEN_R = crate::FR<bool, INTENR>;
impl INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENW {
    #[doc = "FIFO level interrupts are not enabled."]
    DISABLED,
    #[doc = "FIFO level interrupts are enabled."]
    ENABLED,
}
impl INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTENW::DISABLED => false,
            INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO level interrupts are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTENW::DISABLED)
    }
    #[doc = "FIFO level interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTENW::ENABLED)
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
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA requests are not enabled."]
    DISABLED,
    #[doc = "DMA requests based on FIFO level are enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for DMAENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DMAENR::DISABLED => false,
            DMAENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DMAEN_R = crate::FR<bool, DMAENR>;
impl DMAEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENW {
    #[doc = "DMA requests are not enabled."]
    DISABLED,
    #[doc = "DMA requests based on FIFO level are enabled."]
    ENABLED,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::DISABLED => false,
            DMAENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA requests are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAENW::DISABLED)
    }
    #[doc = "DMA requests based on FIFO level are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAENW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRIGLVL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRIGLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGLVLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&mut self) -> _RESETNW {
        _RESETNW { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn triglvl(&mut self) -> _TRIGLVLW {
        _TRIGLVLW { w: self }
    }
}
