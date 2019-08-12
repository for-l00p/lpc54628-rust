#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
        0x0801
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `MSTPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPENDINGR {
    #[doc = "In progress. Communication is in progress and the Master function is busy and cannot currently accept a command."]
    IN_PROGRESS,
    #[doc = "Pending. The Master function needs software service or is in the idle state. If the master is not in the idle state, it is waiting to receive or transmit data or the NACK bit."]
    PENDING,
}
impl crate::ToBits<bool> for MSTPENDINGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTPENDINGR::IN_PROGRESS => false,
            MSTPENDINGR::PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTPENDING_R = crate::FR<bool, MSTPENDINGR>;
impl MSTPENDING_R {
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == MSTPENDINGR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MSTPENDINGR::PENDING
    }
}
#[doc = "Possible values of the field `MSTSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTATER {
    #[doc = "Idle. The Master function is available to be used for a new transaction."]
    IDLE,
    #[doc = "Receive ready. Received data available (Master Receiver mode). Address plus Read was previously sent and Acknowledged by slave."]
    RECEIVE_READY,
    #[doc = "Transmit ready. Data can be transmitted (Master Transmitter mode). Address plus Write was previously sent and Acknowledged by slave."]
    TRANSMIT_READY,
    #[doc = "NACK Address. Slave NACKed address."]
    NACK_ADDRESS,
    #[doc = "NACK Data. Slave NACKed transmitted data."]
    NACK_DATA,
}
impl crate::ToBits<u8> for MSTSTATER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MSTSTATER::IDLE => 0,
            MSTSTATER::RECEIVE_READY => 1,
            MSTSTATER::TRANSMIT_READY => 2,
            MSTSTATER::NACK_ADDRESS => 3,
            MSTSTATER::NACK_DATA => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTSTATE_R = crate::FR<u8, MSTSTATER>;
impl MSTSTATE_R {
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MSTSTATER::IDLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE_READY`"]
    #[inline(always)]
    pub fn is_receive_ready(&self) -> bool {
        *self == MSTSTATER::RECEIVE_READY
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_READY`"]
    #[inline(always)]
    pub fn is_transmit_ready(&self) -> bool {
        *self == MSTSTATER::TRANSMIT_READY
    }
    #[doc = "Checks if the value of the field is `NACK_ADDRESS`"]
    #[inline(always)]
    pub fn is_nack_address(&self) -> bool {
        *self == MSTSTATER::NACK_ADDRESS
    }
    #[doc = "Checks if the value of the field is `NACK_DATA`"]
    #[inline(always)]
    pub fn is_nack_data(&self) -> bool {
        *self == MSTSTATER::NACK_DATA
    }
}
#[doc = "Possible values of the field `MSTARBLOSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSR {
    #[doc = "No Arbitration Loss has occurred."]
    NO_LOSS,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS,
}
impl crate::ToBits<bool> for MSTARBLOSSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSR::NO_LOSS => false,
            MSTARBLOSSR::ARBITRATION_LOSS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTARBLOSS_R = crate::FR<bool, MSTARBLOSSR>;
