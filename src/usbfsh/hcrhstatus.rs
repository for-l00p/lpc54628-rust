#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCRHSTATUS {
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
pub type LPS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSW<'a> {
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
pub type OCI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OCIW<'a> {
    w: &'a mut W,
}
impl<'a> _OCIW<'a> {
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
pub type DRWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRWEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRWEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSCW<'a> {
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
pub type OCIC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OCICW<'a> {
    w: &'a mut W,
}
impl<'a> _OCICW<'a> {
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
pub type CRWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CRWEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRWEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&self) -> OCI_R {
        OCI_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&self) -> DRWE_R {
        DRWE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&self) -> LPSC_R {
        LPSC_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&self) -> CRWE_R {
        CRWE_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&mut self) -> _LPSW {
        _LPSW { w: self }
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&mut self) -> _OCIW {
        _OCIW { w: self }
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&mut self) -> _DRWEW {
        _DRWEW { w: self }
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&mut self) -> _LPSCW {
        _LPSCW { w: self }
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&mut self) -> _OCICW {
        _OCICW { w: self }
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&mut self) -> _CRWEW {
        _CRWEW { w: self }
    }
}
