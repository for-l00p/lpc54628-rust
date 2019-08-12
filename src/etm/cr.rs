#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
        0x0411
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type ETMPD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETMPDW<'a> {
    w: &'a mut W,
}
impl<'a> _ETMPDW<'a> {
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
pub type PS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPW<'a> {
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
pub type BO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BOW<'a> {
    w: &'a mut W,
}
impl<'a> _BOW<'a> {
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
pub type DRC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRCW<'a> {
    w: &'a mut W,
}
impl<'a> _DRCW<'a> {
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
pub type ETMP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ETMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ETMPW<'a> {
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
#[doc = "Possible values of the field `ETMPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMPSR {
    #[doc = "ETMEN is LOW."]
    ETMPS_0,
    #[doc = "ETMEN is HIGH."]
    ETMPS_1,
}
impl crate::ToBits<bool> for ETMPSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ETMPSR::ETMPS_0 => false,
            ETMPSR::ETMPS_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ETMPS_R = crate::FR<bool, ETMPSR>;
impl ETMPS_R {
    #[doc = "Checks if the value of the field is `ETMPS_0`"]
    #[inline(always)]
    pub fn is_etmps_0(&self) -> bool {
        *self == ETMPSR::ETMPS_0
    }
    #[doc = "Checks if the value of the field is `ETMPS_1`"]
    #[inline(always)]
    pub fn is_etmps_1(&self) -> bool {
        *self == ETMPSR::ETMPS_1
    }
}
#[doc = "Values that can be written to the field `ETMPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMPSW {
    #[doc = "ETMEN is LOW."]
    ETMPS_0,
    #[doc = "ETMEN is HIGH."]
    ETMPS_1,
}
impl ETMPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ETMPSW::ETMPS_0 => false,
            ETMPSW::ETMPS_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ETMPSW<'a> {
    w: &'a mut W,
}
impl<'a> _ETMPSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ETMEN is LOW."]
    #[inline(always)]
    pub fn etmps_0(self) -> &'a mut W {
        self.variant(ETMPSW::ETMPS_0)
    }
    #[doc = "ETMEN is HIGH."]
    #[inline(always)]
    pub fn etmps_1(self) -> &'a mut W {
        self.variant(ETMPSW::ETMPS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PM2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PM2W<'a> {
    w: &'a mut W,
}
impl<'a> _PM2W<'a> {
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
pub type PM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PS3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PS3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
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
    #[doc = "Bit 0 - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
    #[inline(always)]
    pub fn etmpd(&self) -> ETMPD_R {
        ETMPD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn drc(&self) -> DRC_R {
        DRC_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
    #[inline(always)]
    pub fn etmp(&self) -> ETMP_R {
        ETMP_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn etmps(&self) -> ETMPS_R {
        ETMPS_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn pm2(&self) -> PM2_R {
        PM2_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 21 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn ps3(&self) -> PS3_R {
        PS3_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ETM power down. This bit can be used by an implementation to control if the ETM is in a low power state. This bit must be cleared by the trace software tools at the beginning of a debug session. When this bit is set to 1, writes to some registers and fields might be ignored."]
    #[inline(always)]
    pub fn etmpd(&mut self) -> _ETMPDW {
        _ETMPDW { w: self }
    }
    #[doc = "Bits 4:6 - Port size. The ETM-M4 has no influence over the external pins used for trace. These bits are implemented but not used. On an ETM reset these bits reset to 0b001."]
    #[inline(always)]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 7 - Stall processor. The FIFOFULL output can be used to stall the processor to prevent overflow. The FIFOFULL output is only enabled when the stall processor bit is set to 1. When the bit is 0 the FIFOFULL output remains LOW at all times and the FIFO overflows if there are too many trace packets. Trace resumes without corruption once the FIFO has drained, if overflow does occur. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn sp(&mut self) -> _SPW {
        _SPW { w: self }
    }
    #[doc = "Bit 8 - Branch output. When set to 1 all branch addresses are output, even if the branch was because of a direct branch instruction. Setting this bit enables reconstruction of the program flow without having access to the memory image of the code being executed. When this bit is set to 1, more trace data is generated, and this may affect the performance of the trace system. Information about the execution of a branch is traced regardless of the state of this bit. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn bo(&mut self) -> _BOW {
        _BOW { w: self }
    }
    #[doc = "Bit 9 - Debug request control. When set to 1 and the trigger event occurs, the DBGRQ output is asserted until DBGACK is observed. This enables the ARM processor to be forced into Debug state. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn drc(&mut self) -> _DRCW {
        _DRCW { w: self }
    }
    #[doc = "Bit 10 - ETM programming. This bit must be set to 1 at the start of the ETM programming sequence. Tracing is prevented while this bit is set to 1. On an ETM reset this bit is set to b1."]
    #[inline(always)]
    pub fn etmp(&mut self) -> _ETMPW {
        _ETMPW { w: self }
    }
    #[doc = "Bit 11 - ETM port selection. This bit can be used to control other trace components in an implementation. This bit must be set by the trace software tools to ensure that trace output is enabled from this ETM. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn etmps(&mut self) -> _ETMPSW {
        _ETMPSW { w: self }
    }
    #[doc = "Bit 13 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn pm2(&mut self) -> _PM2W {
        _PM2W { w: self }
    }
    #[doc = "Bits 16:17 - These bits are implemented but have no function. An ETM reset sets these bits to 0."]
    #[inline(always)]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 21 - This bit is implemented but has no function. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn ps3(&mut self) -> _PS3W {
        _PS3W { w: self }
    }
    #[doc = "Bit 28 - When set, this bit enables timestamping. An ETM reset sets this bit to 0."]
    #[inline(always)]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
}
