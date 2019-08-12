#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTSC1 {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PED_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PEDW<'a> {
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
pub type PEDC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PEDCW<'a> {
    w: &'a mut W,
}
impl<'a> _PEDCW<'a> {
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
pub type OCA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OCAW<'a> {
    w: &'a mut W,
}
impl<'a> _OCAW<'a> {
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
pub type OCC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OCCW<'a> {
    w: &'a mut W,
}
impl<'a> _OCCW<'a> {
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
pub type FPR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPRW<'a> {
    w: &'a mut W,
}
impl<'a> _FPRW<'a> {
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
pub type SUSP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPW<'a> {
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
pub type PR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
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
pub type SUS_L1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SUS_L1W<'a> {
    w: &'a mut W,
}
impl<'a> _SUS_L1W<'a> {
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
pub type LS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PPW<'a> {
    w: &'a mut W,
}
impl<'a> _PPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PIC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PICW<'a> {
    w: &'a mut W,
}
impl<'a> _PICW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PTC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PTCW<'a> {
    w: &'a mut W,
}
impl<'a> _PTCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PSPD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PSPDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WOO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WOOW<'a> {
    w: &'a mut W,
}
impl<'a> _WOOW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SUS_STAT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SUS_STATW<'a> {
    w: &'a mut W,
}
impl<'a> _SUS_STATW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DEV_ADD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DEV_ADDW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ADDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&self) -> PEDC_R {
        PEDC_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
    #[inline(always)]
    pub fn sus_l1(&self) -> SUS_L1_R {
        SUS_L1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits() >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&self) -> WOO_R {
        WOO_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
    #[inline(always)]
    pub fn sus_stat(&self) -> SUS_STAT_R {
        SUS_STAT_R::new(((self.bits() >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:31 - Device Address for LPM tokens."]
    #[inline(always)]
    pub fn dev_add(&self) -> DEV_ADD_R {
        DEV_ADD_R::new(((self.bits() >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&mut self) -> _CCSW {
        _CCSW { w: self }
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&mut self) -> _CSCW {
        _CSCW { w: self }
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&mut self) -> _PEDW {
        _PEDW { w: self }
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&mut self) -> _PEDCW {
        _PEDCW { w: self }
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&mut self) -> _OCAW {
        _OCAW { w: self }
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&mut self) -> _OCCW {
        _OCCW { w: self }
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&mut self) -> _FPRW {
        _FPRW { w: self }
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
    #[doc = "Bit 9 - Suspend using L1 0b = Suspend using L2 1b = Suspend using L1 When this bit is set to a 1 and a non-zero value is specified in the Device Address field, the host controller will generate an LPM Token to enter the L1 state whenever software writes a one to the Suspend bit, as well as L1 exit timing during any device or host-initiated resume."]
    #[inline(always)]
    pub fn sus_l1(&mut self) -> _SUS_L1W {
        _SUS_L1W { w: self }
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&mut self) -> _PPW {
        _PPW { w: self }
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&mut self) -> _PICW {
        _PICW { w: self }
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&mut self) -> _PTCW {
        _PTCW { w: self }
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&mut self) -> _PSPDW {
        _PSPDW { w: self }
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&mut self) -> _WOOW {
        _WOOW { w: self }
    }
    #[doc = "Bits 23:24 - These two bits are used by software to determine whether the most recent L1 suspend request was successful: 00b: Success-state transition was successful (ACK) 01b: Not Yet - Device was unable to enter the L1 state at this time (NYET) 10b: Not supported - Device does not support the L1 state (STALL) 11b: Timeout/Error - Device failed to respond or an error occurred."]
    #[inline(always)]
    pub fn sus_stat(&mut self) -> _SUS_STATW {
        _SUS_STATW { w: self }
    }
    #[doc = "Bits 25:31 - Device Address for LPM tokens."]
    #[inline(always)]
    pub fn dev_add(&mut self) -> _DEV_ADDW {
        _DEV_ADDW { w: self }
    }
}
