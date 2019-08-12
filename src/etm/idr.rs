#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IDR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type IMPLEMENTATIONREVISION_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MINORETMARCHITECTUREVERSION_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type MAJORETMARCHITECTUREVERSION_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type PROCESSORFAMILY_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type LOADPCFIRST_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `ThumbInstructionTracing`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THUMBINSTRUCTIONTRACINGR {
    #[doc = "A 32-bit Thumb instruction is traced as two instructions, and exceptions might occur between these two instructions."]
    THUMBINSTRUCTIONTRACING_0,
    #[doc = "A 32-bit Thimb instruction is traced as a single instruction."]
    THUMBINSTRUCTIONTRACING_1,
}
impl crate::ToBits<bool> for THUMBINSTRUCTIONTRACINGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            THUMBINSTRUCTIONTRACINGR::THUMBINSTRUCTIONTRACING_0 => false,
            THUMBINSTRUCTIONTRACINGR::THUMBINSTRUCTIONTRACING_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type THUMBINSTRUCTIONTRACING_R = crate::FR<bool, THUMBINSTRUCTIONTRACINGR>;
impl THUMBINSTRUCTIONTRACING_R {
    #[doc = "Checks if the value of the field is `THUMBINSTRUCTIONTRACING_0`"]
    #[inline(always)]
    pub fn is_thumb_instruction_tracing_0(&self) -> bool {
        *self == THUMBINSTRUCTIONTRACINGR::THUMBINSTRUCTIONTRACING_0
    }
    #[doc = "Checks if the value of the field is `THUMBINSTRUCTIONTRACING_1`"]
    #[inline(always)]
    pub fn is_thumb_instruction_tracing_1(&self) -> bool {
        *self == THUMBINSTRUCTIONTRACINGR::THUMBINSTRUCTIONTRACING_1
    }
}
#[doc = "Possible values of the field `SecurityExtensionSupport`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECURITYEXTENSIONSUPPORTR {
    #[doc = "The ETM behaves as if the processor is in Secure state at all times."]
    SECURITYEXTENSIONSUPPORT_0,
    #[doc = "The ARM architecture Security Extensions are implemented by the processor."]
    SECURITYEXTENSIONSUPPORT_1,
}
impl crate::ToBits<bool> for SECURITYEXTENSIONSUPPORTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SECURITYEXTENSIONSUPPORTR::SECURITYEXTENSIONSUPPORT_0 => false,
            SECURITYEXTENSIONSUPPORTR::SECURITYEXTENSIONSUPPORT_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SECURITYEXTENSIONSUPPORT_R = crate::FR<bool, SECURITYEXTENSIONSUPPORTR>;
impl SECURITYEXTENSIONSUPPORT_R {
    #[doc = "Checks if the value of the field is `SECURITYEXTENSIONSUPPORT_0`"]
    #[inline(always)]
    pub fn is_security_extension_support_0(&self) -> bool {
        *self == SECURITYEXTENSIONSUPPORTR::SECURITYEXTENSIONSUPPORT_0
    }
    #[doc = "Checks if the value of the field is `SECURITYEXTENSIONSUPPORT_1`"]
    #[inline(always)]
    pub fn is_security_extension_support_1(&self) -> bool {
        *self == SECURITYEXTENSIONSUPPORTR::SECURITYEXTENSIONSUPPORT_1
    }
}
#[doc = "Possible values of the field `BranchPacketEncoding`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRANCHPACKETENCODINGR {
    #[doc = "The ETM implements the original branch packet encoding."]
    BRANCHPACKETENCODING_0,
    #[doc = "The ETM implements the alternative branch packet encoding."]
    BRANCHPACKETENCODING_1,
}
impl crate::ToBits<bool> for BRANCHPACKETENCODINGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BRANCHPACKETENCODINGR::BRANCHPACKETENCODING_0 => false,
            BRANCHPACKETENCODINGR::BRANCHPACKETENCODING_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BRANCHPACKETENCODING_R = crate::FR<bool, BRANCHPACKETENCODINGR>;
impl BRANCHPACKETENCODING_R {
    #[doc = "Checks if the value of the field is `BRANCHPACKETENCODING_0`"]
    #[inline(always)]
    pub fn is_branch_packet_encoding_0(&self) -> bool {
        *self == BRANCHPACKETENCODINGR::BRANCHPACKETENCODING_0
    }
    #[doc = "Checks if the value of the field is `BRANCHPACKETENCODING_1`"]
    #[inline(always)]
    pub fn is_branch_packet_encoding_1(&self) -> bool {
        *self == BRANCHPACKETENCODINGR::BRANCHPACKETENCODING_1
    }
}
#[doc = r"Reader of the field"]
pub type IMPLEMENTORCODE_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Implementation revision. The value of these bits is b0000, indicating implementation revision, 0."]
    #[inline(always)]
    pub fn implementation_revision(&self) -> IMPLEMENTATIONREVISION_R {
        IMPLEMENTATIONREVISION_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Minor ETM architecture version. The value of these bits is 0b0101, indicating minor architecture version number 5."]
    #[inline(always)]
    pub fn minor_etmarchitecture_version(&self) -> MINORETMARCHITECTUREVERSION_R {
        MINORETMARCHITECTUREVERSION_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Major ETM architecture version. The value of these bits is 0b0010, indicating major architecture version number 3, ETMv3."]
    #[inline(always)]
    pub fn major_etmarchitecture_version(&self) -> MAJORETMARCHITECTUREVERSION_R {
        MAJORETMARCHITECTUREVERSION_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Processor family. The value of these bits is 0b1111, indicating that the processor family is not identified in this register."]
    #[inline(always)]
    pub fn processor_family(&self) -> PROCESSORFAMILY_R {
        PROCESSORFAMILY_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Load PC first. The value of this bit is 0, indicating that data tracing is not supported."]
    #[inline(always)]
    pub fn load_pcfirst(&self) -> LOADPCFIRST_R {
        LOADPCFIRST_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 32-bit Thumb instruction tracing. The value of this bit is 1, indicating that a 32-bit Thumb instruction is traced as a single instruction."]
    #[inline(always)]
    pub fn thumb_instruction_tracing(&self) -> THUMBINSTRUCTIONTRACING_R {
        THUMBINSTRUCTIONTRACING_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security Extensions support. The value of this bit is 0, indicating that the ETM behaves as if the processor is in Secure state at all times."]
    #[inline(always)]
    pub fn security_extension_support(&self) -> SECURITYEXTENSIONSUPPORT_R {
        SECURITYEXTENSIONSUPPORT_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Branch packet encoding. The value of this bit is 1, indicating that alternative branch packet encoding is implemented."]
    #[inline(always)]
    pub fn branch_packet_encoding(&self) -> BRANCHPACKETENCODING_R {
        BRANCHPACKETENCODING_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Implementor code. These bits identify ARM as the implementor of the processor. The value of these bits is 01000001."]
    #[inline(always)]
    pub fn implementor_code(&self) -> IMPLEMENTORCODE_R {
        IMPLEMENTORCODE_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
