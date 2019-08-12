#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Register for reading the AES key."]
    pub aeskey: [AESKEY; 8],
    #[doc = "0x30 - ECRP options."]
    pub ecrp: ECRP,
    _reserved2: [u8; 4usize],
    #[doc = "0x38 - User application specific options."]
    pub user0: USER0,
    #[doc = "0x3c - User application specific options."]
    pub user1: USER1,
}
#[doc = "Register for reading the AES key."]
pub struct AESKEY {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Register for reading the AES key."]
pub mod aeskey;
#[doc = "ECRP options."]
pub struct ECRP {
    register: vcell::VolatileCell<u32>,
}
#[doc = "ECRP options."]
pub mod ecrp;
#[doc = "User application specific options."]
pub struct USER0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "User application specific options."]
pub mod user0;
#[doc = "User application specific options."]
pub struct USER1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "User application specific options."]
pub mod user1;
