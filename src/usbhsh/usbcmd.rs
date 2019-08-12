#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCMD {
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
pub type RS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSW<'a> {
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
pub type HCRESET_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HCRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _HCRESETW<'a> {
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
pub type FLS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FLSW<'a> {
    w: &'a mut W,
}
impl<'a> _FLSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LHCR_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LHCRW<'a> {
    w: &'a mut W,
}
impl<'a> _LHCRW<'a> {
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
pub type ATL_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ATL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ATL_ENW<'a> {
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
pub type ISO_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ISO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_ENW<'a> {
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
pub type INT_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ENW<'a> {
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
pub type HIRD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _HIRDW<'a> {
    w: &'a mut W,
}
impl<'a> _HIRDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPM_RWU_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPM_RWUW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_RWUW<'a> {
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
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&self) -> HCRESET_R {
        HCRESET_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&self) -> FLS_R {
        FLS_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&self) -> LHCR_R {
        LHCR_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&self) -> ATL_EN_R {
        ATL_EN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&self) -> ISO_EN_R {
        ISO_EN_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits() >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - bRemoteWake field."]
    #[inline(always)]
    pub fn lpm_rwu(&self) -> LPM_RWU_R {
        LPM_RWU_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&mut self) -> _RSW {
        _RSW { w: self }
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&mut self) -> _HCRESETW {
        _HCRESETW { w: self }
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&mut self) -> _FLSW {
        _FLSW { w: self }
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&mut self) -> _LHCRW {
        _LHCRW { w: self }
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&mut self) -> _ATL_ENW {
        _ATL_ENW { w: self }
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&mut self) -> _ISO_ENW {
        _ISO_ENW { w: self }
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&mut self) -> _INT_ENW {
        _INT_ENW { w: self }
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&mut self) -> _HIRDW {
        _HIRDW { w: self }
    }
    #[doc = "Bit 28 - bRemoteWake field."]
    #[inline(always)]
    pub fn lpm_rwu(&mut self) -> _LPM_RWUW {
        _LPM_RWUW { w: self }
    }
}
