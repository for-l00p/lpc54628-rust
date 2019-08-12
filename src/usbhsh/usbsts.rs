#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBSTS {
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
pub type PCD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PCDW<'a> {
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
pub type FLR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLRW<'a> {
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
pub type ATL_IRQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ATL_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ATL_IRQW<'a> {
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
pub type ISO_IRQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ISO_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_IRQW<'a> {
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
pub type INT_IRQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INT_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_IRQW<'a> {
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
pub type SOF_IRQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOF_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SOF_IRQW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&self) -> FLR_R {
        FLR_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&self) -> ATL_IRQ_R {
        ATL_IRQ_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&self) -> ISO_IRQ_R {
        ISO_IRQ_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&self) -> INT_IRQ_R {
        INT_IRQ_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&self) -> SOF_IRQ_R {
        SOF_IRQ_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&mut self) -> _PCDW {
        _PCDW { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&mut self) -> _FLRW {
        _FLRW { w: self }
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&mut self) -> _ATL_IRQW {
        _ATL_IRQW { w: self }
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&mut self) -> _ISO_IRQW {
        _ISO_IRQW { w: self }
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&mut self) -> _INT_IRQW {
        _INT_IRQW { w: self }
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&mut self) -> _SOF_IRQW {
        _SOF_IRQW { w: self }
    }
}
