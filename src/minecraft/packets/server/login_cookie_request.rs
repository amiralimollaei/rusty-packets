use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct LoginCookieRequest {
    key: String,
}

impl LoginCookieRequest {
    #[inline]
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}

impl Packet for LoginCookieRequest {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for LoginCookieRequest {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            key: reader.read_identifier(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for LoginCookieRequest {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_identifier_str(self.key.as_str());
    }
}

impl PacketRecv for LoginCookieRequest {}
impl PacketSend for LoginCookieRequest {}
