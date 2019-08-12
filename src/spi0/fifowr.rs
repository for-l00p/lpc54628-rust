#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOWR {
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
pub type TXDATA_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TXDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDATAW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL0_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL0_NW {
    #[doc = "SSEL0 asserted."]
    ASSERTED,
    #[doc = "SSEL0 not asserted."]
    NOT_ASSERTED,
}
impl TXSSEL0_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL0_NW::ASSERTED => false,
            TXSSEL0_NW::NOT_ASSERTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXSSEL0_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL0_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL0_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL0 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_NW::ASSERTED)
    }
    #[doc = "SSEL0 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_NW::NOT_ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL1_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL1_NW {
    #[doc = "SSEL1 asserted."]
    ASSERTED,
    #[doc = "SSEL1 not asserted."]
    NOT_ASSERTED,
}
impl TXSSEL1_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL1_NW::ASSERTED => false,
            TXSSEL1_NW::NOT_ASSERTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXSSEL1_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL1_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL1_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL1 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_NW::ASSERTED)
    }
    #[doc = "SSEL1 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_NW::NOT_ASSERTED)
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
#[doc = "Values that can be written to the field `TXSSEL2_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL2_NW {
    #[doc = "SSEL2 asserted."]
    ASSERTED,
    #[doc = "SSEL2 not asserted."]
    NOT_ASSERTED,
}
impl TXSSEL2_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL2_NW::ASSERTED => false,
            TXSSEL2_NW::NOT_ASSERTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXSSEL2_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL2_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL2_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL2 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_NW::ASSERTED)
    }
    #[doc = "SSEL2 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_NW::NOT_ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL3_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL3_NW {
    #[doc = "SSEL3 asserted."]
    ASSERTED,
    #[doc = "SSEL3 not asserted."]
    NOT_ASSERTED,
}
impl TXSSEL3_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL3_NW::ASSERTED => false,
            TXSSEL3_NW::NOT_ASSERTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TXSSEL3_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL3_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL3_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL3 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_NW::ASSERTED)
    }
    #[doc = "SSEL3 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_NW::NOT_ASSERTED)
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
#[doc = "Values that can be written to the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTW {
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    NOT_DEASSERTED,
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    DEASSERTED,
}
impl EOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EOTW::NOT_DEASSERTED => false,
            EOTW::DEASSERTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EOTW<'a> {
    w: &'a mut W,
}
impl<'a> _EOTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline(always)]
    pub fn not_deasserted(self) -> &'a mut W {
        self.variant(EOTW::NOT_DEASSERTED)
    }
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(EOTW::DEASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Values that can be written to the field `EOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOFW {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    NOT_EOF,
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    EOF,
}
impl EOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            EOFW::NOT_EOF => false,
            EOFW::EOF => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EOFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    #[inline(always)]
    pub fn not_eof(self) -> &'a mut W {
        self.variant(EOFW::NOT_EOF)
    }
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    #[inline(always)]
    pub fn eof(self) -> &'a mut W {
        self.variant(EOFW::EOF)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIGNORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNOREW {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE,
}
impl RXIGNOREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIGNOREW::READ => false,
            RXIGNOREW::IGNORE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RXIGNOREW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIGNOREW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIGNOREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RXIGNOREW::READ)
    }
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RXIGNOREW::IGNORE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _LENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Transmit data to the FIFO."]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits() & 0xffff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Transmit data to the FIFO."]
    #[inline(always)]
    pub fn txdata(&mut self) -> _TXDATAW {
        _TXDATAW { w: self }
    }
    #[doc = "Bit 16 - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> _TXSSEL0_NW {
        _TXSSEL0_NW { w: self }
    }
    #[doc = "Bit 17 - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel1_n(&mut self) -> _TXSSEL1_NW {
        _TXSSEL1_NW { w: self }
    }
    #[doc = "Bit 18 - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel2_n(&mut self) -> _TXSSEL2_NW {
        _TXSSEL2_NW { w: self }
    }
    #[doc = "Bit 19 - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel3_n(&mut self) -> _TXSSEL3_NW {
        _TXSSEL3_NW { w: self }
    }
    #[doc = "Bit 20 - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub fn eot(&mut self) -> _EOTW {
        _EOTW { w: self }
    }
    #[doc = "Bit 21 - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub fn eof(&mut self) -> _EOFW {
        _EOFW { w: self }
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub fn rxignore(&mut self) -> _RXIGNOREW {
        _RXIGNOREW { w: self }
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub fn len(&mut self) -> _LENW {
        _LENW { w: self }
    }
}
