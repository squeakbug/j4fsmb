use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct TransformHeader {
    pub protocol_id: u32,
    pub signature: u128,
    pub nonce: u128,
    pub original_message_size: u32,
    pub reserved: u16,
    pub flags: u16,
    pub session_id: u32,
}
