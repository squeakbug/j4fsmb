#[derive(Debug)]
pub enum SmbError {
    MalformedPacket(String),
    UnsupportedCommand(u16),
}

#[repr(C)]
#[derive(Debug)]
pub struct ErrorResponse {
    pub structure_size: u16,
    pub error_context_count: u8,
    pub reserved: u8,
    pub byte_count: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ErrorContextResponse {
    pub error_data_length: u32,
    pub error_id: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SymbolicLinkErrorResponse {
    
}

#[repr(C)]
#[derive(Debug)]
pub struct ShareRedirectErrorContextResponse {

}
