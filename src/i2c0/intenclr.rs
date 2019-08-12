#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR {
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
#[doc = r"Proxy"]
pub struct _MSTPENDINGCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTPENDINGCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _MSTARBLOSSCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTARBLOSSCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _MSTSTSTPERRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTSTSTPERRCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _SLVPENDINGCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVPENDINGCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _SLVNOTSTRCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVNOTSTRCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _SLVDESELCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDESELCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _MONRDYCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MONRDYCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _MONOVCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MONOVCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _MONIDLECLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MONIDLECLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _EVENTTIMEOUTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _EVENTTIMEOUTCLRW<'a> {
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
#[doc = r"Proxy"]
pub struct _SCLTIMEOUTCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLTIMEOUTCLRW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub fn mstpendingclr(&mut self) -> _MSTPENDINGCLRW {
        _MSTPENDINGCLRW { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub fn mstarblossclr(&mut self) -> _MSTARBLOSSCLRW {
        _MSTARBLOSSCLRW { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub fn mstststperrclr(&mut self) -> _MSTSTSTPERRCLRW {
        _MSTSTSTPERRCLRW { w: self }
    }
    #[doc = "Bit 8 - Slave Pending interrupt clear."]
    #[inline(always)]
    pub fn slvpendingclr(&mut self) -> _SLVPENDINGCLRW {
        _SLVPENDINGCLRW { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub fn slvnotstrclr(&mut self) -> _SLVNOTSTRCLRW {
        _SLVNOTSTRCLRW { w: self }
    }
    #[doc = "Bit 15 - Slave Deselect interrupt clear."]
    #[inline(always)]
    pub fn slvdeselclr(&mut self) -> _SLVDESELCLRW {
        _SLVDESELCLRW { w: self }
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub fn monrdyclr(&mut self) -> _MONRDYCLRW {
        _MONRDYCLRW { w: self }
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub fn monovclr(&mut self) -> _MONOVCLRW {
        _MONOVCLRW { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle interrupt clear."]
    #[inline(always)]
    pub fn monidleclr(&mut self) -> _MONIDLECLRW {
        _MONIDLECLRW { w: self }
    }
    #[doc = "Bit 24 - Event time-out interrupt clear."]
    #[inline(always)]
    pub fn eventtimeoutclr(&mut self) -> _EVENTTIMEOUTCLRW {
        _EVENTTIMEOUTCLRW { w: self }
    }
    #[doc = "Bit 25 - SCL time-out interrupt clear."]
    #[inline(always)]
    pub fn scltimeoutclr(&mut self) -> _SCLTIMEOUTCLRW {
        _SCLTIMEOUTCLRW { w: self }
    }
}