impl MSTARBLOSS_R {
    #[doc = "Checks if the value of the field is `NO_LOSS`"]
    #[inline(always)]
    pub fn is_no_loss(&self) -> bool {
        *self == MSTARBLOSSR::NO_LOSS
    }
    #[doc = "Checks if the value of the field is `ARBITRATION_LOSS`"]
    #[inline(always)]
    pub fn is_arbitration_loss(&self) -> bool {
        *self == MSTARBLOSSR::ARBITRATION_LOSS
    }
}
#[doc = "Values that can be written to the field `MSTARBLOSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTARBLOSSW {
    #[doc = "No Arbitration Loss has occurred."]
    NO_LOSS,
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    ARBITRATION_LOSS,
}
impl MSTARBLOSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTARBLOSSW::NO_LOSS => false,
            MSTARBLOSSW::ARBITRATION_LOSS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTARBLOSSW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTARBLOSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Arbitration Loss has occurred."]
    #[inline(always)]
    pub fn no_loss(self) -> &'a mut W {
        self.variant(MSTARBLOSSW::NO_LOSS)
    }
    #[doc = "Arbitration loss. The Master function has experienced an Arbitration Loss. At this point, the Master function has already stopped driving the bus and gone to an idle state. Software can respond by doing nothing, or by sending a Start in order to attempt to gain control of the bus when it next becomes idle."]
    #[inline(always)]
    pub fn arbitration_loss(self) -> &'a mut W {
        self.variant(MSTARBLOSSW::ARBITRATION_LOSS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `MSTSTSTPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRR {
    #[doc = "No Start/Stop Error has occurred."]
    NO_ERROR,
    #[doc = "The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    ERROR,
}
impl crate::ToBits<bool> for MSTSTSTPERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRR::NO_ERROR => false,
            MSTSTSTPERRR::ERROR => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MSTSTSTPERR_R = crate::FR<bool, MSTSTSTPERRR>;
impl MSTSTSTPERR_R {
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MSTSTSTPERRR::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MSTSTSTPERRR::ERROR
    }
}
#[doc = "Values that can be written to the field `MSTSTSTPERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTSTSTPERRW {
    #[doc = "No Start/Stop Error has occurred."]
    NO_ERROR,
    #[doc = "The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    ERROR,
}
impl MSTSTSTPERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTSTSTPERRW::NO_ERROR => false,
            MSTSTSTPERRW::ERROR => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MSTSTSTPERRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTSTSTPERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Start/Stop Error has occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(MSTSTSTPERRW::NO_ERROR)
    }
    #[doc = "The Master function has experienced a Start/Stop Error. A Start or Stop was detected at a time when it is not allowed by the I2C specification. The Master interface has stopped driving the bus and gone to an idle state, no action is required. A request for a Start could be made, or software could attempt to insure that the bus has not stalled."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(MSTSTSTPERRW::ERROR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `SLVPENDING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVPENDINGR {
    #[doc = "In progress. The Slave function does not currently need service."]
    IN_PROGRESS,
    #[doc = "Pending. The Slave function needs service. Information on what is needed can be found in the adjacent SLVSTATE field."]
    PENDING,
}
impl crate::ToBits<bool> for SLVPENDINGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVPENDINGR::IN_PROGRESS => false,
            SLVPENDINGR::PENDING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVPENDING_R = crate::FR<bool, SLVPENDINGR>;
impl SLVPENDING_R {
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == SLVPENDINGR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SLVPENDINGR::PENDING
    }
}
#[doc = "Possible values of the field `SLVSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSTATER {
    #[doc = "Slave address. Address plus R/W received. At least one of the four slave addresses has been matched by hardware."]
    SLAVE_ADDRESS,
    #[doc = "Slave receive. Received data is available (Slave Receiver mode)."]
    SLAVE_RECEIVE,
    #[doc = "Slave transmit. Data can be transmitted (Slave Transmitter mode)."]
    SLAVE_TRANSMIT,
}
impl crate::ToBits<u8> for SLVSTATER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SLVSTATER::SLAVE_ADDRESS => 0,
            SLVSTATER::SLAVE_RECEIVE => 1,
            SLVSTATER::SLAVE_TRANSMIT => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVSTATE_R = crate::FR<u8, SLVSTATER>;
impl SLVSTATE_R {
    #[doc = "Checks if the value of the field is `SLAVE_ADDRESS`"]
    #[inline(always)]
    pub fn is_slave_address(&self) -> bool {
        *self == SLVSTATER::SLAVE_ADDRESS
    }
    #[doc = "Checks if the value of the field is `SLAVE_RECEIVE`"]
    #[inline(always)]
    pub fn is_slave_receive(&self) -> bool {
        *self == SLVSTATER::SLAVE_RECEIVE
    }
    #[doc = "Checks if the value of the field is `SLAVE_TRANSMIT`"]
    #[inline(always)]
    pub fn is_slave_transmit(&self) -> bool {
        *self == SLVSTATER::SLAVE_TRANSMIT
    }
}
#[doc = "Possible values of the field `SLVNOTSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVNOTSTRR {
    #[doc = "Stretching. The slave function is currently stretching the I2C bus clock. Deep-Sleep or Power-down mode cannot be entered at this time."]
    STRETCHING,
    #[doc = "Not stretching. The slave function is not currently stretching the I 2C bus clock. Deep-sleep or Power-down mode could be entered at this time."]
    NOT_STRETCHING,
}
impl crate::ToBits<bool> for SLVNOTSTRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVNOTSTRR::STRETCHING => false,
            SLVNOTSTRR::NOT_STRETCHING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVNOTSTR_R = crate::FR<bool, SLVNOTSTRR>;
