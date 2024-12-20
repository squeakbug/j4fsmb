use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct SetInfoRequest {
    pub structure_size: u16,
    pub info_type: u8,
    pub file_info_class: u8,
    pub buffer_length: u32,
    pub buffer_offset: u16,
    pub reserved: u16,
    pub additional_information: u32,
    pub file_id: u128,
    #[deku(count = "buffer_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Request for SetInfoRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct SetInfoResponse {
    pub structure_size: u16,
}

impl Smb2Response for SetInfoResponse { }
