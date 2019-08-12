#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHBMATPRIO {
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
pub type PRI_ICODE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_ICODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_ICODEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_DCODE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_DCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_DCODEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_SYS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_SYSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SYSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_DMA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_DMAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_ETH_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_ETHW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_ETHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_LCD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_LCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_LCDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_USB0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_USB0W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_USB0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_USB1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_USB1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_USB1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_SDIO_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_SDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SDIOW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_MCAN1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_MCAN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_MCAN1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_MCAN2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_MCAN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_MCAN2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRI_SHA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRI_SHAW<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_SHAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - I-Code bus priority."]
    #[inline(always)]
    pub fn pri_icode(&self) -> PRI_ICODE_R {
        PRI_ICODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&self) -> PRI_DCODE_R {
        PRI_DCODE_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&self) -> PRI_SYS_R {
        PRI_SYS_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:9 - DMA controller priority."]
    #[inline(always)]
    pub fn pri_dma(&self) -> PRI_DMA_R {
        PRI_DMA_R::new(((self.bits() >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&self) -> PRI_ETH_R {
        PRI_ETH_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&self) -> PRI_LCD_R {
        PRI_LCD_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - USB0 DMA priority."]
    #[inline(always)]
    pub fn pri_usb0(&self) -> PRI_USB0_R {
        PRI_USB0_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - USB1 DMA priority."]
    #[inline(always)]
    pub fn pri_usb1(&self) -> PRI_USB1_R {
        PRI_USB1_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - SDIO priority."]
    #[inline(always)]
    pub fn pri_sdio(&self) -> PRI_SDIO_R {
        PRI_SDIO_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - MCAN1 priority."]
    #[inline(always)]
    pub fn pri_mcan1(&self) -> PRI_MCAN1_R {
        PRI_MCAN1_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - MCAN2 priority."]
    #[inline(always)]
    pub fn pri_mcan2(&self) -> PRI_MCAN2_R {
        PRI_MCAN2_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - SHA priority."]
    #[inline(always)]
    pub fn pri_sha(&self) -> PRI_SHA_R {
        PRI_SHA_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - I-Code bus priority."]
    #[inline(always)]
    pub fn pri_icode(&mut self) -> _PRI_ICODEW {
        _PRI_ICODEW { w: self }
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&mut self) -> _PRI_DCODEW {
        _PRI_DCODEW { w: self }
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&mut self) -> _PRI_SYSW {
        _PRI_SYSW { w: self }
    }
    #[doc = "Bits 6:9 - DMA controller priority."]
    #[inline(always)]
    pub fn pri_dma(&mut self) -> _PRI_DMAW {
        _PRI_DMAW { w: self }
    }
    #[doc = "Bits 10:11 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&mut self) -> _PRI_ETHW {
        _PRI_ETHW { w: self }
    }
    #[doc = "Bits 12:13 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&mut self) -> _PRI_LCDW {
        _PRI_LCDW { w: self }
    }
    #[doc = "Bits 14:15 - USB0 DMA priority."]
    #[inline(always)]
    pub fn pri_usb0(&mut self) -> _PRI_USB0W {
        _PRI_USB0W { w: self }
    }
    #[doc = "Bits 16:17 - USB1 DMA priority."]
    #[inline(always)]
    pub fn pri_usb1(&mut self) -> _PRI_USB1W {
        _PRI_USB1W { w: self }
    }
    #[doc = "Bits 18:19 - SDIO priority."]
    #[inline(always)]
    pub fn pri_sdio(&mut self) -> _PRI_SDIOW {
        _PRI_SDIOW { w: self }
    }
    #[doc = "Bits 20:21 - MCAN1 priority."]
    #[inline(always)]
    pub fn pri_mcan1(&mut self) -> _PRI_MCAN1W {
        _PRI_MCAN1W { w: self }
    }
    #[doc = "Bits 22:23 - MCAN2 priority."]
    #[inline(always)]
    pub fn pri_mcan2(&mut self) -> _PRI_MCAN2W {
        _PRI_MCAN2W { w: self }
    }
    #[doc = "Bits 24:25 - SHA priority."]
    #[inline(always)]
    pub fn pri_sha(&mut self) -> _PRI_SHAW {
        _PRI_SHAW { w: self }
    }
}
