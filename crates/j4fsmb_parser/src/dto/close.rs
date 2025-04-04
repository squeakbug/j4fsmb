use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CloseRequest {
    pub structure_size: u16,
    pub flags: u16,
    pub reserved: u32,
    pub file_id: u128,
}

impl Smb2Request for CloseRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CloseResponse {
    pub structure_size: u16,
    pub flags: u16,
    pub reserved: u32,
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    pub allocation_size: u64,
    pub end_of_file: u64,
    pub file_attributes: u32,
}

impl Smb2Response for CloseResponse { }
