#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPACR {
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
#[doc = "Possible values of the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    CP10_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    CP10_1,
    #[doc = "Full access."]
    CP10_3,
}
impl crate::ToBits<u8> for CP10R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CP10R::CP10_0 => 0,
            CP10R::CP10_1 => 1,
            CP10R::CP10_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CP10_R = crate::FR<u8, CP10R>;
impl CP10_R {
    #[doc = "Checks if the value of the field is `CP10_0`"]
    #[inline(always)]
    pub fn is_cp10_0(&self) -> bool {
        *self == CP10R::CP10_0
    }
    #[doc = "Checks if the value of the field is `CP10_1`"]
    #[inline(always)]
    pub fn is_cp10_1(&self) -> bool {
        *self == CP10R::CP10_1
    }
    #[doc = "Checks if the value of the field is `CP10_3`"]
    #[inline(always)]
    pub fn is_cp10_3(&self) -> bool {
        *self == CP10R::CP10_3
    }
}
#[doc = "Values that can be written to the field `CP10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP10W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    CP10_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    CP10_1,
    #[doc = "Full access."]
    CP10_3,
}
impl CP10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP10W::CP10_0 => 0,
            CP10W::CP10_1 => 1,
            CP10W::CP10_3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CP10W<'a> {
    w: &'a mut W,
}
impl<'a> _CP10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline(always)]
    pub fn cp10_0(self) -> &'a mut W {
        self.variant(CP10W::CP10_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn cp10_1(self) -> &'a mut W {
        self.variant(CP10W::CP10_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp10_3(self) -> &'a mut W {
        self.variant(CP10W::CP10_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11R {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    CP11_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    CP11_1,
    #[doc = "Full access."]
    CP11_3,
}
impl crate::ToBits<u8> for CP11R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CP11R::CP11_0 => 0,
            CP11R::CP11_1 => 1,
            CP11R::CP11_3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CP11_R = crate::FR<u8, CP11R>;
impl CP11_R {
    #[doc = "Checks if the value of the field is `CP11_0`"]
    #[inline(always)]
    pub fn is_cp11_0(&self) -> bool {
        *self == CP11R::CP11_0
    }
    #[doc = "Checks if the value of the field is `CP11_1`"]
    #[inline(always)]
    pub fn is_cp11_1(&self) -> bool {
        *self == CP11R::CP11_1
    }
    #[doc = "Checks if the value of the field is `CP11_3`"]
    #[inline(always)]
    pub fn is_cp11_3(&self) -> bool {
        *self == CP11R::CP11_3
    }
}
#[doc = "Values that can be written to the field `CP11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CP11W {
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    CP11_0,
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    CP11_1,
    #[doc = "Full access."]
    CP11_3,
}
impl CP11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CP11W::CP11_0 => 0,
            CP11W::CP11_1 => 1,
            CP11W::CP11_3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CP11W<'a> {
    w: &'a mut W,
}
impl<'a> _CP11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault"]
    #[inline(always)]
    pub fn cp11_0(self) -> &'a mut W {
        self.variant(CP11W::CP11_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP fault."]
    #[inline(always)]
    pub fn cp11_1(self) -> &'a mut W {
        self.variant(CP11W::CP11_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp11_3(self) -> &'a mut W {
        self.variant(CP11W::CP11_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits() >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits() >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&mut self) -> _CP10W {
        _CP10W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&mut self) -> _CP11W {
        _CP11W { w: self }
    }
}