impl SLVNOTSTR_R {
    #[doc = "Checks if the value of the field is `STRETCHING`"]
    #[inline(always)]
    pub fn is_stretching(&self) -> bool {
        *self == SLVNOTSTRR::STRETCHING
    }
    #[doc = "Checks if the value of the field is `NOT_STRETCHING`"]
    #[inline(always)]
    pub fn is_not_stretching(&self) -> bool {
        *self == SLVNOTSTRR::NOT_STRETCHING
    }
}
#[doc = "Possible values of the field `SLVIDX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVIDXR {
    #[doc = "Address 0. Slave address 0 was matched."]
    ADDRESS0,
    #[doc = "Address 1. Slave address 1 was matched."]
    ADDRESS1,
    #[doc = "Address 2. Slave address 2 was matched."]
    ADDRESS2,
    #[doc = "Address 3. Slave address 3 was matched."]
    ADDRESS3,
}
impl crate::ToBits<u8> for SLVIDXR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SLVIDXR::ADDRESS0 => 0,
            SLVIDXR::ADDRESS1 => 1,
            SLVIDXR::ADDRESS2 => 2,
            SLVIDXR::ADDRESS3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVIDX_R = crate::FR<u8, SLVIDXR>;
impl SLVIDX_R {
    #[doc = "Checks if the value of the field is `ADDRESS0`"]
    #[inline(always)]
    pub fn is_address0(&self) -> bool {
        *self == SLVIDXR::ADDRESS0
    }
    #[doc = "Checks if the value of the field is `ADDRESS1`"]
    #[inline(always)]
    pub fn is_address1(&self) -> bool {
        *self == SLVIDXR::ADDRESS1
    }
    #[doc = "Checks if the value of the field is `ADDRESS2`"]
    #[inline(always)]
    pub fn is_address2(&self) -> bool {
        *self == SLVIDXR::ADDRESS2
    }
    #[doc = "Checks if the value of the field is `ADDRESS3`"]
    #[inline(always)]
    pub fn is_address3(&self) -> bool {
        *self == SLVIDXR::ADDRESS3
    }
}
#[doc = "Possible values of the field `SLVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVSELR {
    #[doc = "Not selected. The Slave function is not currently selected."]
    NOT_SELECTED,
    #[doc = "Selected. The Slave function is currently selected."]
    SELECTED,
}
impl crate::ToBits<bool> for SLVSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVSELR::NOT_SELECTED => false,
            SLVSELR::SELECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVSEL_R = crate::FR<bool, SLVSELR>;
impl SLVSEL_R {
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == SLVSELR::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == SLVSELR::SELECTED
    }
}
#[doc = "Possible values of the field `SLVDESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELR {
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED,
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED,
}
impl crate::ToBits<bool> for SLVDESELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLVDESELR::NOT_DESELECTED => false,
            SLVDESELR::DESELECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLVDESEL_R = crate::FR<bool, SLVDESELR>;
