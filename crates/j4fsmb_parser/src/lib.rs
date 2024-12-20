pub mod dto;
use std::{future::Future, pin::Pin};

pub use dto::*;
pub mod header;
pub use header::*;

use async_trait::async_trait;

use crate::dto::{
    read::*,
    write::*,
    create::*,
    close::*,
};

pub trait Smb2Request {

}

pub trait Smb2Response {

}

// TODO: найти сдантартные способы для выражения From через Future<Output=...>
pub trait AsyncTryFrom<T>: Sized {
    type Error;

    fn try_from(value: T) -> Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;
}

pub enum Smb2RequestBody {
    CreateRequest(CreateRequest),
    CloseRequest(CloseRequest),
    ReadRequest(ReadRequest),
    WriteRequest(WriteRequest),
}
