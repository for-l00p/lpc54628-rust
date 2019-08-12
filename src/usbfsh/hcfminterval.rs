#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HCFMINTERVAL {
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
        0x2edf
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type FI_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _FIW<'a> {
    w: &'a mut W,
}
impl<'a> _FIW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSMPS_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _FSMPSW<'a> {
    w: &'a mut W,
}
impl<'a> _FSMPSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FIT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FITW<'a> {
    w: &'a mut W,
}
impl<'a> _FITW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new((self.bits() & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&self) -> FSMPS_R {
        FSMPS_R::new(((self.bits() >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&self) -> FIT_R {
        FIT_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&mut self) -> _FIW {
        _FIW { w: self }
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&mut self) -> _FSMPSW {
        _FSMPSW { w: self }
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&mut self) -> _FITW {
        _FITW { w: self }
    }
}
