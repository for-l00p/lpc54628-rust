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
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type CONTROLLER_RESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CONTROLLER_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTROLLER_RESETW<'a> {
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
pub type FIFO_RESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FIFO_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_RESETW<'a> {
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
pub type DMA_RESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMA_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_RESETW<'a> {
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
pub type INT_ENABLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INT_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ENABLEW<'a> {
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
pub type READ_WAIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _READ_WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_WAITW<'a> {
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
#[doc = r"Reader of the field"]
pub type SEND_IRQ_RESPONSE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEND_IRQ_RESPONSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_IRQ_RESPONSEW<'a> {
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
pub type ABORT_READ_DATA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ABORT_READ_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _ABORT_READ_DATAW<'a> {
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
pub type SEND_CCSD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEND_CCSDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_CCSDW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SEND_AUTO_STOP_CCSD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEND_AUTO_STOP_CCSDW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_AUTO_STOP_CCSDW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CEATA_DEVICE_INTERRUPT_STATUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CEATA_DEVICE_INTERRUPT_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _CEATA_DEVICE_INTERRUPT_STATUSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CARD_VOLTAGE_A0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CARD_VOLTAGE_A0W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_VOLTAGE_A0W<'a> {
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
pub type CARD_VOLTAGE_A1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CARD_VOLTAGE_A1W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_VOLTAGE_A1W<'a> {
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
pub type CARD_VOLTAGE_A2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CARD_VOLTAGE_A2W<'a> {
    w: &'a mut W,
}
impl<'a> _CARD_VOLTAGE_A2W<'a> {
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
#[doc = r"Reader of the field"]
pub type USE_INTERNAL_DMAC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USE_INTERNAL_DMACW<'a> {
    w: &'a mut W,
}
impl<'a> _USE_INTERNAL_DMACW<'a> {
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
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&self) -> CONTROLLER_RESET_R {
        CONTROLLER_RESET_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&self) -> FIFO_RESET_R {
        FIFO_RESET_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&self) -> DMA_RESET_R {
        DMA_RESET_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&self) -> READ_WAIT_R {
        READ_WAIT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&self) -> SEND_IRQ_RESPONSE_R {
        SEND_IRQ_RESPONSE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&self) -> ABORT_READ_DATA_R {
        ABORT_READ_DATA_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SEND_CCSD_R {
        SEND_CCSD_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> SEND_AUTO_STOP_CCSD_R {
        SEND_AUTO_STOP_CCSD_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> CEATA_DEVICE_INTERRUPT_STATUS_R {
        CEATA_DEVICE_INTERRUPT_STATUS_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&self) -> CARD_VOLTAGE_A0_R {
        CARD_VOLTAGE_A0_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&self) -> CARD_VOLTAGE_A1_R {
        CARD_VOLTAGE_A1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&self) -> CARD_VOLTAGE_A2_R {
        CARD_VOLTAGE_A2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&self) -> USE_INTERNAL_DMAC_R {
        USE_INTERNAL_DMAC_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&mut self) -> _CONTROLLER_RESETW {
        _CONTROLLER_RESETW { w: self }
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&mut self) -> _FIFO_RESETW {
        _FIFO_RESETW { w: self }
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&mut self) -> _DMA_RESETW {
        _DMA_RESETW { w: self }
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&mut self) -> _INT_ENABLEW {
        _INT_ENABLEW { w: self }
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&mut self) -> _READ_WAITW {
        _READ_WAITW { w: self }
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&mut self) -> _SEND_IRQ_RESPONSEW {
        _SEND_IRQ_RESPONSEW { w: self }
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&mut self) -> _ABORT_READ_DATAW {
        _ABORT_READ_DATAW { w: self }
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&mut self) -> _SEND_CCSDW {
        _SEND_CCSDW { w: self }
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&mut self) -> _SEND_AUTO_STOP_CCSDW {
        _SEND_AUTO_STOP_CCSDW { w: self }
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&mut self) -> _CEATA_DEVICE_INTERRUPT_STATUSW {
        _CEATA_DEVICE_INTERRUPT_STATUSW { w: self }
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&mut self) -> _CARD_VOLTAGE_A0W {
        _CARD_VOLTAGE_A0W { w: self }
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&mut self) -> _CARD_VOLTAGE_A1W {
        _CARD_VOLTAGE_A1W { w: self }
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&mut self) -> _CARD_VOLTAGE_A2W {
        _CARD_VOLTAGE_A2W { w: self }
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&mut self) -> _USE_INTERNAL_DMACW {
        _USE_INTERNAL_DMACW { w: self }
    }
}
