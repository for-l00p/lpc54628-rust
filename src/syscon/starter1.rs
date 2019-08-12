#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STARTER1 {
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
pub type PINT4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINT4W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT4W<'a> {
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
pub type PINT5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINT5W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT5W<'a> {
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
pub type PINT6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINT6W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT6W<'a> {
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
pub type PINT7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PINT7W<'a> {
    w: &'a mut W,
}
impl<'a> _PINT7W<'a> {
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
pub type CTIMER2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER2W<'a> {
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
pub type CTIMER4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CTIMER4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTIMER4W<'a> {
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
pub type SPIFI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SPIFIW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIFIW<'a> {
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
pub type FLEXCOMM8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM8W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM8W<'a> {
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
pub type FLEXCOMM9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FLEXCOMM9W<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXCOMM9W<'a> {
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
pub type USB1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1W<'a> {
    w: &'a mut W,
}
impl<'a> _USB1W<'a> {
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
pub type USB1_ACT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USB1_ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB1_ACTW<'a> {
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
#[doc = r"Reader of the field"]
pub type ENET_INT1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ENET_INT1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_INT1W<'a> {
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
#[doc = r"Reader of the field"]
pub type ENET_INT2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ENET_INT2W<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_INT2W<'a> {
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
#[doc = r"Reader of the field"]
pub type ENET_INT0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ENET_INT0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENET_INT0W<'a> {
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
#[doc = r"Reader of the field"]
pub type SMARTCARD0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMARTCARD0W<'a> {
    w: &'a mut W,
}
impl<'a> _SMARTCARD0W<'a> {
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
pub type SMARTCARD1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMARTCARD1W<'a> {
    w: &'a mut W,
}
impl<'a> _SMARTCARD1W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - GPIO pin interrupt 4 wake-up."]
    #[inline(always)]
    pub fn pint4(&self) -> PINT4_R {
        PINT4_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn pint5(&self) -> PINT5_R {
        PINT5_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn pint6(&self) -> PINT6_R {
        PINT6_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn pint7(&self) -> PINT7_R {
        PINT7_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Standard counter/timer CTIMER2 wake-up."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Standard counter/timer CTIMER4 wake-up."]
    #[inline(always)]
    pub fn ctimer4(&self) -> CTIMER4_R {
        CTIMER4_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
    #[inline(always)]
    pub fn spifi(&self) -> SPIFI_R {
        SPIFI_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flexcomm Interface 8 wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&self) -> FLEXCOMM8_R {
        FLEXCOMM8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Flexcomm Interface 9 wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&self) -> FLEXCOMM9_R {
        FLEXCOMM9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USB 1 wake-up."]
    #[inline(always)]
    pub fn usb1(&self) -> USB1_R {
        USB1_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB 1 activity wake-up."]
    #[inline(always)]
    pub fn usb1_act(&self) -> USB1_ACT_R {
        USB1_ACT_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Ethernet."]
    #[inline(always)]
    pub fn enet_int1(&self) -> ENET_INT1_R {
        ENET_INT1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Ethernet."]
    #[inline(always)]
    pub fn enet_int2(&self) -> ENET_INT2_R {
        ENET_INT2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Ethernet."]
    #[inline(always)]
    pub fn enet_int0(&self) -> ENET_INT0_R {
        ENET_INT0_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Smart card 0 wake-up."]
    #[inline(always)]
    pub fn smartcard0(&self) -> SMARTCARD0_R {
        SMARTCARD0_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Smart card 1 wake-up."]
    #[inline(always)]
    pub fn smartcard1(&self) -> SMARTCARD1_R {
        SMARTCARD1_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPIO pin interrupt 4 wake-up."]
    #[inline(always)]
    pub fn pint4(&mut self) -> _PINT4W {
        _PINT4W { w: self }
    }
    #[doc = "Bit 1 - GPIO pin interrupt 5 wake-up."]
    #[inline(always)]
    pub fn pint5(&mut self) -> _PINT5W {
        _PINT5W { w: self }
    }
    #[doc = "Bit 2 - GPIO pin interrupt 6 wake-up."]
    #[inline(always)]
    pub fn pint6(&mut self) -> _PINT6W {
        _PINT6W { w: self }
    }
    #[doc = "Bit 3 - GPIO pin interrupt 7 wake-up."]
    #[inline(always)]
    pub fn pint7(&mut self) -> _PINT7W {
        _PINT7W { w: self }
    }
    #[doc = "Bit 4 - Standard counter/timer CTIMER2 wake-up."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> _CTIMER2W {
        _CTIMER2W { w: self }
    }
    #[doc = "Bit 5 - Standard counter/timer CTIMER4 wake-up."]
    #[inline(always)]
    pub fn ctimer4(&mut self) -> _CTIMER4W {
        _CTIMER4W { w: self }
    }
    #[doc = "Bit 7 - SPIFI interrupt wake-up. 0 = Wake-up disabled. 1 = Wake-up enabled."]
    #[inline(always)]
    pub fn spifi(&mut self) -> _SPIFIW {
        _SPIFIW { w: self }
    }
    #[doc = "Bit 8 - Flexcomm Interface 8 wake-up."]
    #[inline(always)]
    pub fn flexcomm8(&mut self) -> _FLEXCOMM8W {
        _FLEXCOMM8W { w: self }
    }
    #[doc = "Bit 9 - Flexcomm Interface 9 wake-up."]
    #[inline(always)]
    pub fn flexcomm9(&mut self) -> _FLEXCOMM9W {
        _FLEXCOMM9W { w: self }
    }
    #[doc = "Bit 15 - USB 1 wake-up."]
    #[inline(always)]
    pub fn usb1(&mut self) -> _USB1W {
        _USB1W { w: self }
    }
    #[doc = "Bit 16 - USB 1 activity wake-up."]
    #[inline(always)]
    pub fn usb1_act(&mut self) -> _USB1_ACTW {
        _USB1_ACTW { w: self }
    }
    #[doc = "Bit 17 - Ethernet."]
    #[inline(always)]
    pub fn enet_int1(&mut self) -> _ENET_INT1W {
        _ENET_INT1W { w: self }
    }
    #[doc = "Bit 18 - Ethernet."]
    #[inline(always)]
    pub fn enet_int2(&mut self) -> _ENET_INT2W {
        _ENET_INT2W { w: self }
    }
    #[doc = "Bit 19 - Ethernet."]
    #[inline(always)]
    pub fn enet_int0(&mut self) -> _ENET_INT0W {
        _ENET_INT0W { w: self }
    }
    #[doc = "Bit 23 - Smart card 0 wake-up."]
    #[inline(always)]
    pub fn smartcard0(&mut self) -> _SMARTCARD0W {
        _SMARTCARD0W { w: self }
    }
    #[doc = "Bit 24 - Smart card 1 wake-up."]
    #[inline(always)]
    pub fn smartcard1(&mut self) -> _SMARTCARD1W {
        _SMARTCARD1W { w: self }
    }
}
