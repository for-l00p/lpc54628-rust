#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOTH {
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
        0x001f_0000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type TX_WMARK_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TX_WMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_WMARKW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RX_WMARK_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _RX_WMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_WMARKW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DMA_MTS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DMA_MTSW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_MTSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TX_WMARK_R {
        TX_WMARK_R::new((self.bits() & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RX_WMARK_R {
        RX_WMARK_R::new(((self.bits() >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&self) -> DMA_MTS_R {
        DMA_MTS_R::new(((self.bits() >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&mut self) -> _TX_WMARKW {
        _TX_WMARKW { w: self }
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&mut self) -> _RX_WMARKW {
        _RX_WMARKW { w: self }
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&mut self) -> _DMA_MTSW {
        _DMA_MTSW { w: self }
    }
}
