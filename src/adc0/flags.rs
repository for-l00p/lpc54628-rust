#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLAGS {
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
pub type THCMP0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP0W<'a> {
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
pub type THCMP1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP1W<'a> {
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
pub type THCMP2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP2W<'a> {
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
pub type THCMP3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP3W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP3W<'a> {
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
pub type THCMP4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP4W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP4W<'a> {
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
pub type THCMP5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP5W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP5W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type THCMP6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP6W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP6W<'a> {
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
pub type THCMP7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP7W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP7W<'a> {
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
pub type THCMP8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP8W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP8W<'a> {
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
pub type THCMP9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP9W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP9W<'a> {
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
pub type THCMP10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP10W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP10W<'a> {
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
pub type THCMP11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _THCMP11W<'a> {
    w: &'a mut W,
}
impl<'a> _THCMP11W<'a> {
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
pub type OVERRUN0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERRUN11_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SEQA_OVR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SEQB_OVR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SEQA_INT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SEQB_INT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type THCMP_INT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVR_INT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Threshold comparison event on Channel 0. Set to 1 upon either an out-of-range result or a threshold-crossing result if enabled to do so in the INTEN register. This bit is cleared by writing a 1."]
    #[inline(always)]
    pub fn thcmp0(&self) -> THCMP0_R {
        THCMP0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Threshold comparison event on Channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp1(&self) -> THCMP1_R {
        THCMP1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Threshold comparison event on Channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp2(&self) -> THCMP2_R {
        THCMP2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Threshold comparison event on Channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp3(&self) -> THCMP3_R {
        THCMP3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Threshold comparison event on Channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp4(&self) -> THCMP4_R {
        THCMP4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Threshold comparison event on Channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp5(&self) -> THCMP5_R {
        THCMP5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Threshold comparison event on Channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp6(&self) -> THCMP6_R {
        THCMP6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Threshold comparison event on Channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp7(&self) -> THCMP7_R {
        THCMP7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Threshold comparison event on Channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp8(&self) -> THCMP8_R {
        THCMP8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Threshold comparison event on Channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp9(&self) -> THCMP9_R {
        THCMP9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Threshold comparison event on Channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp10(&self) -> THCMP10_R {
        THCMP10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Threshold comparison event on Channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp11(&self) -> THCMP11_R {
        THCMP11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mirrors the OVERRRUN status flag from the result register for ADC channel 0"]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mirrors the OVERRRUN status flag from the result register for ADC channel 1"]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mirrors the OVERRRUN status flag from the result register for ADC channel 2"]
    #[inline(always)]
    pub fn overrun2(&self) -> OVERRUN2_R {
        OVERRUN2_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mirrors the OVERRRUN status flag from the result register for ADC channel 3"]
    #[inline(always)]
    pub fn overrun3(&self) -> OVERRUN3_R {
        OVERRUN3_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mirrors the OVERRRUN status flag from the result register for ADC channel 4"]
    #[inline(always)]
    pub fn overrun4(&self) -> OVERRUN4_R {
        OVERRUN4_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mirrors the OVERRRUN status flag from the result register for ADC channel 5"]
    #[inline(always)]
    pub fn overrun5(&self) -> OVERRUN5_R {
        OVERRUN5_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mirrors the OVERRRUN status flag from the result register for ADC channel 6"]
    #[inline(always)]
    pub fn overrun6(&self) -> OVERRUN6_R {
        OVERRUN6_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mirrors the OVERRRUN status flag from the result register for ADC channel 7"]
    #[inline(always)]
    pub fn overrun7(&self) -> OVERRUN7_R {
        OVERRUN7_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Mirrors the OVERRRUN status flag from the result register for ADC channel 8"]
    #[inline(always)]
    pub fn overrun8(&self) -> OVERRUN8_R {
        OVERRUN8_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Mirrors the OVERRRUN status flag from the result register for ADC channel 9"]
    #[inline(always)]
    pub fn overrun9(&self) -> OVERRUN9_R {
        OVERRUN9_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mirrors the OVERRRUN status flag from the result register for ADC channel 10"]
    #[inline(always)]
    pub fn overrun10(&self) -> OVERRUN10_R {
        OVERRUN10_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Mirrors the OVERRRUN status flag from the result register for ADC channel 11"]
    #[inline(always)]
    pub fn overrun11(&self) -> OVERRUN11_R {
        OVERRUN11_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mirrors the global OVERRUN status flag in the SEQA_GDAT register"]
    #[inline(always)]
    pub fn seqa_ovr(&self) -> SEQA_OVR_R {
        SEQA_OVR_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Mirrors the global OVERRUN status flag in the SEQB_GDAT register"]
    #[inline(always)]
    pub fn seqb_ovr(&self) -> SEQB_OVR_R {
        SEQB_OVR_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Sequence A interrupt/DMA trigger. If the MODE bit in the SEQA_CTRL register is 0, this flag will mirror the DATAVALID bit in the sequence A global data register (SEQA_GDAT), which is set at the end of every ADC conversion performed as part of sequence A. It will be cleared automatically when the SEQA_GDAT register is read. If the MODE bit in the SEQA_CTRL register is 1, this flag will be set upon completion of an entire A sequence. In this case it must be cleared by writing a 1 to this SEQA_INT bit. This interrupt must be enabled in the INTEN register."]
    #[inline(always)]
    pub fn seqa_int(&self) -> SEQA_INT_R {
        SEQA_INT_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Sequence A interrupt/DMA trigger. If the MODE bit in the SEQB_CTRL register is 0, this flag will mirror the DATAVALID bit in the sequence A global data register (SEQB_GDAT), which is set at the end of every ADC conversion performed as part of sequence B. It will be cleared automatically when the SEQB_GDAT register is read. If the MODE bit in the SEQB_CTRL register is 1, this flag will be set upon completion of an entire B sequence. In this case it must be cleared by writing a 1 to this SEQB_INT bit. This interrupt must be enabled in the INTEN register."]
    #[inline(always)]
    pub fn seqb_int(&self) -> SEQB_INT_R {
        SEQB_INT_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Threshold Comparison Interrupt. This bit will be set if any of the THCMP flags in the lower bits of this register are set to 1 (due to an enabled out-of-range or threshold-crossing event on any channel). Each type of threshold comparison interrupt on each channel must be individually enabled in the INTEN register to cause this interrupt. This bit will be cleared when all of the individual threshold flags are cleared via writing 1s to those bits."]
    #[inline(always)]
    pub fn thcmp_int(&self) -> THCMP_INT_R {
        THCMP_INT_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Overrun Interrupt flag. Any overrun bit in any of the individual channel data registers will cause this interrupt. In addition, if the MODE bit in either of the SEQn_CTRL registers is 0 then the OVERRUN bit in the corresponding SEQn_GDAT register will also cause this interrupt. This interrupt must be enabled in the INTEN register. This bit will be cleared when all of the individual overrun bits have been cleared via reading the corresponding data registers."]
    #[inline(always)]
    pub fn ovr_int(&self) -> OVR_INT_R {
        OVR_INT_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Threshold comparison event on Channel 0. Set to 1 upon either an out-of-range result or a threshold-crossing result if enabled to do so in the INTEN register. This bit is cleared by writing a 1."]
    #[inline(always)]
    pub fn thcmp0(&mut self) -> _THCMP0W {
        _THCMP0W { w: self }
    }
    #[doc = "Bit 1 - Threshold comparison event on Channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp1(&mut self) -> _THCMP1W {
        _THCMP1W { w: self }
    }
    #[doc = "Bit 2 - Threshold comparison event on Channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp2(&mut self) -> _THCMP2W {
        _THCMP2W { w: self }
    }
    #[doc = "Bit 3 - Threshold comparison event on Channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp3(&mut self) -> _THCMP3W {
        _THCMP3W { w: self }
    }
    #[doc = "Bit 4 - Threshold comparison event on Channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp4(&mut self) -> _THCMP4W {
        _THCMP4W { w: self }
    }
    #[doc = "Bit 5 - Threshold comparison event on Channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp5(&mut self) -> _THCMP5W {
        _THCMP5W { w: self }
    }
    #[doc = "Bit 6 - Threshold comparison event on Channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp6(&mut self) -> _THCMP6W {
        _THCMP6W { w: self }
    }
    #[doc = "Bit 7 - Threshold comparison event on Channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp7(&mut self) -> _THCMP7W {
        _THCMP7W { w: self }
    }
    #[doc = "Bit 8 - Threshold comparison event on Channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp8(&mut self) -> _THCMP8W {
        _THCMP8W { w: self }
    }
    #[doc = "Bit 9 - Threshold comparison event on Channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp9(&mut self) -> _THCMP9W {
        _THCMP9W { w: self }
    }
    #[doc = "Bit 10 - Threshold comparison event on Channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp10(&mut self) -> _THCMP10W {
        _THCMP10W { w: self }
    }
    #[doc = "Bit 11 - Threshold comparison event on Channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp11(&mut self) -> _THCMP11W {
        _THCMP11W { w: self }
    }
}
