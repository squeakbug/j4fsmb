use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CreateRequest {
    pub structure_size: u16,
    pub security_flags: u8,
    pub requested_oplock_level: u8,
    pub impersonation_level: u32,
    pub smb_create_flags: u64,
    pub reserved: u64,
    pub desired_access: u32,
    pub file_attributes: u32,
    pub share_access: u32,
    pub create_disposition: u32,
    pub create_options: u32,
    pub name_offset: u16,
    pub name_length: u16,
    pub create_contexts_offset: u32,
    pub create_contexts_length: u32,
    #[deku(count = "create_contexts_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Request for CreateRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CreateResponse {
    pub structure_size: u16,
    pub oplock_level: u8,
    pub flags: u8,
    pub create_action: u32,
    pub creation_time: u64,
    pub last_access_time: u64,
    pub last_write_time: u64,
    pub change_time: u64,
    pub allocation_size: u64,
    pub end_of_file: u64,
    pub file_attributes: u32,
    pub reserved2: u16,
    pub file_id: u128,
    pub create_contexts_offset: u32,
    pub create_contexts_length: u32,
    #[deku(count = "create_contexts_length")]
    pub buffer: Vec<u8>,
}

impl Smb2Response for CreateResponse { }
