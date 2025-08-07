use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::minecraft::packets::PacketRecv;
use crate::minecraft::packets::PacketSend;
use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketWriter,
};

#[derive(Clone)]
pub struct LoginPluginResponsePacket {
    message_id: i32,
    successful: bool,
    data: Vec<u8>,
}

impl LoginPluginResponsePacket {
    #[inline]
    pub fn new(message_id: i32, successful: bool, data: Vec<u8>) -> Self {
        Self {
            message_id: message_id,
            successful: successful,
            data: data,
        }
    }
}

impl Packet for LoginPluginResponsePacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for LoginPluginResponsePacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let message_id = reader.read_varint();
        let successful = reader.read_boolean();
        let data = reader.read_to_end();
        Self {
            message_id: message_id,
            successful: successful,
            data: data,
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for LoginPluginResponsePacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.message_id);
        writer.write_boolean(self.successful);
        writer.write_from_buffer(self.data.as_slice());
    }
}


impl PacketRecv for LoginPluginResponsePacket {}
impl PacketSend for LoginPluginResponsePacket {}
