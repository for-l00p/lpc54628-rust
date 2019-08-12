#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
#[doc = "Possible values of the field `SEQA_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTENR {
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl crate::ToBits<bool> for SEQA_INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEQA_INTENR::DISABLED => false,
            SEQA_INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEQA_INTEN_R = crate::FR<bool, SEQA_INTENR>;
impl SEQA_INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQA_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQA_INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SEQA_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTENW {
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl SEQA_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQA_INTENW::DISABLED => false,
            SEQA_INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SEQA_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQA_INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQA_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQA_INTENW::DISABLED)
    }
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQA_INTENW::ENABLED)
    }
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
#[doc = "Possible values of the field `SEQB_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTENR {
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl crate::ToBits<bool> for SEQB_INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SEQB_INTENR::DISABLED => false,
            SEQB_INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEQB_INTEN_R = crate::FR<bool, SEQB_INTENR>;
impl SEQB_INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQB_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQB_INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SEQB_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTENW {
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl SEQB_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQB_INTENW::DISABLED => false,
            SEQB_INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SEQB_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQB_INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQB_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_INTENW::DISABLED)
    }
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_INTENW::ENABLED)
    }
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
#[doc = "Possible values of the field `OVR_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTENR {
    #[doc = "Disabled. The overrun interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    ENABLED,
}
impl crate::ToBits<bool> for OVR_INTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OVR_INTENR::DISABLED => false,
            OVR_INTENR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OVR_INTEN_R = crate::FR<bool, OVR_INTENR>;
impl OVR_INTEN_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_INTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_INTENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `OVR_INTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTENW {
    #[doc = "Disabled. The overrun interrupt is disabled."]
    DISABLED,
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    ENABLED,
}
impl OVR_INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OVR_INTENW::DISABLED => false,
            OVR_INTENW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OVR_INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVR_INTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_INTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The overrun interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_INTENW::DISABLED)
    }
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_INTENW::ENABLED)
    }
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
#[doc = "Possible values of the field `ADCMPINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN0R {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl crate::ToBits<u8> for ADCMPINTEN0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0R::DISABLED => 0,
            ADCMPINTEN0R::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0R::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN0_R = crate::FR<u8, ADCMPINTEN0R>;
impl ADCMPINTEN0_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN0R::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN0R::CROSSING_THRESHOLD
    }
}
#[doc = "Values that can be written to the field `ADCMPINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN0W {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl ADCMPINTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCMPINTEN0W::DISABLED => 0,
            ADCMPINTEN0W::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0W::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADCMPINTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0W::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN4_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN5_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN5W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN6_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN6W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN7_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN7W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN8_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN8W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN9_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN9W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN10_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN10W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADCMPINTEN11_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _ADCMPINTEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMPINTEN11W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&self) -> SEQA_INTEN_R {
        SEQA_INTEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&self) -> SEQB_INTEN_R {
        SEQB_INTEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&self) -> OVR_INTEN_R {
        OVR_INTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0_R {
        ADCMPINTEN0_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1_R {
        ADCMPINTEN1_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2_R {
        ADCMPINTEN2_R::new(((self.bits() >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3_R {
        ADCMPINTEN3_R::new(((self.bits() >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4_R {
        ADCMPINTEN4_R::new(((self.bits() >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5_R {
        ADCMPINTEN5_R::new(((self.bits() >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6_R {
        ADCMPINTEN6_R::new(((self.bits() >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7_R {
        ADCMPINTEN7_R::new(((self.bits() >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8_R {
        ADCMPINTEN8_R::new(((self.bits() >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9_R {
        ADCMPINTEN9_R::new(((self.bits() >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10_R {
        ADCMPINTEN10_R::new(((self.bits() >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11_R {
        ADCMPINTEN11_R::new(((self.bits() >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&mut self) -> _SEQA_INTENW {
        _SEQA_INTENW { w: self }
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&mut self) -> _SEQB_INTENW {
        _SEQB_INTENW { w: self }
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&mut self) -> _OVR_INTENW {
        _OVR_INTENW { w: self }
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&mut self) -> _ADCMPINTEN0W {
        _ADCMPINTEN0W { w: self }
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&mut self) -> _ADCMPINTEN1W {
        _ADCMPINTEN1W { w: self }
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&mut self) -> _ADCMPINTEN2W {
        _ADCMPINTEN2W { w: self }
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&mut self) -> _ADCMPINTEN3W {
        _ADCMPINTEN3W { w: self }
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&mut self) -> _ADCMPINTEN4W {
        _ADCMPINTEN4W { w: self }
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&mut self) -> _ADCMPINTEN5W {
        _ADCMPINTEN5W { w: self }
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&mut self) -> _ADCMPINTEN6W {
        _ADCMPINTEN6W { w: self }
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&mut self) -> _ADCMPINTEN7W {
        _ADCMPINTEN7W { w: self }
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&mut self) -> _ADCMPINTEN8W {
        _ADCMPINTEN8W { w: self }
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&mut self) -> _ADCMPINTEN9W {
        _ADCMPINTEN9W { w: self }
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&mut self) -> _ADCMPINTEN10W {
        _ADCMPINTEN10W { w: self }
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&mut self) -> _ADCMPINTEN11W {
        _ADCMPINTEN11W { w: self }
    }
}