impl SLVDESEL_R {
    #[doc = "Checks if the value of the field is `NOT_DESELECTED`"]
    #[inline(always)]
    pub fn is_not_deselected(&self) -> bool {
        *self == SLVDESELR::NOT_DESELECTED
    }
    #[doc = "Checks if the value of the field is `DESELECTED`"]
    #[inline(always)]
    pub fn is_deselected(&self) -> bool {
        *self == SLVDESELR::DESELECTED
    }
}
#[doc = "Values that can be written to the field `SLVDESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDESELW {
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    NOT_DESELECTED,
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    DESELECTED,
}
impl SLVDESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDESELW::NOT_DESELECTED => false,
            SLVDESELW::DESELECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLVDESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVDESELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not deselected. The Slave function has not become deselected. This does not mean that it is currently selected. That information can be found in the SLVSEL flag."]
    #[inline(always)]
    pub fn not_deselected(self) -> &'a mut W {
        self.variant(SLVDESELW::NOT_DESELECTED)
    }
    #[doc = "Deselected. The Slave function has become deselected. This is specifically caused by the SLVSEL flag changing from 1 to 0. See the description of SLVSEL for details on when that event occurs."]
    #[inline(always)]
    pub fn deselected(self) -> &'a mut W {
        self.variant(SLVDESELW::DESELECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Possible values of the field `MONRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDYR {
    #[doc = "No data. The Monitor function does not currently have data available."]
    NO_DATA,
    #[doc = "Data waiting. The Monitor function has data waiting to be read."]
    DATA_WAITING,
}
impl crate::ToBits<bool> for MONRDYR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONRDYR::NO_DATA => false,
            MONRDYR::DATA_WAITING => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONRDY_R = crate::FR<bool, MONRDYR>;
impl MONRDY_R {
    #[doc = "Checks if the value of the field is `NO_DATA`"]
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == MONRDYR::NO_DATA
    }
    #[doc = "Checks if the value of the field is `DATA_WAITING`"]
    #[inline(always)]
    pub fn is_data_waiting(&self) -> bool {
        *self == MONRDYR::DATA_WAITING
    }
}
#[doc = "Possible values of the field `MONOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVR {
    #[doc = "No overrun. Monitor data has not overrun."]
    NO_OVERRUN,
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN,
}
impl crate::ToBits<bool> for MONOVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONOVR::NO_OVERRUN => false,
            MONOVR::OVERRUN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONOV_R = crate::FR<bool, MONOVR>;
impl MONOV_R {
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == MONOVR::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == MONOVR::OVERRUN
    }
}
#[doc = "Values that can be written to the field `MONOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONOVW {
    #[doc = "No overrun. Monitor data has not overrun."]
    NO_OVERRUN,
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    OVERRUN,
}
impl MONOVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONOVW::NO_OVERRUN => false,
            MONOVW::OVERRUN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONOVW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONOVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun. Monitor data has not overrun."]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(MONOVW::NO_OVERRUN)
    }
    #[doc = "Overrun. A Monitor data overrun has occurred. This can only happen when Monitor clock stretching not enabled via the MONCLKSTR bit in the CFG register. Writing 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(MONOVW::OVERRUN)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `MONACTIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONACTIVER {
    #[doc = "Inactive. The Monitor function considers the I2C bus to be inactive."]
    INACTIVE,
    #[doc = "Active. The Monitor function considers the I2C bus to be active."]
    ACTIVE,
}
impl crate::ToBits<bool> for MONACTIVER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONACTIVER::INACTIVE => false,
            MONACTIVER::ACTIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONACTIVE_R = crate::FR<bool, MONACTIVER>;
impl MONACTIVE_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MONACTIVER::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == MONACTIVER::ACTIVE
    }
}
#[doc = "Possible values of the field `MONIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLER {
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE,
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE,
}
impl crate::ToBits<bool> for MONIDLER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MONIDLER::NOT_IDLE => false,
            MONIDLER::IDLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MONIDLE_R = crate::FR<bool, MONIDLER>;
