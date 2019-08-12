#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAC_HW_FEAT0 {
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
pub type MIISEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type HDSEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type VLHASH_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SMASEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RWKSEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MGKSEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MGKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MGKSELW<'a> {
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
pub type MMCSEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ARPOFFSEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TSSEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EEESEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXCOESEL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXCOESEL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXCOESELW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCOESELW<'a> {
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
pub type TSSTSSEL_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type ACTPHYSEL_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - 10 or 100 Mbps Support."]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half-duplex Support."]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hash Table Based Filtering option."]
    #[inline(always)]
    pub fn vlhash(&self) -> VLHASH_R {
        VLHASH_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SMA (MDIO) Interface."]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PMT Remote Wake-up Packet Detection."]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PMT magic packet detection."]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RMON Module Enable."]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ARP Offload Enabled."]
    #[inline(always)]
    pub fn arpoffsel(&self) -> ARPOFFSEL_R {
        ARPOFFSEL_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IEEE 1588-2008 Timestamp support ."]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Energy Efficient Ethernet Support ."]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Checksum Offload Support."]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Checksum Offload Support."]
    #[inline(always)]
    pub fn rxcoesel(&self) -> RXCOESEL_R {
        RXCOESEL_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Timestamp System Time Source."]
    #[inline(always)]
    pub fn tsstssel(&self) -> TSSTSSEL_R {
        TSSTSSEL_R::new(((self.bits() >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 28:30 - Active PHY Selected."]
    #[inline(always)]
    pub fn actphysel(&self) -> ACTPHYSEL_R {
        ACTPHYSEL_R::new(((self.bits() >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - PMT magic packet detection."]
    #[inline(always)]
    pub fn mgksel(&mut self) -> _MGKSELW {
        _MGKSELW { w: self }
    }
    #[doc = "Bit 16 - Receive Checksum Offload Support."]
    #[inline(always)]
    pub fn rxcoesel(&mut self) -> _RXCOESELW {
        _RXCOESELW { w: self }
    }
}
