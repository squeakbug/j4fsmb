use std::{
    io::{Read, Write}, // F*cking extension methods: hello C#!
    net::TcpStream, sync::{Arc, Mutex}
};

use deku::{DekuContainerRead, DekuContainerWrite, DekuWriter};
use j4fsmb_parser::{close::CloseResponse, create::CreateResponse, read::{ReadRequest, ReadResponse}, write::WriteResponse, Command, Smb2Request, Smb2Response, Smb2SyncHeader};

use crate::Response;

pub struct Request {
    client: Arc<Mutex<TcpStream>>,
    buff: Vec<u8>,
}

pub struct RequestBuilder {
    client: Arc<Mutex<TcpStream>>,
    header: Smb2SyncHeader,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn new(client: Arc<Mutex<TcpStream>>) -> Self {
        RequestBuilder {
            client,
            header: Smb2SyncHeader::default(),
            body: None,
        }
    }

    pub fn request_body(
        mut self, 
        body: impl Smb2Request + DekuContainerWrite
    ) -> anyhow::Result<RequestBuilder> {
        self.body = Some(body.to_bytes()?);
        Ok(self)
    }

    pub fn command(mut self, cmd: Command) -> RequestBuilder {
        self.header.command = cmd as u16;
        self
    }

    pub fn build(self) -> anyhow::Result<Request> {
        let mut body = self.body.ok_or(anyhow::anyhow!("Body are not provided"))?;
        let mut header = self.header.to_bytes()?;

        let mut buff = Vec::new();
        buff.append(&mut header);
        buff.append(&mut body);

        Ok(Request {
            client: self.client.clone(),
            buff,
        })
    }
}

// Алгоритм чтения из потока байти типизированного объекта переменной длины

impl Request {
    // TODO: reimplement with streams
    pub fn send(self) -> anyhow::Result<()> {
        let mut stream = self.client.lock().unwrap_or_else(|poisoned| {
            println!("Lock was poisoned; recovering...");
            poisoned.into_inner()
        });
        stream.write_all(&self.buff)?;
        stream.flush()?;

        let (_read, header) = Smb2SyncHeader::from_reader((&mut stream, 0))?;

        // Hey, use slab cache
        match header.command {
            SMB2_COMMAND_READ => {
                let (_read, body) = ReadResponse::from_reader((&mut stream, 0))?;
            },
            _ => panic!("Boy...")
        }

        todo!()
    }
}
