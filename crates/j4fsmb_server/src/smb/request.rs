pub struct Request {
    pub request_id: u32,
    pub packet_type: u16,
    pub flags: u16,
    pub data: Vec<u8>,
}
