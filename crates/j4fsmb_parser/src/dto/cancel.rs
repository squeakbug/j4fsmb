use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CloseRequest {
    pub structure_size: u16,
    pub flags: u16,
    pub reserved: u32,
    pub file_id: u128,
}
