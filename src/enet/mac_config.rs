#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_CONFIG {
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
        0x8000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type RE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
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
pub type TE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
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
pub type PRELEN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRELENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRELENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCW<'a> {
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
pub type BL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _BLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRW<'a> {
    w: &'a mut W,
}
impl<'a> _DRW<'a> {
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
pub type DCRS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCRSW<'a> {
    w: &'a mut W,
}
impl<'a> _DCRSW<'a> {
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
pub type DO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DOW<'a> {
    w: &'a mut W,
}
impl<'a> _DOW<'a> {
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
pub type ECRSFD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECRSFDW<'a> {
    w: &'a mut W,
}
impl<'a> _ECRSFDW<'a> {
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
pub type LM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LMW<'a> {
    w: &'a mut W,
}
impl<'a> _LMW<'a> {
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
pub type DM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FES_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type JE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _JEW<'a> {
    w: &'a mut W,
}
impl<'a> _JEW<'a> {
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
pub type JD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _JDW<'a> {
    w: &'a mut W,
}
impl<'a> _JDW<'a> {
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
pub type BE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BEW<'a> {
    w: &'a mut W,
}
impl<'a> _BEW<'a> {
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
pub type WD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDW<'a> {
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
pub type ACS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ACSW<'a> {
    w: &'a mut W,
}
impl<'a> _ACSW<'a> {
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
#[doc = r"Reader of the field"]
pub type CST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type S2KP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _S2KPW<'a> {
    w: &'a mut W,
}
impl<'a> _S2KPW<'a> {
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
pub type GPSLCE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GPSLCEW<'a> {
    w: &'a mut W,
}
impl<'a> _GPSLCEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IPG_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _IPGW<'a> {
    w: &'a mut W,
}
impl<'a> _IPGW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IPC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IPCW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCW<'a> {
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
    #[doc = "Bit 0 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
    #[inline(always)]
    pub fn ecrsfd(&self) -> ECRSFD_R {
        ECRSFD_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Portselect."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IEEE 802."]
    #[inline(always)]
    pub fn s2kp(&self) -> S2KP_R {
        S2KP_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
    #[inline(always)]
    pub fn gpslce(&self) -> GPSLCE_R {
        GPSLCE_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new(((self.bits() >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the MII."]
    #[inline(always)]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 1 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the MII."]
    #[inline(always)]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bits 2:3 - Preamble Length for Transmit packets These bits control the number of preamble bytes that are added to the beginning of every Tx packet."]
    #[inline(always)]
    pub fn prelen(&mut self) -> _PRELENW {
        _PRELENW { w: self }
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC."]
    #[inline(always)]
    pub fn dc(&mut self) -> _DCW {
        _DCW { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) the MAC waits before rescheduling a transmission attempt during retries after a collision."]
    #[inline(always)]
    pub fn bl(&mut self) -> _BLW {
        _BLW { w: self }
    }
    #[doc = "Bit 8 - Disable Retry When this bit is set, the MAC will attempt only one transmission."]
    #[inline(always)]
    pub fn dr(&mut self) -> _DRW {
        _DRW { w: self }
    }
    #[doc = "Bit 9 - Disable Carrier Sense During Transmission When this bit is set, the MAC transmitter ignores the MII CRS signal during packet transmission in the half-duplex mode."]
    #[inline(always)]
    pub fn dcrs(&mut self) -> _DCRSW {
        _DCRSW { w: self }
    }
    #[doc = "Bit 10 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the gmii_txen_o is asserted in Half-Duplex mode."]
    #[inline(always)]
    pub fn do_(&mut self) -> _DOW {
        _DOW { w: self }
    }
    #[doc = "Bit 11 - Enable Carrier Sense Before Transmission in Full-Duplex Mode When this bit is set, the MAC transmitter checks the CRS signal before packet transmission in the full-duplex mode."]
    #[inline(always)]
    pub fn ecrsfd(&mut self) -> _ECRSFDW {
        _ECRSFDW { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in loopback mode at MII."]
    #[inline(always)]
    pub fn lm(&mut self) -> _LMW {
        _LMW { w: self }
    }
    #[doc = "Bit 13 - Duplex Mode When this bit is set, the MAC operates in a Full-Duplex mode where it can transmit and receive simultaneously."]
    #[inline(always)]
    pub fn dm(&mut self) -> _DMW {
        _DMW { w: self }
    }
    #[doc = "Bit 14 - Speed Indicates the speed in Fast Ethernet (MII) mode: This bit is reserved (RO) by default and is enabled only when RMII/SMII is enabled during configuration."]
    #[inline(always)]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 16 - Jumbo Frame Enable When this bit is set, MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&mut self) -> _JEW {
        _JEW { w: self }
    }
    #[doc = "Bit 17 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter, and can transfer frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn jd(&mut self) -> _JDW {
        _JDW { w: self }
    }
    #[doc = "Bit 18 - Packet Burst Enable When this bit is set, the MAC allows packet bursting during transmission in the MII half-duplex mode."]
    #[inline(always)]
    pub fn be(&mut self) -> _BEW {
        _BEW { w: self }
    }
    #[doc = "Bit 19 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver, and can receive frames of up to 16,384 bytes."]
    #[inline(always)]
    pub fn wd(&mut self) -> _WDW {
        _WDW { w: self }
    }
    #[doc = "Bit 20 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming packets only if the value of the length field is less than 1,536 bytes."]
    #[inline(always)]
    pub fn acs(&mut self) -> _ACSW {
        _ACSW { w: self }
    }
    #[doc = "Bit 21 - CRC stripping for Type packets When this bit is set, the last four bytes (FCS) of all packets of Ether type (type field greater than 1,536) are stripped and dropped before forwarding the packet to the application."]
    #[inline(always)]
    pub fn cst(&mut self) -> _CSTW {
        _CSTW { w: self }
    }
    #[doc = "Bit 22 - IEEE 802."]
    #[inline(always)]
    pub fn s2kp(&mut self) -> _S2KPW {
        _S2KPW { w: self }
    }
    #[doc = "Bit 23 - Giant Packet Size Limit Control Enable When this bit is set, the MAC considers the value in GPSL field in MAC Ext Configuration register to declare a received packet as Giant packet."]
    #[inline(always)]
    pub fn gpslce(&mut self) -> _GPSLCEW {
        _GPSLCEW { w: self }
    }
    #[doc = "Bits 24:26 - Inter-Packet Gap These bits control the minimum IPG between packets during transmission."]
    #[inline(always)]
    pub fn ipg(&mut self) -> _IPGW {
        _IPGW { w: self }
    }
    #[doc = "Bit 27 - Checksum Offload When set, this bit enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&mut self) -> _IPCW {
        _IPCW { w: self }
    }
}
