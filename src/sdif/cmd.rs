#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMD {
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
pub type CMD_INDEX_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CMD_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_INDEXW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RESPONSE_EXPECT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESPONSE_EXPECTW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_EXPECTW<'a> {
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
pub type RESPONSE_LENGTH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RESPONSE_LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPONSE_LENGTHW<'a> {
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
pub type CHECK_RESPONSE_CRC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CHECK_RESPONSE_CRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CHECK_RESPONSE_CRCW<'a> {
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
pub type DATA_EXPECTED_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATA_EXPECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_EXPECTEDW<'a> {
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
pub type READ_WRITE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _READ_WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_WRITEW<'a> {
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
pub type TRANSFER_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TRANSFER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFER_MODEW<'a> {
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
pub type SEND_AUTO_STOP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEND_AUTO_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_AUTO_STOPW<'a> {
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
pub type WAIT_PRVDATA_COMPLETE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WAIT_PRVDATA_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_PRVDATA_COMPLETEW<'a> {
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
pub type STOP_ABORT_CMD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STOP_ABORT_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOP_ABORT_CMDW<'a> {
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
pub type SEND_INITIALIZATION_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEND_INITIALIZATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SEND_INITIALIZATIONW<'a> {
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
pub type UPDATE_CLOCK_REGISTERS_ONLY_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UPDATE_CLOCK_REGISTERS_ONLYW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATE_CLOCK_REGISTERS_ONLYW<'a> {
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
pub type READ_CEATA_DEVICE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _READ_CEATA_DEVICEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_CEATA_DEVICEW<'a> {
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
pub type CCS_EXPECTED_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CCS_EXPECTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _CCS_EXPECTEDW<'a> {
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
pub type ENABLE_BOOT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ENABLE_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLE_BOOTW<'a> {
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
pub type EXPECT_BOOT_ACK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EXPECT_BOOT_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _EXPECT_BOOT_ACKW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DISABLE_BOOT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DISABLE_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_BOOTW<'a> {
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
pub type BOOT_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BOOT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_MODEW<'a> {
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
#[doc = r"Reader of the field"]
pub type VOLT_SWITCH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VOLT_SWITCHW<'a> {
    w: &'a mut W,
}
impl<'a> _VOLT_SWITCHW<'a> {
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
#[doc = r"Reader of the field"]
pub type USE_HOLD_REG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USE_HOLD_REGW<'a> {
    w: &'a mut W,
}
impl<'a> _USE_HOLD_REGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type START_CMD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _START_CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _START_CMDW<'a> {
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
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new((self.bits() & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&self) -> RESPONSE_EXPECT_R {
        RESPONSE_EXPECT_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&self) -> RESPONSE_LENGTH_R {
        RESPONSE_LENGTH_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRC_R {
        CHECK_RESPONSE_CRC_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DATA_EXPECTED_R {
        DATA_EXPECTED_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TRANSFER_MODE_R {
        TRANSFER_MODE_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOP_R {
        SEND_AUTO_STOP_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETE_R {
        WAIT_PRVDATA_COMPLETE_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMD_R {
        STOP_ABORT_CMD_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SEND_INITIALIZATION_R {
        SEND_INITIALIZATION_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLY_R {
        UPDATE_CLOCK_REGISTERS_ONLY_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICE_R {
        READ_CEATA_DEVICE_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CCS_EXPECTED_R {
        CCS_EXPECTED_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&self) -> ENABLE_BOOT_R {
        ENABLE_BOOT_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&self) -> EXPECT_BOOT_ACK_R {
        EXPECT_BOOT_ACK_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&self) -> DISABLE_BOOT_R {
        DISABLE_BOOT_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&self) -> VOLT_SWITCH_R {
        VOLT_SWITCH_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&self) -> USE_HOLD_REG_R {
        USE_HOLD_REG_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&self) -> START_CMD_R {
        START_CMD_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> _CMD_INDEXW {
        _CMD_INDEXW { w: self }
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&mut self) -> _RESPONSE_EXPECTW {
        _RESPONSE_EXPECTW { w: self }
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&mut self) -> _RESPONSE_LENGTHW {
        _RESPONSE_LENGTHW { w: self }
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&mut self) -> _CHECK_RESPONSE_CRCW {
        _CHECK_RESPONSE_CRCW { w: self }
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&mut self) -> _DATA_EXPECTEDW {
        _DATA_EXPECTEDW { w: self }
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&mut self) -> _READ_WRITEW {
        _READ_WRITEW { w: self }
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&mut self) -> _TRANSFER_MODEW {
        _TRANSFER_MODEW { w: self }
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&mut self) -> _SEND_AUTO_STOPW {
        _SEND_AUTO_STOPW { w: self }
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&mut self) -> _WAIT_PRVDATA_COMPLETEW {
        _WAIT_PRVDATA_COMPLETEW { w: self }
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&mut self) -> _STOP_ABORT_CMDW {
        _STOP_ABORT_CMDW { w: self }
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&mut self) -> _SEND_INITIALIZATIONW {
        _SEND_INITIALIZATIONW { w: self }
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&mut self) -> _UPDATE_CLOCK_REGISTERS_ONLYW {
        _UPDATE_CLOCK_REGISTERS_ONLYW { w: self }
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&mut self) -> _READ_CEATA_DEVICEW {
        _READ_CEATA_DEVICEW { w: self }
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&mut self) -> _CCS_EXPECTEDW {
        _CCS_EXPECTEDW { w: self }
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&mut self) -> _ENABLE_BOOTW {
        _ENABLE_BOOTW { w: self }
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&mut self) -> _EXPECT_BOOT_ACKW {
        _EXPECT_BOOT_ACKW { w: self }
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&mut self) -> _DISABLE_BOOTW {
        _DISABLE_BOOTW { w: self }
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> _BOOT_MODEW {
        _BOOT_MODEW { w: self }
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&mut self) -> _VOLT_SWITCHW {
        _VOLT_SWITCHW { w: self }
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&mut self) -> _USE_HOLD_REGW {
        _USE_HOLD_REGW { w: self }
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&mut self) -> _START_CMDW {
        _START_CMDW { w: self }
    }
}
