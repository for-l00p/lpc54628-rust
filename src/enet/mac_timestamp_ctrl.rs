#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_TIMESTAMP_CTRL {
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
        0x2000
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type TSENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENAW<'a> {
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
pub type TSCFUPDT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSCFUPDTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCFUPDTW<'a> {
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
pub type TSINIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSINITW<'a> {
    w: &'a mut W,
}
impl<'a> _TSINITW<'a> {
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
pub type TSUPDT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSUPDTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUPDTW<'a> {
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
pub type TSTRIG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTRIGW<'a> {
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
pub type TADDREG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TADDREGW<'a> {
    w: &'a mut W,
}
impl<'a> _TADDREGW<'a> {
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
pub type TSENALL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSENALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENALLW<'a> {
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
pub type TSCTRLSSR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSCTRLSSRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSCTRLSSRW<'a> {
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
pub type TSVER2ENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSVER2ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSVER2ENAW<'a> {
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
pub type TSIPENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSIPENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIPENAW<'a> {
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
pub type TSIPV6ENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSIPV6ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIPV6ENAW<'a> {
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
pub type TSIPV4ENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSIPV4ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIPV4ENAW<'a> {
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
pub type TSEVTENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSEVTENAW<'a> {
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
pub type TSMSTRENA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSMSTRENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TSMSTRENAW<'a> {
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
pub type SNAPTYPSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SNAPTYPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SNAPTYPSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TSENMACADDR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TSENMACADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENMACADDRW<'a> {
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
pub type TXTTSSTSM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXTTSSTSMW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTTSSTSMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type AV8021ASMEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AV8021ASMENW<'a> {
    w: &'a mut W,
}
impl<'a> _AV8021ASMENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    #[inline(always)]
    pub fn taddreg(&self) -> TADDREG_R {
        TADDREG_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    #[inline(always)]
    pub fn tsevtena(&self) -> TSEVTENA_R {
        TSEVTENA_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    #[inline(always)]
    pub fn txttsstsm(&self) -> TXTTSSTSM_R {
        TXTTSSTSM_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - AV 802."]
    #[inline(always)]
    pub fn av8021asmen(&self) -> AV8021ASMEN_R {
        AV8021ASMEN_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enable Timestamp When this bit is set, the timestamp is added for Transmit and Receive packets."]
    #[inline(always)]
    pub fn tsena(&mut self) -> _TSENAW {
        _TSENAW { w: self }
    }
    #[doc = "Bit 1 - Fine or Coarse Timestamp Update When this bit is set, the Fine method is used to update system timestamp."]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> _TSCFUPDTW {
        _TSCFUPDTW { w: self }
    }
    #[doc = "Bit 2 - Initialize Timestamp When this bit is set, the system time is initialized (overwritten) with the value specified in the MAC Register 80 (System Time Seconds Update."]
    #[inline(always)]
    pub fn tsinit(&mut self) -> _TSINITW {
        _TSINITW { w: self }
    }
    #[doc = "Bit 3 - Update Timestamp When this bit is set, the system time is updated (added or subtracted) with the value specified in MAC System Time Seconds Update Table 753 and MAC System Time Nanoseconds Update Table 754."]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> _TSUPDTW {
        _TSUPDTW { w: self }
    }
    #[doc = "Bit 4 - Enable Timestamp Interrupt Trigger When this bit is set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register."]
    #[inline(always)]
    pub fn tstrig(&mut self) -> _TSTRIGW {
        _TSTRIGW { w: self }
    }
    #[doc = "Bit 5 - Update Addend Register When this bit is set, the content of the Timestamp Addend register is updated in the PTP block for fine correction."]
    #[inline(always)]
    pub fn taddreg(&mut self) -> _TADDREGW {
        _TADDREGW { w: self }
    }
    #[doc = "Bit 8 - Enable Timestamp for All Packets When this bit is set, the timestamp snapshot is enabled for all packets received by the MAC."]
    #[inline(always)]
    pub fn tsenall(&mut self) -> _TSENALLW {
        _TSENALLW { w: self }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When this bit is set, the Timestamp Low register rolls over after 0x3B9AC9FF value (that is, 1 nanosecond accuracy) and increments the timestamp (High) seconds."]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> _TSCTRLSSRW {
        _TSCTRLSSRW { w: self }
    }
    #[doc = "Bit 10 - Enable PTP Packet Processing for Version 2 Format When this bit is set, the IEEE 1588 version 2 format is used to process the PTP packets."]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> _TSVER2ENAW {
        _TSVER2ENAW { w: self }
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Packets When this bit is set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet packets."]
    #[inline(always)]
    pub fn tsipena(&mut self) -> _TSIPENAW {
        _TSIPENAW { w: self }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Packets Sent over 1Pv6-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv6-UDP packets."]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> _TSIPV6ENAW {
        _TSIPV6ENAW { w: self }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Packets Sent over IPv4-UDP When this bit is set, the MAC receiver processes the PTP packets encapsulated in IPv4-UDP packets."]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> _TSIPV4ENAW {
        _TSIPV4ENAW { w: self }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When this bit is set, the timestamp snapshot is taken only for event messages (SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp)."]
    #[inline(always)]
    pub fn tsevtena(&mut self) -> _TSEVTENAW {
        _TSEVTENAW { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When this bit is set, the snapshot is taken only for the messages that are relevant to the master node."]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> _TSMSTRENAW {
        _TSMSTRENAW { w: self }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits, along with Bits 15 and 14, decide the set of PTP packet types for which snapshot needs to be taken."]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> _SNAPTYPSELW {
        _SNAPTYPSELW { w: self }
    }
    #[doc = "Bit 18 - Enable MAC Address for PTP Packet Filtering When this bit is set, the DA MAC address (that matches any MAC Address register) is used to filter the PTP packets when PTP is directly sent over Ethernet."]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> _TSENMACADDRW {
        _TSENMACADDRW { w: self }
    }
    #[doc = "Bit 24 - Transmit Timestamp Status Mode When this bit is set, the MAC overwrites the earlier transmit timestamp status even if it is not read by the software."]
    #[inline(always)]
    pub fn txttsstsm(&mut self) -> _TXTTSSTSMW {
        _TXTTSSTSMW { w: self }
    }
    #[doc = "Bit 28 - AV 802."]
    #[inline(always)]
    pub fn av8021asmen(&mut self) -> _AV8021ASMENW {
        _AV8021ASMENW { w: self }
    }
}
