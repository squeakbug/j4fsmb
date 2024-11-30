use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct TreeConnectRequest {
    pub structure_size: u16,
    pub flags: u16,
    pub path_offset: u16,
    pub path_length: u16,
    #[deku(count = "path_length")]
    pub buffer: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct TreeConnectResponse {
    pub structure_size: u16,
    pub share_type: u8,
    pub reserved: u8,
    pub share_flags: u32,
    pub capabilities: u32,
    pub maximal_access: u32,
}
