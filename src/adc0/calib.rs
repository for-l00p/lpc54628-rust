#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CALIB {
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
        0x02
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type CALIB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CALIBW<'a> {
    w: &'a mut W,
}
impl<'a> _CALIBW<'a> {
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
pub type CALREQD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CALREQDW<'a> {
    w: &'a mut W,
}
impl<'a> _CALREQDW<'a> {
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
pub type CALVALUE_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CALVALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _CALVALUEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 2)) | (((value as u32) & 0x7f) << 2);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Calibration request. Setting this bit will launch an ADC calibration cycle. This bit can only be set to a '1' by software. It is cleared automatically when the calibration cycle completes."]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Calibration required. This read-only bit indicates if calibration is required when enabling the ADC. CALREQD will be '1' if no calibration has been run since the chip was powered-up and if the BYPASSCAL bit in the CTRL register is low. Software will test this bit to determine whether to initiate a calibration cycle or whether to set the ADC_INIT bit (in the STARTUP register) to launch the ADC initialization process which includes a 'dummy' conversion cycle. Note: A 'dummy' conversion cycle requires approximately 6 ADC clocks as opposed to 81 clocks required for calibration."]
    #[inline(always)]
    pub fn calreqd(&self) -> CALREQD_R {
        CALREQD_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:8 - Calibration Value. This read-only field displays the calibration value established during last calibration cycle. This value is not typically of any use to the user."]
    #[inline(always)]
    pub fn calvalue(&self) -> CALVALUE_R {
        CALVALUE_R::new(((self.bits() >> 2) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Calibration request. Setting this bit will launch an ADC calibration cycle. This bit can only be set to a '1' by software. It is cleared automatically when the calibration cycle completes."]
    #[inline(always)]
    pub fn calib(&mut self) -> _CALIBW {
        _CALIBW { w: self }
    }
    #[doc = "Bit 1 - Calibration required. This read-only bit indicates if calibration is required when enabling the ADC. CALREQD will be '1' if no calibration has been run since the chip was powered-up and if the BYPASSCAL bit in the CTRL register is low. Software will test this bit to determine whether to initiate a calibration cycle or whether to set the ADC_INIT bit (in the STARTUP register) to launch the ADC initialization process which includes a 'dummy' conversion cycle. Note: A 'dummy' conversion cycle requires approximately 6 ADC clocks as opposed to 81 clocks required for calibration."]
    #[inline(always)]
    pub fn calreqd(&mut self) -> _CALREQDW {
        _CALREQDW { w: self }
    }
    #[doc = "Bits 2:8 - Calibration Value. This read-only field displays the calibration value established during last calibration cycle. This value is not typically of any use to the user."]
    #[inline(always)]
    pub fn calvalue(&mut self) -> _CALVALUEW {
        _CALVALUEW { w: self }
    }
}
