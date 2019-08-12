#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWWAKE {
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
pub type FORCEWAKE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FORCEWAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEWAKEW<'a> {
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
pub type FCWAKE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FCWAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCWAKEW<'a> {
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
pub type WAKEDMIC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WAKEDMICW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEDMICW<'a> {
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
pub type WAKEDMA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WAKEDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEDMAW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&self) -> FORCEWAKE_R {
        FORCEWAKE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&self) -> FCWAKE_R {
        FCWAKE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn wakedmic(&self) -> WAKEDMIC_R {
        WAKEDMIC_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
    #[inline(always)]
    pub fn wakedma(&self) -> WAKEDMA_R {
        WAKEDMA_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&mut self) -> _FORCEWAKEW {
        _FORCEWAKEW { w: self }
    }
    #[doc = "Bit 1 - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&mut self) -> _FCWAKEW {
        _FCWAKEW { w: self }
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn wakedmic(&mut self) -> _WAKEDMICW {
        _WAKEDMICW { w: self }
    }
    #[doc = "Bit 3 - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
    #[inline(always)]
    pub fn wakedma(&mut self) -> _WAKEDMAW {
        _WAKEDMAW { w: self }
    }
}
