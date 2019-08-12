#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCRHPORTSTATUS {
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
pub type CCS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CCSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCSW<'a> {
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
pub type PES_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PESW<'a> {
    w: &'a mut W,
}
impl<'a> _PESW<'a> {
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
pub type PSS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PSSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSSW<'a> {
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
pub type POCI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _POCIW<'a> {
    w: &'a mut W,
}
impl<'a> _POCIW<'a> {
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
pub type PRS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PRSW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSW<'a> {
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
pub type PPS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PPSW<'a> {
    w: &'a mut W,
}
impl<'a> _PPSW<'a> {
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
pub type LSDA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LSDAW<'a> {
    w: &'a mut W,
}
impl<'a> _LSDAW<'a> {
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
pub type CSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCW<'a> {
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
pub type PESC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PESCW<'a> {
    w: &'a mut W,
}
impl<'a> _PESCW<'a> {
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
pub type PSSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PSSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PSSCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRSC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PRSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&self) -> POCI_R {
        POCI_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&self) -> PPS_R {
        PPS_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&self) -> LSDA_R {
        LSDA_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&self) -> PESC_R {
        PESC_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&mut self) -> _CCSW {
        _CCSW { w: self }
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&mut self) -> _PESW {
        _PESW { w: self }
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&mut self) -> _PSSW {
        _PSSW { w: self }
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&mut self) -> _POCIW {
        _POCIW { w: self }
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&mut self) -> _PRSW {
        _PRSW { w: self }
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&mut self) -> _PPSW {
        _PPSW { w: self }
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&mut self) -> _LSDAW {
        _LSDAW { w: self }
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&mut self) -> _CSCW {
        _CSCW { w: self }
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&mut self) -> _PESCW {
        _PESCW { w: self }
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&mut self) -> _PSSCW {
        _PSSCW { w: self }
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&mut self) -> _OCICW {
        _OCICW { w: self }
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&mut self) -> _PRSCW {
        _PRSCW { w: self }
    }
}
