use deku::prelude::*;

type Dialiect = u32;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct PreauthIntegrityCapabilities {
    pub hash_algorithm_count: u16,
    pub salt_length: u16,
    #[deku(count = "hash_algorithm_count")]
    pub hash_algorithms: Vec<u32>,
    #[deku(count = "salt_length")]
    pub salt: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct EncryptionCapabilities {
    pub cipher_count: u16,
    #[deku(count = "cipher_count")]
    pub ciphers: Vec<u16>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CompressionCapabilities {
    pub compression_algorithm_count: u16,
    pub padding: u16,
    pub flags: u32,
    #[deku(count = "compression_algorithm_count")]
    pub compression_algorithms: Vec<u32>,
}

type TransportCapabilities = u32;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct RDMATransformCapabilities {
    pub transform_count: u16,
    pub reserved1: u16,
    pub reserved2: u32,
    #[deku(count = "transform_count")]
    pub rdma_transform_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct SigningCapabilities {
    pub signing_algorithm_count: u16,
    #[deku(count = "signing_algorithm_count")]
    pub signing_algorithms: Vec<u16>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "context_type: u16", id = "context_type")]
pub enum NegotiateContextType {
    #[deku(id = 0x0001)]
    PreauthIntegrity(PreauthIntegrityCapabilities),
    #[deku(id = 0x0002)]
    Encryption(EncryptionCapabilities),
    #[deku(id = 0x0003)]
    Compression(CompressionCapabilities),
    #[deku(id = 0x0006)]
    Transport(TransportCapabilities),
    #[deku(id = 0x0007)]
    RDMATransform(RDMATransformCapabilities),
    #[deku(id = 0x0008)]
    Signing(SigningCapabilities),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct NegotiateContext {
    pub context_type: u16,
    pub data_length: u16,
    pub reserved: u32,
    #[deku(ctx = "*context_type")]
    pub data: NegotiateContextType,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct NegotiateRequest {
    pub structure_size: u16,
    pub dialect_count: u16,
    pub security_mode: u16,
    pub reserved: u16,
    pub capabilities: u32,
    pub client_guid: u128,
    pub client_start_time: u64,
    #[deku(count = "dialect_count")]
    pub dialects: Vec<Dialiect>,
    #[deku(count = "dialect_count")]
    pub negotiate_context_list: Vec<NegotiateContext>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct NegotiateResponse {
    pub structure_size: u16,
    pub security_mode: u16,
    pub reserved: u32,
    pub capabilities: u32,
    pub max_transact_size: u32,
    pub max_read_size: u32,
    pub max_write_size: u32,
    pub system_time: [u8; 8],
    pub server_time_zone: u32,
    pub sign_key_length: u16,
    pub reserved2: u16,
    pub dialect_index: u16,
    pub dialects: [u16; 1],
}

pub fn smb2_dialect_string(d: u16) -> String {
    match d {
        0x0202 => "2.02",
        0x0210 => "2.10",
        0x0222 => "2.22",
        0x0224 => "2.24",
        0x02ff => "2.??",
        0x0300 => "3.00",
        0x0302 => "3.02",
        0x0310 => "3.10",
        0x0311 => "3.11",
        _ => { return (d).to_string(); },
    }.to_string()
}
