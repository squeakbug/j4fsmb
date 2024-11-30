use sha2::Sha256;
use aes::Aes128;
use cmac::{Cmac, Mac};
use hmac::{Hmac, Mac};

use j4fsmb_parser::transform::TransformHeader;

type HmacSha256 = Hmac<Sha256>;
type CmacAes128 = Cmac<Aes128>;

const aes_ccm_nonce_length: u32 = 11;

pub fn calculate_signature(
    signing_key: &str,
    dialect: u16,
) -> String {
    if dialect == 0x0202 || dialect == 0x0210 {
        HmacSha256::new_from_slice(signing_key)
    } else {
        CmacAes128::new_from_slice(signing_key)
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
        rust_kbkdf::kbkdf(mode)
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
    rust_kbkdf::kbkdf(mode)
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
    return rust_kbkdf::kbkdf(mode);
}

/// Encyrpt message and prefix with SMB2 TransformHeader
pub fn TransformMessage(
    key: &str, 
    message: &str, 
    session_id: u32
) -> String{
    let nonce = GenerateAesCcmNonce();
    let signature;
    let encryptedMessage = EncryptMessage(key, nonce, message, sessionID, out signature);
    SMB2TransformHeader transformHeader = CreateTransformHeader(nonce, message.Length, sessionID);
    transformHeader.Signature = signature;

    byte[] buffer = new byte[SMB2TransformHeader.Length + message.Length];
    transformHeader.WriteBytes(buffer, 0);
    ByteWriter.WriteBytes(buffer, SMB2TransformHeader.Length, encryptedMessage);
    return buffer;
}

pub fn encrypt_message(
    key: &str, 
    nonce: &str, 
    message: &str, 
    session_id: u64, 
) -> String {
    let transform_header = TransformHeader {

    };
    return AesCcm.Encrypt(key, nonce, message, associatedata, SMB2TransformHeader.SignatureLength, out signature);
}

pub fn decrypt_message(
    key: &str,
    transform_header: TransformHeader, 
    encrypted_message: &str,
) -> String {
    let associatedData = transform_header.signature;
    let aesCcmNonce = transform_header.nonce;
    return DecryptAndAuthenticate(key, aesCcmNonce, encryptedMessage, associatedData, transformHeader.Signature);
}

fn generate_aes_ccm_nonce() -> String {
    byte[] aesCcmNonce = new byte[AesCcmNonceLength];
    new Random().NextBytes(aesCcmNonce);
    return aesCcmNonce;
}
