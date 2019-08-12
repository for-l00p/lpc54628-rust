#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWVADST10 {
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
#[doc = "Possible values of the field `ST10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST10R {
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    NORMAL,
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    RESET,
}
impl crate::ToBits<bool> for ST10R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ST10R::NORMAL => false,
            ST10R::RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ST10_R = crate::FR<bool, ST10R>;
impl ST10_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ST10R::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ST10R::RESET
    }
}
#[doc = "Values that can be written to the field `ST10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST10W {
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    NORMAL,
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    RESET,
}
impl ST10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ST10W::NORMAL => false,
            ST10W::RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ST10W<'a> {
    w: &'a mut W,
}
impl<'a> _ST10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ST10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ST10W::NORMAL)
    }
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ST10W::RESET)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Stage 0"]
    #[inline(always)]
    pub fn st10(&self) -> ST10_R {
        ST10_R::new((self.bits() & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Stage 0"]
    #[inline(always)]
    pub fn st10(&mut self) -> _ST10W {
        _ST10W { w: self }
    }
}
