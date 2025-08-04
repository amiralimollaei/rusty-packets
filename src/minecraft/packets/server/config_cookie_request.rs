use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

#[derive(Debug)]
pub struct ConfigCookieRequest {
    key: String,
}

impl ConfigCookieRequest {
    #[inline]
    pub fn new(key: String) -> Self {
        Self { key: key }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}


impl Packet for ConfigCookieRequest {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ConfigCookieRequest {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { key: reader.read_string() }
    }
}

impl<T: Write + Seek> PacketOut<T> for ConfigCookieRequest {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_str(self.key.as_str());
    }
}

impl PacketRecv for ConfigCookieRequest {}
impl PacketSend for ConfigCookieRequest {}
