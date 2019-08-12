#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBINTR {
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
pub type PCDE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PCDEW<'a> {
    w: &'a mut W,
}
impl<'a> _PCDEW<'a> {
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
pub type FLRE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLREW<'a> {
    w: &'a mut W,
}
impl<'a> _FLREW<'a> {
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
pub type ATL_IRQ_E_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ATL_IRQ_EW<'a> {
    w: &'a mut W,
}
impl<'a> _ATL_IRQ_EW<'a> {
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
pub type ISO_IRQ_E_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ISO_IRQ_EW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_IRQ_EW<'a> {
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
pub type INT_IRQ_E_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INT_IRQ_EW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_IRQ_EW<'a> {
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
pub type SOF_E_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOF_EW<'a> {
    w: &'a mut W,
}
impl<'a> _SOF_EW<'a> {
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
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&self) -> PCDE_R {
        PCDE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&self) -> FLRE_R {
        FLRE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&self) -> ATL_IRQ_E_R {
        ATL_IRQ_E_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&self) -> ISO_IRQ_E_R {
        ISO_IRQ_E_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&self) -> INT_IRQ_E_R {
        INT_IRQ_E_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&self) -> SOF_E_R {
        SOF_E_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Port Change Detect Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn pcde(&mut self) -> _PCDEW {
        _PCDEW { w: self }
    }
    #[doc = "Bit 3 - Frame List Rollover Interrupt Enable: 1: enable 0: disable."]
    #[inline(always)]
    pub fn flre(&mut self) -> _FLREW {
        _FLREW { w: self }
    }
    #[doc = "Bit 16 - ATL IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn atl_irq_e(&mut self) -> _ATL_IRQ_EW {
        _ATL_IRQ_EW { w: self }
    }
    #[doc = "Bit 17 - ISO IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn iso_irq_e(&mut self) -> _ISO_IRQ_EW {
        _ISO_IRQ_EW { w: self }
    }
    #[doc = "Bit 18 - INT IRQ Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn int_irq_e(&mut self) -> _INT_IRQ_EW {
        _INT_IRQ_EW { w: self }
    }
    #[doc = "Bit 19 - SOF Interrupt Enable bit: 1: enable 0: disable."]
    #[inline(always)]
    pub fn sof_e(&mut self) -> _SOF_EW {
        _SOF_EW { w: self }
    }
}
