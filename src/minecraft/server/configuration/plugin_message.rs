use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv,
};

#[derive(Debug)]
pub struct PluginMessagesPacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for PluginMessagesPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for PluginMessagesPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            channel: reader.read_identifier(),
            data: reader.read_to_end(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for PluginMessagesPacket {
    fn write(&self, writer: &mut crate::minecraft::packets::PacketWriter<T>) {
        writer.write_identifier_str(&self.channel);
        writer.write_from_buffer(&self.data);
    }
}

impl PacketRecv for PluginMessagesPacket {}
