use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

#[derive(Debug)]
pub struct CookieRequestPacket {
    key: String,
}

impl CookieRequestPacket {
    #[inline]
    pub fn new(key: String) -> Self {
        Self { key: key }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}


impl Packet for CookieRequestPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for CookieRequestPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { key: reader.read_string() }
    }
}

impl<T: Write + Seek> PacketOut<T> for CookieRequestPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_str(self.key.as_str());
    }
}

impl PacketRecv for CookieRequestPacket {}
impl PacketSend for CookieRequestPacket {}
