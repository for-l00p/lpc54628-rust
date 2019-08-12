#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOCFG {
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
#[doc = "Possible values of the field `ENABLETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLETXR {
    #[doc = "The transmit FIFO is not enabled."]
    DISABLED,
    #[doc = "The transmit FIFO is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for ENABLETXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENABLETXR::DISABLED => false,
            ENABLETXR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLETX_R = crate::FR<bool, ENABLETXR>;
impl ENABLETX_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLETXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLETXR::ENABLED
    }
}
#[doc = "Values that can be written to the field `ENABLETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLETXW {
    #[doc = "The transmit FIFO is not enabled."]
    DISABLED,
    #[doc = "The transmit FIFO is enabled."]
    ENABLED,
}
impl ENABLETXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLETXW::DISABLED => false,
            ENABLETXW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENABLETXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLETXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLETXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLETXW::DISABLED)
    }
    #[doc = "The transmit FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLETXW::ENABLED)
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
#[doc = "Possible values of the field `ENABLERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLERXR {
    #[doc = "The receive FIFO is not enabled."]
    DISABLED,
    #[doc = "The receive FIFO is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for ENABLERXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ENABLERXR::DISABLED => false,
            ENABLERXR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLERX_R = crate::FR<bool, ENABLERXR>;
impl ENABLERX_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLERXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLERXR::ENABLED
    }
}
#[doc = "Values that can be written to the field `ENABLERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLERXW {
    #[doc = "The receive FIFO is not enabled."]
    DISABLED,
    #[doc = "The receive FIFO is enabled."]
    ENABLED,
}
impl ENABLERXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLERXW::DISABLED => false,
            ENABLERXW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ENABLERXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLERXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLERXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The receive FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLERXW::DISABLED)
    }
    #[doc = "The receive FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLERXW::ENABLED)
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
pub type SIZE_R = crate::FR<u8, u8>;
#[doc = "Possible values of the field `DMATX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATXR {
    #[doc = "DMA is not used for the transmit function."]
    DISABLED,
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for DMATXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DMATXR::DISABLED => false,
            DMATXR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DMATX_R = crate::FR<bool, DMATXR>;
impl DMATX_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATXR::ENABLED
    }
}
#[doc = "Values that can be written to the field `DMATX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATXW {
    #[doc = "DMA is not used for the transmit function."]
    DISABLED,
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl DMATXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMATXW::DISABLED => false,
            DMATXW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DMATXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMATXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMATXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not used for the transmit function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATXW::DISABLED)
    }
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATXW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `DMARX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARXR {
    #[doc = "DMA is not used for the receive function."]
    DISABLED,
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for DMARXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DMARXR::DISABLED => false,
            DMARXR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DMARX_R = crate::FR<bool, DMARXR>;
impl DMARX_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARXR::ENABLED
    }
}
#[doc = "Values that can be written to the field `DMARX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARXW {
    #[doc = "DMA is not used for the receive function."]
    DISABLED,
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl DMARXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DMARXW::DISABLED => false,
            DMARXW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DMARXW<'a> {
    w: &'a mut W,
}
impl<'a> _DMARXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMARXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is not used for the receive function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARXW::DISABLED)
    }
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARXW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `WAKETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKETXR {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for WAKETXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WAKETXR::DISABLED => false,
            WAKETXR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WAKETX_R = crate::FR<bool, WAKETXR>;
impl WAKETX_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKETXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKETXR::ENABLED
    }
}
#[doc = "Values that can be written to the field `WAKETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKETXW {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED,
}
impl WAKETXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKETXW::DISABLED => false,
            WAKETXW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WAKETXW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKETXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKETXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKETXW::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKETXW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `WAKERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKERXR {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for WAKERXR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WAKERXR::DISABLED => false,
            WAKERXR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WAKERX_R = crate::FR<bool, WAKERXR>;
impl WAKERX_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKERXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKERXR::ENABLED
    }
}
#[doc = "Values that can be written to the field `WAKERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKERXW {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED,
}
impl WAKERXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKERXW::DISABLED => false,
            WAKERXW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WAKERXW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKERXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKERXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKERXW::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKERXW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EMPTYTX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMPTYTXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTYTXW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EMPTYRX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EMPTYRXW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTYRXW<'a> {
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
#[doc = "Possible values of the field `POPDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POPDBGR {
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    DO_NOT_POP,
    #[doc = "A debug read will cause the FIFO to pop."]
    POP,
}
impl crate::ToBits<bool> for POPDBGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            POPDBGR::DO_NOT_POP => false,
            POPDBGR::POP => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type POPDBG_R = crate::FR<bool, POPDBGR>;
impl POPDBG_R {
    #[doc = "Checks if the value of the field is `DO_NOT_POP`"]
    #[inline(always)]
    pub fn is_do_not_pop(&self) -> bool {
        *self == POPDBGR::DO_NOT_POP
    }
    #[doc = "Checks if the value of the field is `POP`"]
    #[inline(always)]
    pub fn is_pop(&self) -> bool {
        *self == POPDBGR::POP
    }
}
#[doc = "Values that can be written to the field `POPDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POPDBGW {
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    DO_NOT_POP,
    #[doc = "A debug read will cause the FIFO to pop."]
    POP,
}
impl POPDBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            POPDBGW::DO_NOT_POP => false,
            POPDBGW::POP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _POPDBGW<'a> {
    w: &'a mut W,
}
impl<'a> _POPDBGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POPDBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    #[inline(always)]
    pub fn do_not_pop(self) -> &'a mut W {
        self.variant(POPDBGW::DO_NOT_POP)
    }
    #[doc = "A debug read will cause the FIFO to pop."]
    #[inline(always)]
    pub fn pop(self) -> &'a mut W {
        self.variant(POPDBGW::POP)
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
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&self) -> ENABLETX_R {
        ENABLETX_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&self) -> ENABLERX_R {
        ENABLERX_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&self) -> WAKETX_R {
        WAKETX_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&self) -> WAKERX_R {
        WAKERX_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&self) -> EMPTYTX_R {
        EMPTYTX_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&self) -> EMPTYRX_R {
        EMPTYRX_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pop FIFO for debug reads."]
    #[inline(always)]
    pub fn popdbg(&self) -> POPDBG_R {
        POPDBG_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&mut self) -> _ENABLETXW {
        _ENABLETXW { w: self }
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&mut self) -> _ENABLERXW {
        _ENABLERXW { w: self }
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&mut self) -> _DMATXW {
        _DMATXW { w: self }
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&mut self) -> _DMARXW {
        _DMARXW { w: self }
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&mut self) -> _WAKETXW {
        _WAKETXW { w: self }
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&mut self) -> _WAKERXW {
        _WAKERXW { w: self }
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&mut self) -> _EMPTYTXW {
        _EMPTYTXW { w: self }
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&mut self) -> _EMPTYRXW {
        _EMPTYRXW { w: self }
    }
    #[doc = "Bit 18 - Pop FIFO for debug reads."]
    #[inline(always)]
    pub fn popdbg(&mut self) -> _POPDBGW {
        _POPDBGW { w: self }
    }
}
