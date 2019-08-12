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
pub type DATALEN_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _DATALENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATALENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type POLL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _POLLW<'a> {
    w: &'a mut W,
}
impl<'a> _POLLW<'a> {
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
#[doc = "Possible values of the field `DOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUTR {
    #[doc = "Input from serial flash."]
    INPUT,
    #[doc = "Output to serial flash."]
    OUTPUT,
}
impl crate::ToBits<bool> for DOUTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DOUTR::INPUT => false,
            DOUTR::OUTPUT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DOUT_R = crate::FR<bool, DOUTR>;
impl DOUT_R {
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DOUTR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DOUTR::OUTPUT
    }
}
#[doc = "Values that can be written to the field `DOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOUTW {
    #[doc = "Input from serial flash."]
    INPUT,
    #[doc = "Output to serial flash."]
    OUTPUT,
}
impl DOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DOUTW::INPUT => false,
            DOUTW::OUTPUT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _DOUTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input from serial flash."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DOUTW::INPUT)
    }
    #[doc = "Output to serial flash."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DOUTW::OUTPUT)
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
#[doc = r"Reader of the field"]
pub type INTLEN_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _INTLENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTLENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `FIELDFORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDFORMR {
    #[doc = "All serial. All fields of the command are serial."]
    ALL_SERIAL,
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    QUAD_DUAL_DATA,
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    SERIAL_OPCODE,
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    ALL_QUAD_DUAL,
}
impl crate::ToBits<u8> for FIELDFORMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FIELDFORMR::ALL_SERIAL => 0,
            FIELDFORMR::QUAD_DUAL_DATA => 1,
            FIELDFORMR::SERIAL_OPCODE => 2,
            FIELDFORMR::ALL_QUAD_DUAL => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FIELDFORM_R = crate::FR<u8, FIELDFORMR>;
impl FIELDFORM_R {
    #[doc = "Checks if the value of the field is `ALL_SERIAL`"]
    #[inline(always)]
    pub fn is_all_serial(&self) -> bool {
        *self == FIELDFORMR::ALL_SERIAL
    }
    #[doc = "Checks if the value of the field is `QUAD_DUAL_DATA`"]
    #[inline(always)]
    pub fn is_quad_dual_data(&self) -> bool {
        *self == FIELDFORMR::QUAD_DUAL_DATA
    }
    #[doc = "Checks if the value of the field is `SERIAL_OPCODE`"]
    #[inline(always)]
    pub fn is_serial_opcode(&self) -> bool {
        *self == FIELDFORMR::SERIAL_OPCODE
    }
    #[doc = "Checks if the value of the field is `ALL_QUAD_DUAL`"]
    #[inline(always)]
    pub fn is_all_quad_dual(&self) -> bool {
        *self == FIELDFORMR::ALL_QUAD_DUAL
    }
}
#[doc = "Values that can be written to the field `FIELDFORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDFORMW {
    #[doc = "All serial. All fields of the command are serial."]
    ALL_SERIAL,
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    QUAD_DUAL_DATA,
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    SERIAL_OPCODE,
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    ALL_QUAD_DUAL,
}
impl FIELDFORMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FIELDFORMW::ALL_SERIAL => 0,
            FIELDFORMW::QUAD_DUAL_DATA => 1,
            FIELDFORMW::SERIAL_OPCODE => 2,
            FIELDFORMW::ALL_QUAD_DUAL => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FIELDFORMW<'a> {
    w: &'a mut W,
}
impl<'a> _FIELDFORMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDFORMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "All serial. All fields of the command are serial."]
    #[inline(always)]
    pub fn all_serial(self) -> &'a mut W {
        self.variant(FIELDFORMW::ALL_SERIAL)
    }
    #[doc = "Quad/dual data. Data field is quad/dual, other fields are serial."]
    #[inline(always)]
    pub fn quad_dual_data(self) -> &'a mut W {
        self.variant(FIELDFORMW::QUAD_DUAL_DATA)
    }
    #[doc = "Serial opcode. Opcode field is serial. Other fields are quad/dual."]
    #[inline(always)]
    pub fn serial_opcode(self) -> &'a mut W {
        self.variant(FIELDFORMW::SERIAL_OPCODE)
    }
    #[doc = "All quad/dual. All fields of the command are in quad/dual format."]
    #[inline(always)]
    pub fn all_quad_dual(self) -> &'a mut W {
        self.variant(FIELDFORMW::ALL_QUAD_DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `FRAMEFORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMEFORMR {
    #[doc = "Opcode. Opcode only, no address."]
    OPCODE,
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    OPCODE_1_BYTE,
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    OPCODE_2_BYTES,
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    OPCODE_3_BYTES,
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_4_BYTES,
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    NO_OPCODE_3_BYTES,
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    NO_OPCODE_4_BYTES,
}
impl crate::ToBits<u8> for FRAMEFORMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FRAMEFORMR::OPCODE => 1,
            FRAMEFORMR::OPCODE_1_BYTE => 2,
            FRAMEFORMR::OPCODE_2_BYTES => 3,
            FRAMEFORMR::OPCODE_3_BYTES => 4,
            FRAMEFORMR::OPCODE_4_BYTES => 5,
            FRAMEFORMR::NO_OPCODE_3_BYTES => 6,
            FRAMEFORMR::NO_OPCODE_4_BYTES => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FRAMEFORM_R = crate::FR<u8, FRAMEFORMR>;
impl FRAMEFORM_R {
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline(always)]
    pub fn is_opcode(&self) -> bool {
        *self == FRAMEFORMR::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPCODE_1_BYTE`"]
    #[inline(always)]
    pub fn is_opcode_1_byte(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_1_BYTE
    }
    #[doc = "Checks if the value of the field is `OPCODE_2_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_2_bytes(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_2_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_3_bytes(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_opcode_4_bytes(&self) -> bool {
        *self == FRAMEFORMR::OPCODE_4_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_3_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_3_bytes(&self) -> bool {
        *self == FRAMEFORMR::NO_OPCODE_3_BYTES
    }
    #[doc = "Checks if the value of the field is `NO_OPCODE_4_BYTES`"]
    #[inline(always)]
    pub fn is_no_opcode_4_bytes(&self) -> bool {
        *self == FRAMEFORMR::NO_OPCODE_4_BYTES
    }
}
#[doc = "Values that can be written to the field `FRAMEFORM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMEFORMW {
    #[doc = "Opcode. Opcode only, no address."]
    OPCODE,
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    OPCODE_1_BYTE,
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    OPCODE_2_BYTES,
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    OPCODE_3_BYTES,
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    OPCODE_4_BYTES,
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    NO_OPCODE_3_BYTES,
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    NO_OPCODE_4_BYTES,
}
impl FRAMEFORMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRAMEFORMW::OPCODE => 1,
            FRAMEFORMW::OPCODE_1_BYTE => 2,
            FRAMEFORMW::OPCODE_2_BYTES => 3,
            FRAMEFORMW::OPCODE_3_BYTES => 4,
            FRAMEFORMW::OPCODE_4_BYTES => 5,
            FRAMEFORMW::NO_OPCODE_3_BYTES => 6,
            FRAMEFORMW::NO_OPCODE_4_BYTES => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FRAMEFORMW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMEFORMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMEFORMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Opcode. Opcode only, no address."]
    #[inline(always)]
    pub fn opcode(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE)
    }
    #[doc = "Opcode one byte. Opcode, least significant byte of address."]
    #[inline(always)]
    pub fn opcode_1_byte(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_1_BYTE)
    }
    #[doc = "Opcode two bytes. Opcode, two least significant bytes of address."]
    #[inline(always)]
    pub fn opcode_2_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_2_BYTES)
    }
    #[doc = "Opcode three bytes. Opcode, three least significant bytes of address."]
    #[inline(always)]
    pub fn opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_3_BYTES)
    }
    #[doc = "Opcode four bytes. Opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::OPCODE_4_BYTES)
    }
    #[doc = "No opcode three bytes. No opcode, 3 least significant bytes of address."]
    #[inline(always)]
    pub fn no_opcode_3_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::NO_OPCODE_3_BYTES)
    }
    #[doc = "No opcode four bytes. No opcode, 4 bytes of address."]
    #[inline(always)]
    pub fn no_opcode_4_bytes(self) -> &'a mut W {
        self.variant(FRAMEFORMW::NO_OPCODE_4_BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type OPCODE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _OPCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OPCODEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:13 - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new((self.bits() & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the DATALEN field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
    #[inline(always)]
    pub fn poll(&self) -> POLL_R {
        POLL_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If the DATALEN field is not zero, this bit controls the direction of the data:"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline(always)]
    pub fn intlen(&self) -> INTLEN_R {
        INTLEN_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline(always)]
    pub fn fieldform(&self) -> FIELDFORM_R {
        FIELDFORM_R::new(((self.bits() >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline(always)]
    pub fn frameform(&self) -> FRAMEFORM_R {
        FRAMEFORM_R::new(((self.bits() >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:13 - Except when the POLL bit in this register is 1, this field controls how many data bytes are in the command. 0 indicates that the command does not contain a data field."]
    #[inline(always)]
    pub fn datalen(&mut self) -> _DATALENW {
        _DATALENW { w: self }
    }
    #[doc = "Bit 14 - This bit should be written as 1 only with an opcode that a) contains an input data field, and b) causes the serial flash device to return byte status repetitively (e.g., a Read Status command). When this bit is 1, the SPIFI hardware continues to read bytes until the test specified by the DATALEN field is met. The hardware tests the bit in each status byte selected by DATALEN bits 2:0, until a bit is found that is equal to DATALEN bit 3. When the test succeeds, the SPIFI captures the byte that meets this test so that it can be read from the Data Register, and terminates the command by raising CS. The end-of-command interrupt can be enabled to inform software when this occurs"]
    #[inline(always)]
    pub fn poll(&mut self) -> _POLLW {
        _POLLW { w: self }
    }
    #[doc = "Bit 15 - If the DATALEN field is not zero, this bit controls the direction of the data:"]
    #[inline(always)]
    pub fn dout(&mut self) -> _DOUTW {
        _DOUTW { w: self }
    }
    #[doc = "Bits 16:18 - This field controls how many intermediate bytes precede the data. (Each such byte may require 8 or 2 SCK cycles, depending on whether the intermediate field is in serial, 2-bit, or 4-bit format.) Intermediate bytes are output by the SPIFI, and include post-address control information, dummy and delay bytes. See the description of the Intermediate Data register for the contents of such bytes."]
    #[inline(always)]
    pub fn intlen(&mut self) -> _INTLENW {
        _INTLENW { w: self }
    }
    #[doc = "Bits 19:20 - This field controls how the fields of the command are sent."]
    #[inline(always)]
    pub fn fieldform(&mut self) -> _FIELDFORMW {
        _FIELDFORMW { w: self }
    }
    #[doc = "Bits 21:23 - This field controls the opcode and address fields."]
    #[inline(always)]
    pub fn frameform(&mut self) -> _FRAMEFORMW {
        _FRAMEFORMW { w: self }
    }
    #[doc = "Bits 24:31 - The opcode of the command (not used for some FRAMEFORM values)."]
    #[inline(always)]
    pub fn opcode(&mut self) -> _OPCODEW {
        _OPCODEW { w: self }
    }
}
