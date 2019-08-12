#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVCTL {
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
#[doc = "Possible values of the field `SLVCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUER {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    CONTINUE,
}
impl crate::ToBits<bool> for SLVCONTINUER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVCONTINUER::NO_EFFECT => false,
            SLVCONTINUER::CONTINUE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVCONTINUE_R = crate::FR<bool, SLVCONTINUER>;
impl SLVCONTINUE_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVCONTINUER::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CONTINUE`"]
    #[inline(always)]
    pub fn is_continue_(&self) -> bool {
        *self == SLVCONTINUER::CONTINUE
    }
}
#[doc = "Values that can be written to the field `SLVCONTINUE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVCONTINUEW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    CONTINUE,
}
impl SLVCONTINUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVCONTINUEW::NO_EFFECT => false,
            SLVCONTINUEW::CONTINUE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVCONTINUEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVCONTINUEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVCONTINUEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVCONTINUEW::NO_EFFECT)
    }
    #[doc = "Continue. Informs the Slave function to continue to the next operation, by clearing the SLVPENDING flag in the STAT register. This must be done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation. Automatic Operation has different requirements. SLVCONTINUE should not be set unless SLVPENDING = 1."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(SLVCONTINUEW::CONTINUE)
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
#[doc = "Possible values of the field `SLVNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACKR {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK,
}
impl crate::ToBits<bool> for SLVNACKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVNACKR::NO_EFFECT => false,
            SLVNACKR::NACK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVNACK_R = crate::FR<bool, SLVNACKR>;
impl SLVNACK_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SLVNACKR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == SLVNACKR::NACK
    }
}
#[doc = "Values that can be written to the field `SLVNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNACKW {
    #[doc = "No effect."]
    NO_EFFECT,
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    NACK,
}
impl SLVNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVNACKW::NO_EFFECT => false,
            SLVNACKW::NACK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNACKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SLVNACKW::NO_EFFECT)
    }
    #[doc = "NACK. Causes the Slave function to NACK the master when the slave is receiving data from the master (Slave Receiver mode)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(SLVNACKW::NACK)
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
#[doc = "Possible values of the field `SLVDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDMAR {
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    DISABLED,
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    ENABLED,
}
impl crate::ToBits<bool> for SLVDMAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVDMAR::DISABLED => false,
            SLVDMAR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVDMA_R = crate::FR<bool, SLVDMAR>;
impl SLVDMA_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDMAR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDMAR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SLVDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDMAW {
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    DISABLED,
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    ENABLED,
}
impl SLVDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDMAW::DISABLED => false,
            SLVDMAW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDMAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. No DMA requests are issued for Slave mode operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDMAW::DISABLED)
    }
    #[doc = "Enabled. DMA requests are issued for I2C slave data transmission and reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDMAW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `AUTOACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOACKR {
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    NORMAL,
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    AUTOMATIC_ACK,
}
impl crate::ToBits<bool> for AUTOACKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AUTOACKR::NORMAL => false,
            AUTOACKR::AUTOMATIC_ACK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AUTOACK_R = crate::FR<bool, AUTOACKR>;
impl AUTOACK_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == AUTOACKR::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC_ACK`"]
    #[inline(always)]
    pub fn is_automatic_ack(&self) -> bool {
        *self == AUTOACKR::AUTOMATIC_ACK
    }
}
#[doc = "Values that can be written to the field `AUTOACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOACKW {
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    NORMAL,
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    AUTOMATIC_ACK,
}
impl AUTOACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOACKW::NORMAL => false,
            AUTOACKW::AUTOMATIC_ACK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AUTOACKW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOACKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal, non-automatic operation. If AUTONACK = 0, an SlvPending interrupt is generated when a matching address is received. If AUTONACK = 1, received addresses are NACKed (ignored)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AUTOACKW::NORMAL)
    }
    #[doc = "A header with matching SLVADR0 and matching direction as set by AUTOMATCHREAD will be ACKed immediately, allowing the master to move on to the data bytes. If the address matches SLVADR0, but the direction does not match AUTOMATCHREAD, the behavior will depend on the AUTONACK bit in the SLVADR0 register: if AUTONACK is set, then it will be Nacked; else if AUTONACK is clear, then a SlvPending interrupt is generated."]
    #[inline(always)]
    pub fn automatic_ack(self) -> &'a mut W {
        self.variant(AUTOACKW::AUTOMATIC_ACK)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `AUTOMATCHREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOMATCHREADR {
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    I2C_WRITE,
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    I2C_READ,
}
impl crate::ToBits<bool> for AUTOMATCHREADR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AUTOMATCHREADR::I2C_WRITE => false,
            AUTOMATCHREADR::I2C_READ => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AUTOMATCHREAD_R = crate::FR<bool, AUTOMATCHREADR>;
impl AUTOMATCHREAD_R {
    #[doc = "Checks if the value of the field is `I2C_WRITE`"]
    #[inline(always)]
    pub fn is_i2c_write(&self) -> bool {
        *self == AUTOMATCHREADR::I2C_WRITE
    }
    #[doc = "Checks if the value of the field is `I2C_READ`"]
    #[inline(always)]
    pub fn is_i2c_read(&self) -> bool {
        *self == AUTOMATCHREADR::I2C_READ
    }
}
#[doc = "Values that can be written to the field `AUTOMATCHREAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOMATCHREADW {
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    I2C_WRITE,
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    I2C_READ,
}
impl AUTOMATCHREADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOMATCHREADW::I2C_WRITE => false,
            AUTOMATCHREADW::I2C_READ => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AUTOMATCHREADW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOMATCHREADW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOMATCHREADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C write."]
    #[inline(always)]
    pub fn i2c_write(self) -> &'a mut W {
        self.variant(AUTOMATCHREADW::I2C_WRITE)
    }
    #[doc = "The expected next operation in Automatic Mode is an I2C read."]
    #[inline(always)]
    pub fn i2c_read(self) -> &'a mut W {
        self.variant(AUTOMATCHREADW::I2C_READ)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&self) -> SLVCONTINUE_R {
        SLVCONTINUE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&self) -> SLVNACK_R {
        SLVNACK_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline(always)]
    pub fn slvdma(&self) -> SLVDMA_R {
        SLVDMA_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub fn automatchread(&self) -> AUTOMATCHREAD_R {
        AUTOMATCHREAD_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Slave Continue."]
    #[inline(always)]
    pub fn slvcontinue(&mut self) -> _SLVCONTINUEW {
        _SLVCONTINUEW { w: self }
    }
    #[doc = "Bit 1 - Slave NACK."]
    #[inline(always)]
    pub fn slvnack(&mut self) -> _SLVNACKW {
        _SLVNACKW { w: self }
    }
    #[doc = "Bit 3 - Slave DMA enable."]
    #[inline(always)]
    pub fn slvdma(&mut self) -> _SLVDMAW {
        _SLVDMAW { w: self }
    }
    #[doc = "Bit 8 - Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub fn autoack(&mut self) -> _AUTOACKW {
        _AUTOACKW { w: self }
    }
    #[doc = "Bit 9 - When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub fn automatchread(&mut self) -> _AUTOMATCHREADW {
        _AUTOMATCHREADW { w: self }
    }
}
