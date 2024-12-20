use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct WriteRequest {
    pub structure_size: u16,
    pub data_offset: u16,
    pub length: u32,
    pub offset: u64,
    pub file_id: u128,
    pub channel: u32,
    pub remaining_bytes: u32,
    pub write_channel_info_offset: u16,
    pub write_channel_info_length: u16,
    #[deku(count = "write_channel_info_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Request for WriteRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct WriteResponse {
    pub structure_size: u16,
    pub reserved: u16,
    pub count: u32,
    pub remaining: u32,
    pub write_channel_info_offset: u16,
    pub write_channel_info_length: u16,
}

impl Smb2Response for WriteResponse { }
