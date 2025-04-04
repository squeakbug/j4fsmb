use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct ChangeNotifyRequest {
    pub structure_size: u16,
    pub flags: u16,
    pub file_id: u128,
    pub completion_filter: u32,
    pub reserved: u32,
}

impl Smb2Request for ChangeNotifyRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct ChangeNotifyResponse {
    pub structure_size: u16,
    pub output_buffer_offset: u16,
    pub output_buffer_length: u32,
    #[deku(count = "output_buffer_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Response for ChangeNotifyResponse { }
