#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DLY {
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
pub type PRE_DELAY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _PRE_DELAYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type POST_DELAY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _POST_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _POST_DELAYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FRAME_DELAY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FRAME_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAME_DELAYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRANSFER_DELAY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRANSFER_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFER_DELAYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn pre_delay(&self) -> PRE_DELAY_R {
        PRE_DELAY_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn post_delay(&self) -> POST_DELAY_R {
        POST_DELAY_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn frame_delay(&self) -> FRAME_DELAY_R {
        FRAME_DELAY_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn transfer_delay(&self) -> TRANSFER_DELAY_R {
        TRANSFER_DELAY_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn pre_delay(&mut self) -> _PRE_DELAYW {
        _PRE_DELAYW { w: self }
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn post_delay(&mut self) -> _POST_DELAYW {
        _POST_DELAYW { w: self }
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn frame_delay(&mut self) -> _FRAME_DELAYW {
        _FRAME_DELAYW { w: self }
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn transfer_delay(&mut self) -> _TRANSFER_DELAYW {
        _TRANSFER_DELAYW { w: self }
    }
}
