#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCINTERRUPTSTATUS {
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
pub type SO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SOW<'a> {
    w: &'a mut W,
}
impl<'a> _SOW<'a> {
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
pub type WDH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDHW<'a> {
    w: &'a mut W,
}
impl<'a> _WDHW<'a> {
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
pub type SF_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SFW<'a> {
    w: &'a mut W,
}
impl<'a> _SFW<'a> {
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
pub type RD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RDW<'a> {
    w: &'a mut W,
}
impl<'a> _RDW<'a> {
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
pub type UE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UEW<'a> {
    w: &'a mut W,
}
impl<'a> _UEW<'a> {
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
pub type FNO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FNOW<'a> {
    w: &'a mut W,
}
impl<'a> _FNOW<'a> {
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
pub type RHSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RHSCW<'a> {
    w: &'a mut W,
}
impl<'a> _RHSCW<'a> {
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
pub type OC_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _OCW<'a> {
    w: &'a mut W,
}
impl<'a> _OCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&self) -> SF_R {
        SF_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&self) -> FNO_R {
        FNO_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[inline(always)]
    pub fn rhsc(&self) -> RHSC_R {
        RHSC_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new(((self.bits() >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&mut self) -> _SOW {
        _SOW { w: self }
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&mut self) -> _WDHW {
        _WDHW { w: self }
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&mut self) -> _SFW {
        _SFW { w: self }
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&mut self) -> _RDW {
        _RDW { w: self }
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&mut self) -> _UEW {
        _UEW { w: self }
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&mut self) -> _FNOW {
        _FNOW { w: self }
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[inline(always)]
    pub fn rhsc(&mut self) -> _RHSCW {
        _RHSCW { w: self }
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&mut self) -> _OCW {
        _OCW { w: self }
    }
}
