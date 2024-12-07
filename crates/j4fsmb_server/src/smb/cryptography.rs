use sha2::Sha256;
use aes::Aes128;
use cmac::{Mac, Cmac};
use hmac::Hmac;

type HmacSha256 = Hmac<Sha256>;
type CmacAes128 = Cmac<Aes128>;

/*
pub fn calculate_signature(
    signing_key: &str,
    dialect: u16,
) -> anyhow::Result<String> {
    if dialect == 0x0202 || dialect == 0x0210 {
        let mac = HmacSha256::new_from_slice(signing_key.as_bytes())?;
        mac
    } else {
        CmacAes128::new_from_slice(signing_key.as_bytes())
    }
}

pub fn generate_signing_key(
    session_key: &str,
    dialect: u16,
    preauth_integrity_hash_value: String,
) -> String {
    if dialect == 0x0202 || dialect == 0x0210 {
        String::from(session_key)
    } else {
        let label: String = if dialect == 0x0311 {
            String::from("SMBSigningKey")
        } else {
            String::from("SMB2AESCMAC")
        };
        let context = if dialect == 0x0311 {
            preauth_integrity_hash_value
        } else {
            String::from("SmbSign")
        };

        let hmac = HmacSha256::new_from_slice(session_key);
        return String::from("");
    }
}

pub fn generate_client_encryption_key(
    session_key: &str,
    dialect: u16,
    preauth_integrity_hash_value: String,
) -> String {
    let label_string = if dialect == 0x0311 { 
        String::from("SMBC2SCipherKey") 
    } else { 
        String::from("SMB2AESCCM")
    };
    let context = if dialect == 0x0311 { 
        preauth_integrity_hash_value
    } else {
        String::from("ServerIn ")
    };

    let hmac = HmacSha256::new_from_slice(session_key);
    return String::from("");
}

pub fn generate_client_decryption_key(
    session_key: String,
    dialect: u16, 
    preauth_integrity_hash_value: String,
) -> String {

    let label_string = if dialect == 0x0311 { 
        String::from("SMBC2SCipherKey") 
    } else { 
        String::from("SMB2AESCCM")
    };
    let context = if dialect == 0x0311 { 
        preauth_integrity_hash_value
    } else { 
        String::from("ServerOut")
    };

    let hmac = HmacSha256::new_from_slice(session_key);
    let mode = rust_kbkdf::KDFMode::CounterMode(
        rust_kbkdf::CounterMode { counter_length: 128 }
    );
    return String::from("");
}

*/
