use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::minecraft::packets::PacketRecv;
use crate::minecraft::packets::PacketSend;
use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketWriter,
};

#[derive(Clone)]
pub struct CookieResponsePacket {
    key: String,
    payload: Option<Vec<u8>>,
}

impl CookieResponsePacket {
    #[inline]
    pub fn new(key: String, payload: Option<Vec<u8>>) -> Self {
        Self {
            key: key,
            payload: payload,
        }
    }
}

impl Packet for CookieResponsePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for CookieResponsePacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let key = reader.read_identifier();
        let payload = if reader.read_boolean() {
            Some(reader.read_ubyte_array())
        } else {
            None
        };
        Self {
            key: key,
            payload: payload,
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for CookieResponsePacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_identifier_str(&self.key);
        writer.write_boolean(self.payload.is_some());
        if let Some(v) = &self.payload {
            writer.write_ubyte_array(v.clone());
        }
    }
}
impl PacketRecv for CookieResponsePacket {}
impl PacketSend for CookieResponsePacket {}
