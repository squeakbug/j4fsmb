use std::{future::Future, pin::Pin, process::Output};

use deku::prelude::*;
use tokio::io::{AsyncReadExt, AsyncRead};
use async_trait::async_trait;

use crate::{AsyncTryFrom, Smb2Request};

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct CloseRequest {
    pub structure_size: u16,
    pub flags: u16,
    pub reserved: u32,
    pub file_id: u128,
}

impl Smb2Request for CloseRequest { }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_try_from() {
        let input: &[u8] = &[0xffu8; 24];
        let request: CloseRequest = CloseRequest::try_from(input).unwrap();

        assert_eq!(request.structure_size, u16::MAX);
        assert_eq!(request.flags, u16::MAX);
        assert_eq!(request.reserved, u32::MAX);
        assert_eq!(request.file_id, u128::MAX);
    }
}
