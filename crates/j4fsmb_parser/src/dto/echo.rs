use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct EchoRequest {
    pub structure_size: u16,
    pub info_type: u8,
    pub file_info_class: u8,
    pub output_buffer_length: u32,
    pub input_buffer_offset: u16,
    pub reserved: u16,
    pub input_buffer_length: u32,
    pub additional_information: u32,
    pub flags: u32,
    pub file_id: u128,
    #[deku(count = "input_buffer_length")]
    pub buffer: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct EchoResponse {
    pub structure_size: u16,
    pub output_buffer_offset: u16,
    pub output_buffer_length: u32,
    #[deku(count = "output_buffer_length")]
    pub buffer: Vec<u8>,
}
