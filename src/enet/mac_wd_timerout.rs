#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_WD_TIMEROUT {
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
pub type WTO_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _WTOW<'a> {
    w: &'a mut W,
}
impl<'a> _WTOW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PWE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PWEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWEW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Watchdog Timeout When the PWE bit is set and the WD bit of the MAC Configuration register Table 722 is reset, this field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn wto(&mut self) -> _WTOW {
        _WTOW { w: self }
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable When this bit is set and the WD bit of the MAC Configuration register Table 722 is reset, the WTO field is used as watchdog timeout for a received packet."]
    #[inline(always)]
    pub fn pwe(&mut self) -> _PWEW {
        _PWEW { w: self }
    }
}
