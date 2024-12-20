use deku::prelude::*;

use crate::{Smb2Request, Smb2Response};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct FlushRequest {
    pub structure_size: u16,
    pub reserved1: u16,
    pub reserved2: u32,
    pub file_id: u128,
}

impl Smb2Request for FlushRequest { }

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct FlushResponse {
    pub structure_size: u16,
    pub reserved: u16,
}

impl Smb2Response for FlushResponse { }
