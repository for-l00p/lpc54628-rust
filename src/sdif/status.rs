#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
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
        0x0406
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type FIFO_RX_WATERMARK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FIFO_RX_WATERMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_RX_WATERMARKW<'a> {
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
pub type FIFO_TX_WATERMARK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FIFO_TX_WATERMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_TX_WATERMARKW<'a> {
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
pub type FIFO_EMPTY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FIFO_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_EMPTYW<'a> {
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
pub type FIFO_FULL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FIFO_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_FULLW<'a> {
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
pub type CMDFSMSTATES_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CMDFSMSTATESW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDFSMSTATESW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DATA_3_STATUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATA_3_STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_3_STATUSW<'a> {
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
pub type DATA_BUSY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATA_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BUSYW<'a> {
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
pub type DATA_STATE_MC_BUSY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATA_STATE_MC_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_STATE_MC_BUSYW<'a> {
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
pub type RESPONSE_INDEX_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RESPONSE_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_INDEXW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 11)) | (((value as u32) & 0x3f) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FIFO_COUNT_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _FIFO_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_COUNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 17)) | (((value as u32) & 0x1fff) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DMA_ACK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMA_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ACKW<'a> {
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
pub type DMA_REQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMA_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_REQW<'a> {
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
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FIFO_RX_WATERMARK_R {
        FIFO_RX_WATERMARK_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FIFO_TX_WATERMARK_R {
        FIFO_TX_WATERMARK_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&self) -> CMDFSMSTATES_R {
        CMDFSMSTATES_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&self) -> DATA_3_STATUS_R {
        DATA_3_STATUS_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&self) -> DATA_BUSY_R {
        DATA_BUSY_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DATA_STATE_MC_BUSY_R {
        DATA_STATE_MC_BUSY_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&self) -> RESPONSE_INDEX_R {
        RESPONSE_INDEX_R::new(((self.bits() >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&self) -> FIFO_COUNT_R {
        FIFO_COUNT_R::new(((self.bits() >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&self) -> DMA_ACK_R {
        DMA_ACK_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&mut self) -> _FIFO_RX_WATERMARKW {
        _FIFO_RX_WATERMARKW { w: self }
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&mut self) -> _FIFO_TX_WATERMARKW {
        _FIFO_TX_WATERMARKW { w: self }
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&mut self) -> _FIFO_EMPTYW {
        _FIFO_EMPTYW { w: self }
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> _FIFO_FULLW {
        _FIFO_FULLW { w: self }
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&mut self) -> _CMDFSMSTATESW {
        _CMDFSMSTATESW { w: self }
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&mut self) -> _DATA_3_STATUSW {
        _DATA_3_STATUSW { w: self }
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\] 0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&mut self) -> _DATA_BUSYW {
        _DATA_BUSYW { w: self }
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&mut self) -> _DATA_STATE_MC_BUSYW {
        _DATA_STATE_MC_BUSYW { w: self }
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&mut self) -> _RESPONSE_INDEXW {
        _RESPONSE_INDEXW { w: self }
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&mut self) -> _FIFO_COUNTW {
        _FIFO_COUNTW { w: self }
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&mut self) -> _DMA_ACKW {
        _DMA_ACKW { w: self }
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&mut self) -> _DMA_REQW {
        _DMA_REQW { w: self }
    }
}