impl MONIDLE_R {
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == MONIDLER::NOT_IDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == MONIDLER::IDLE
    }
}
#[doc = "Values that can be written to the field `MONIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONIDLEW {
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    NOT_IDLE,
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    IDLE,
}
impl MONIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MONIDLEW::NOT_IDLE => false,
            MONIDLEW::IDLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MONIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not idle. The I2C bus is not idle, or this flag has been cleared by software."]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut W {
        self.variant(MONIDLEW::NOT_IDLE)
    }
    #[doc = "Idle. The I2C bus has gone idle at least once since the last time this flag was cleared by software."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(MONIDLEW::IDLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `EVENTTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTR {
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    NO_TIMEOUT,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    EVEN_TIMEOUT,
}
impl crate::ToBits<bool> for EVENTTIMEOUTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTR::NO_TIMEOUT => false,
            EVENTTIMEOUTR::EVEN_TIMEOUT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EVENTTIMEOUT_R = crate::FR<bool, EVENTTIMEOUTR>;
impl EVENTTIMEOUT_R {
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == EVENTTIMEOUTR::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `EVEN_TIMEOUT`"]
    #[inline(always)]
    pub fn is_even_timeout(&self) -> bool {
        *self == EVENTTIMEOUTR::EVEN_TIMEOUT
    }
}
#[doc = "Values that can be written to the field `EVENTTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTTIMEOUTW {
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    NO_TIMEOUT,
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    EVEN_TIMEOUT,
}
impl EVENTTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EVENTTIMEOUTW::NO_TIMEOUT => false,
            EVENTTIMEOUTW::EVEN_TIMEOUT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EVENTTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTTIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. I2C bus events have not caused a time-out."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTW::NO_TIMEOUT)
    }
    #[doc = "Event time-out. The time between I2C bus events has been longer than the time specified by the TIMEOUT register."]
    #[inline(always)]
    pub fn even_timeout(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTW::EVEN_TIMEOUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `SCLTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTR {
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    NO_TIMEOUT,
    #[doc = "Time-out. SCL low time has caused a time-out."]
    TIMEOUT,
}
impl crate::ToBits<bool> for SCLTIMEOUTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTR::NO_TIMEOUT => false,
            SCLTIMEOUTR::TIMEOUT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SCLTIMEOUT_R = crate::FR<bool, SCLTIMEOUTR>;
impl SCLTIMEOUT_R {
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == SCLTIMEOUTR::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == SCLTIMEOUTR::TIMEOUT
    }
}
#[doc = "Values that can be written to the field `SCLTIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLTIMEOUTW {
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    NO_TIMEOUT,
    #[doc = "Time-out. SCL low time has caused a time-out."]
    TIMEOUT,
}
impl SCLTIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLTIMEOUTW::NO_TIMEOUT => false,
            SCLTIMEOUTW::TIMEOUT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SCLTIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLTIMEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. SCL low time has not caused a time-out."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(SCLTIMEOUTW::NO_TIMEOUT)
    }
    #[doc = "Time-out. SCL low time has caused a time-out."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(SCLTIMEOUTW::TIMEOUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[inline(always)]
    pub fn mststate(&self) -> MSTSTATE_R {
        MSTSTATE_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[inline(always)]
    pub fn slvstate(&self) -> SLVSTATE_R {
        SLVSTATE_R::new(((self.bits() >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline(always)]
    pub fn slvidx(&self) -> SLVIDX_R {
        SLVIDX_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[inline(always)]
    pub fn slvsel(&self) -> SLVSEL_R {
        SLVSEL_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline(always)]
    pub fn monactive(&self) -> MONACTIVE_R {
        MONACTIVE_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstarbloss(&mut self) -> _MSTARBLOSSW {
        _MSTARBLOSSW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn mstststperr(&mut self) -> _MSTSTSTPERRW {
        _MSTSTSTPERRW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn slvdesel(&mut self) -> _SLVDESELW {
        _SLVDESELW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&mut self) -> _MONOVW {
        _MONOVW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn monidle(&mut self) -> _MONIDLEW {
        _MONIDLEW { w: self }
    }
    #[doc = "Bit 24 - Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub fn eventtimeout(&mut self) -> _EVENTTIMEOUTW {
        _EVENTTIMEOUTW { w: self }
    }
    #[doc = "Bit 25 - SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn scltimeout(&mut self) -> _SCLTIMEOUTW {
        _SCLTIMEOUTW { w: self }
    }
}
