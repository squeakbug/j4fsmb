use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct SessionSetupRequest {
    pub structure_size: u16,
    pub flags: u8,
    pub security_mode: u8,
    pub capabilities: u32,
    pub channel: u32,
    pub security_buffer_offset: u16,
    pub security_buffer_length: u16,
    pub previous_session_id: u64,
    #[deku(count = "security_buffer_length")]
    pub buffer: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct SessionSetupResponse {
    pub structure_size: u16,
    pub session_flags: u16,
    pub security_buffer_offset: u16,
    pub security_buffer_length: u16,
    pub previous_session_id: u64,
    #[deku(count = "security_buffer_length")]
    pub buffer: Vec<u8>,
}
