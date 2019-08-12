#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_CHX_TX_CTRL {
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
pub type ST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STW<'a> {
    w: &'a mut W,
}
impl<'a> _STW<'a> {
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
pub type TCW_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TCWW<'a> {
    w: &'a mut W,
}
impl<'a> _TCWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type OSF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OSFW<'a> {
    w: &'a mut W,
}
impl<'a> _OSFW<'a> {
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
pub type TXPBL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TXPBLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPBLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel."]
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn tx_pbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits() >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state."]
    #[inline(always)]
    pub fn st(&mut self) -> _STW {
        _STW { w: self }
    }
    #[doc = "Bits 1:3 - Transmit Channel Weight This field indicates the weight assigned to the corresponding Transmit channel."]
    #[inline(always)]
    pub fn tcw(&mut self) -> _TCWW {
        _TCWW { w: self }
    }
    #[doc = "Bit 4 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second packet of the Transmit data even before the status for the first packet is obtained."]
    #[inline(always)]
    pub fn osf(&mut self) -> _OSFW {
        _OSFW { w: self }
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA data transfer."]
    #[inline(always)]
    pub fn tx_pbl(&mut self) -> _TXPBLW {
        _TXPBLW { w: self }
    }
}
