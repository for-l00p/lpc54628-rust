#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_MDIO_ADDR {
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
pub type MB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MBW<'a> {
    w: &'a mut W,
}
impl<'a> _MBW<'a> {
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
pub type MOC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MOCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NTC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NTCW<'a> {
    w: &'a mut W,
}
impl<'a> _NTCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RDA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RDAW<'a> {
    w: &'a mut W,
}
impl<'a> _RDAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PA_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PAW<'a> {
    w: &'a mut W,
}
impl<'a> _PAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BTB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BTBW<'a> {
    w: &'a mut W,
}
impl<'a> _BTBW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PSE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MII busy."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command."]
    #[inline(always)]
    pub fn moc(&self) -> MOC_R {
        MOC_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - CSR Clock Range."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame."]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits() >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected PHY device."]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits() >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit."]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MII busy."]
    #[inline(always)]
    pub fn mb(&mut self) -> _MBW {
        _MBW { w: self }
    }
    #[doc = "Bits 2:3 - MII Operation Command."]
    #[inline(always)]
    pub fn moc(&mut self) -> _MOCW {
        _MOCW { w: self }
    }
    #[doc = "Bits 8:11 - CSR Clock Range."]
    #[inline(always)]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bits 12:14 - Number of Training Clocks This field controls the number of trailing clock cycles generated on MDC after the end of transmission of MDIO frame."]
    #[inline(always)]
    pub fn ntc(&mut self) -> _NTCW {
        _NTCW { w: self }
    }
    #[doc = "Bits 16:20 - Register/Device Address These bits select the PHY register in selected PHY device."]
    #[inline(always)]
    pub fn rda(&mut self) -> _RDAW {
        _RDAW { w: self }
    }
    #[doc = "Bits 21:25 - Physical Layer Address This field indicates which PHY devices (out of 32 devices) the MAC is accessing."]
    #[inline(always)]
    pub fn pa(&mut self) -> _PAW {
        _PAW { w: self }
    }
    #[doc = "Bit 26 - Back to Back transactions When this bit is set and the NTC has value greater than 0, then the MAC will inform the completion of a read or write command at the end of frame transfer (before the trailing clocks are transmitted)."]
    #[inline(always)]
    pub fn btb(&mut self) -> _BTBW {
        _BTBW { w: self }
    }
    #[doc = "Bit 27 - Preamble Suppression Enable When this bit is set, the SMA will suppress the 32-bit preamble and transmit MDIO frames with only 1 preamble bit."]
    #[inline(always)]
    pub fn pse(&mut self) -> _PSEW {
        _PSEW { w: self }
    }
}
