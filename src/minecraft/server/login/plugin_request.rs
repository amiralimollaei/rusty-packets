use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend,
    PacketWriter,
};

#[derive(Debug, Clone)]
pub struct PluginRequestPacket {
    message_id: i32,
    channel: String,
    data: Vec<u8>,
}

impl PluginRequestPacket {
    #[inline]
    pub fn new(message_id: i32, channel: String, data: Vec<u8>) -> Self {
        Self {
            message_id: message_id,
            channel: channel,
            data: data,
        }
    }

    pub fn get_message_id(&self) -> i32 {
        self.message_id
    }

    pub fn get_channel(&self) -> String {
        self.channel.clone()
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl Packet for PluginRequestPacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for PluginRequestPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            message_id: reader.read_varint(),
            channel: reader.read_identifier(),
            data: reader.read_to_end()
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for PluginRequestPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.message_id);
        writer.write_identifier_str(&self.channel);
        writer.write_from_buffer(&self.data);
    }
}

impl PacketRecv for PluginRequestPacket {}
impl PacketSend for PluginRequestPacket {}
