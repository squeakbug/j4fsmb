use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct ReadRequest {
    pub structure_size: u16,
    pub padding: u8,
    pub flags: u8,
    pub length: u32,
    pub offset: u64,
    pub file_id: u128,
    pub minimum_count: u32,
    pub channel: u32,
    pub remaining_bytes: u32,
    pub read_channel_info_offset: u16,
    pub read_channel_info_length: u16,
    #[deku(count = "read_channel_info_length")]
    pub buffer: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct ReadResponse {
    pub structure_size: u16,
    pub data_offset: u8,
    pub reserved: u8,
    pub data_length: u32,
    pub data_remaining: u32,
    pub flags: u32,
    #[deku(count = "data_length")]
    pub buffer: Vec<u8>,
}
