use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct QueryDirectoryRequest {
    pub structure_size: u16,
    pub file_info_class: u8,
    pub flags: u8,
    pub file_index: u32,
    pub file_id: u128,
    pub file_name_offset: u16,
    pub file_name_length: u16,
    pub output_buffer_length: u32,
    #[deku(count = "file_name_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Request for QueryDirectoryRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct QueryDirectoryResponse {
    pub structure_size: u16,
    pub output_buffer_offset: u16,
    pub output_buffer_length: u32,
    #[deku(count = "output_buffer_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Response for QueryDirectoryResponse { }
