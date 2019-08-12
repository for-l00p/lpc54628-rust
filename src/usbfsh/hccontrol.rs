#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCCONTROL {
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
pub type CBSR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CBSRW<'a> {
    w: &'a mut W,
}
impl<'a> _CBSRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PLEW<'a> {
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
pub type IE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IEW<'a> {
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
pub type CLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLEW<'a> {
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
pub type BLE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLEW<'a> {
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
pub type HCFS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HCFSW<'a> {
    w: &'a mut W,
}
impl<'a> _HCFSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IRW<'a> {
    w: &'a mut W,
}
impl<'a> _IRW<'a> {
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
pub type RWC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWCW<'a> {
    w: &'a mut W,
}
impl<'a> _RWCW<'a> {
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
pub type RWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RWEW<'a> {
    w: &'a mut W,
}
impl<'a> _RWEW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ControlBulkServiceRatio."]
    #[inline(always)]
    pub fn cbsr(&self) -> CBSR_R {
        CBSR_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - PeriodicListEnable."]
    #[inline(always)]
    pub fn ple(&self) -> PLE_R {
        PLE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IsochronousEnable."]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ControlListEnable."]
    #[inline(always)]
    pub fn cle(&self) -> CLE_R {
        CLE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub fn ble(&self) -> BLE_R {
        BLE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub fn hcfs(&self) -> HCFS_R {
        HCFS_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - ControlBulkServiceRatio."]
    #[inline(always)]
    pub fn cbsr(&mut self) -> _CBSRW {
        _CBSRW { w: self }
    }
    #[doc = "Bit 2 - PeriodicListEnable."]
    #[inline(always)]
    pub fn ple(&mut self) -> _PLEW {
        _PLEW { w: self }
    }
    #[doc = "Bit 3 - IsochronousEnable."]
    #[inline(always)]
    pub fn ie(&mut self) -> _IEW {
        _IEW { w: self }
    }
    #[doc = "Bit 4 - ControlListEnable."]
    #[inline(always)]
    pub fn cle(&mut self) -> _CLEW {
        _CLEW { w: self }
    }
    #[doc = "Bit 5 - BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub fn ble(&mut self) -> _BLEW {
        _BLEW { w: self }
    }
    #[doc = "Bits 6:7 - HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub fn hcfs(&mut self) -> _HCFSW {
        _HCFSW { w: self }
    }
    #[doc = "Bit 8 - InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub fn ir(&mut self) -> _IRW {
        _IRW { w: self }
    }
    #[doc = "Bit 9 - RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub fn rwc(&mut self) -> _RWCW {
        _RWCW { w: self }
    }
    #[doc = "Bit 10 - RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub fn rwe(&mut self) -> _RWEW {
        _RWEW { w: self }
    }
}
