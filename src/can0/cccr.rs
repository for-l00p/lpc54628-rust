#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCCR {
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
        0x01
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type INIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INITW<'a> {
    w: &'a mut W,
}
impl<'a> _INITW<'a> {
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
pub type CCE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEW<'a> {
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
pub type ASM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ASMW<'a> {
    w: &'a mut W,
}
impl<'a> _ASMW<'a> {
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
pub type CSA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CSAW<'a> {
    w: &'a mut W,
}
impl<'a> _CSAW<'a> {
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
pub type CSR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CSRW<'a> {
    w: &'a mut W,
}
impl<'a> _CSRW<'a> {
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
pub type MON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MONW<'a> {
    w: &'a mut W,
}
impl<'a> _MONW<'a> {
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
pub type DAR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DARW<'a> {
    w: &'a mut W,
}
impl<'a> _DARW<'a> {
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
pub type TEST_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TESTW<'a> {
    w: &'a mut W,
}
impl<'a> _TESTW<'a> {
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
pub type FDOE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FDOEW<'a> {
    w: &'a mut W,
}
impl<'a> _FDOEW<'a> {
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
pub type BRSE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BRSEW<'a> {
    w: &'a mut W,
}
impl<'a> _BRSEW<'a> {
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
pub type PXHD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PXHDW<'a> {
    w: &'a mut W,
}
impl<'a> _PXHDW<'a> {
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
pub type EFBI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EFBIW<'a> {
    w: &'a mut W,
}
impl<'a> _EFBIW<'a> {
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
pub type TXP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXPW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPW<'a> {
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
pub type NISO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NISOW<'a> {
    w: &'a mut W,
}
impl<'a> _NISOW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Initialization."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configuration change enable."]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Restricted operational mode."]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge."]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request."]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus monitoring mode."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable automatic retransmission."]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test mode enable."]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CAN FD operation enable."]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When CAN FD operation is disabled, this bit is not evaluated."]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protocol exception handling disable."]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Edge filtering during bus integration."]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit pause."]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Non ISO operation."]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Initialization."]
    #[inline(always)]
    pub fn init(&mut self) -> _INITW {
        _INITW { w: self }
    }
    #[doc = "Bit 1 - Configuration change enable."]
    #[inline(always)]
    pub fn cce(&mut self) -> _CCEW {
        _CCEW { w: self }
    }
    #[doc = "Bit 2 - Restricted operational mode."]
    #[inline(always)]
    pub fn asm(&mut self) -> _ASMW {
        _ASMW { w: self }
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge."]
    #[inline(always)]
    pub fn csa(&mut self) -> _CSAW {
        _CSAW { w: self }
    }
    #[doc = "Bit 4 - Clock Stop Request."]
    #[inline(always)]
    pub fn csr(&mut self) -> _CSRW {
        _CSRW { w: self }
    }
    #[doc = "Bit 5 - Bus monitoring mode."]
    #[inline(always)]
    pub fn mon(&mut self) -> _MONW {
        _MONW { w: self }
    }
    #[doc = "Bit 6 - Disable automatic retransmission."]
    #[inline(always)]
    pub fn dar(&mut self) -> _DARW {
        _DARW { w: self }
    }
    #[doc = "Bit 7 - Test mode enable."]
    #[inline(always)]
    pub fn test(&mut self) -> _TESTW {
        _TESTW { w: self }
    }
    #[doc = "Bit 8 - CAN FD operation enable."]
    #[inline(always)]
    pub fn fdoe(&mut self) -> _FDOEW {
        _FDOEW { w: self }
    }
    #[doc = "Bit 9 - When CAN FD operation is disabled, this bit is not evaluated."]
    #[inline(always)]
    pub fn brse(&mut self) -> _BRSEW {
        _BRSEW { w: self }
    }
    #[doc = "Bit 12 - Protocol exception handling disable."]
    #[inline(always)]
    pub fn pxhd(&mut self) -> _PXHDW {
        _PXHDW { w: self }
    }
    #[doc = "Bit 13 - Edge filtering during bus integration."]
    #[inline(always)]
    pub fn efbi(&mut self) -> _EFBIW {
        _EFBIW { w: self }
    }
    #[doc = "Bit 14 - Transmit pause."]
    #[inline(always)]
    pub fn txp(&mut self) -> _TXPW {
        _TXPW { w: self }
    }
    #[doc = "Bit 15 - Non ISO operation."]
    #[inline(always)]
    pub fn niso(&mut self) -> _NISOW {
        _NISOW { w: self }
    }
}
