#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODCFG {
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
        0x0173
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type NOC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NOCW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NOB_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NOBW<'a> {
    w: &'a mut W,
}
impl<'a> _NOBW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `MULTITASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTITASKR {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE,
    #[doc = "Multi-task mode."]
    MULTI_TASK_MODE,
}
impl crate::ToBits<bool> for MULTITASKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MULTITASKR::HARDWARE_STATUS_MODE => false,
            MULTITASKR::MULTI_TASK_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MULTITASK_R = crate::FR<bool, MULTITASKR>;
impl MULTITASK_R {
    #[doc = "Checks if the value of the field is `HARDWARE_STATUS_MODE`"]
    #[inline(always)]
    pub fn is_hardware_status_mode(&self) -> bool {
        *self == MULTITASKR::HARDWARE_STATUS_MODE
    }
    #[doc = "Checks if the value of the field is `MULTI_TASK_MODE`"]
    #[inline(always)]
    pub fn is_multi_task_mode(&self) -> bool {
        *self == MULTITASKR::MULTI_TASK_MODE
    }
}
#[doc = "Values that can be written to the field `MULTITASK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTITASKW {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE,
    #[doc = "Multi-task mode."]
    MULTI_TASK_MODE,
}
impl MULTITASKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MULTITASKW::HARDWARE_STATUS_MODE => false,
            MULTITASKW::MULTI_TASK_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MULTITASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTITASKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MULTITASKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    #[inline(always)]
    pub fn hardware_status_mode(self) -> &'a mut W {
        self.variant(MULTITASKW::HARDWARE_STATUS_MODE)
    }
    #[doc = "Multi-task mode."]
    #[inline(always)]
    pub fn multi_task_mode(self) -> &'a mut W {
        self.variant(MULTITASKW::MULTI_TASK_MODE)
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
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&self) -> NOC_R {
        NOC_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&self) -> NOB_R {
        NOB_R::new(((self.bits() >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn multitask(&self) -> MULTITASK_R {
        MULTITASK_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&mut self) -> _NOCW {
        _NOCW { w: self }
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&mut self) -> _NOBW {
        _NOBW { w: self }
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn multitask(&mut self) -> _MULTITASKW {
        _MULTITASKW { w: self }
    }
}
