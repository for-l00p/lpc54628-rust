#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDSTS {
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
pub type TI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TIW<'a> {
    w: &'a mut W,
}
impl<'a> _TIW<'a> {
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
pub type RI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RIW<'a> {
    w: &'a mut W,
}
impl<'a> _RIW<'a> {
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
pub type FBE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FBEW<'a> {
    w: &'a mut W,
}
impl<'a> _FBEW<'a> {
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
pub type DU_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DUW<'a> {
    w: &'a mut W,
}
impl<'a> _DUW<'a> {
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
pub type CES_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CESW<'a> {
    w: &'a mut W,
}
impl<'a> _CESW<'a> {
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
pub type NIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NISW<'a> {
    w: &'a mut W,
}
impl<'a> _NISW<'a> {
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
pub type AIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _AISW<'a> {
    w: &'a mut W,
}
impl<'a> _AISW<'a> {
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
pub type EB_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _EBW<'a> {
    w: &'a mut W,
}
impl<'a> _EBW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FSMW<'a> {
    w: &'a mut W,
}
impl<'a> _FSMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Interrupt."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Error Bits."]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits() >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:16 - DMAC state machine present state."]
    #[inline(always)]
    pub fn fsm(&self) -> FSM_R {
        FSM_R::new(((self.bits() >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit Interrupt."]
    #[inline(always)]
    pub fn ti(&mut self) -> _TIW {
        _TIW { w: self }
    }
    #[doc = "Bit 1 - Receive Interrupt."]
    #[inline(always)]
    pub fn ri(&mut self) -> _RIW {
        _RIW { w: self }
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub fn fbe(&mut self) -> _FBEW {
        _FBEW { w: self }
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> _DUW {
        _DUW { w: self }
    }
    #[doc = "Bit 5 - Card Error Summary."]
    #[inline(always)]
    pub fn ces(&mut self) -> _CESW {
        _CESW { w: self }
    }
    #[doc = "Bit 8 - Normal Interrupt Summary."]
    #[inline(always)]
    pub fn nis(&mut self) -> _NISW {
        _NISW { w: self }
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary."]
    #[inline(always)]
    pub fn ais(&mut self) -> _AISW {
        _AISW { w: self }
    }
    #[doc = "Bits 10:12 - Error Bits."]
    #[inline(always)]
    pub fn eb(&mut self) -> _EBW {
        _EBW { w: self }
    }
    #[doc = "Bits 13:16 - DMAC state machine present state."]
    #[inline(always)]
    pub fn fsm(&mut self) -> _FSMW {
        _FSMW { w: self }
    }
}
