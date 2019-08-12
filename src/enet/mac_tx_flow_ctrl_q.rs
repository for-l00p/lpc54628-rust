#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_TX_FLOW_CTRL_Q {
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
pub type FCB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FCBW<'a> {
    w: &'a mut W,
}
impl<'a> _FCBW<'a> {
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
pub type TFE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TFEW<'a> {
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
pub type PLT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PLTW<'a> {
    w: &'a mut W,
}
impl<'a> _PLTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DZPQ_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DZPQW<'a> {
    w: &'a mut W,
}
impl<'a> _DZPQW<'a> {
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
pub type PT_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _PTW<'a> {
    w: &'a mut W,
}
impl<'a> _PTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
    #[inline(always)]
    pub fn fcb(&mut self) -> _FCBW {
        _FCBW { w: self }
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
    #[inline(always)]
    pub fn tfe(&mut self) -> _TFEW {
        _TFEW { w: self }
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
    #[inline(always)]
    pub fn plt(&mut self) -> _PLTW {
        _PLTW { w: self }
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
    #[inline(always)]
    pub fn dzpq(&mut self) -> _DZPQW {
        _DZPQW { w: self }
    }
    #[doc = "Bits 16:31 - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
    #[inline(always)]
    pub fn pt(&mut self) -> _PTW {
        _PTW { w: self }
    }
}
