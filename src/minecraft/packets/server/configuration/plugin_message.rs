use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv,
};

#[derive(Debug)]
pub struct PluginMessagesPacket {
    channel: String,
    data: Vec<u8>,
}

impl PluginMessagesPacket {
    #[inline]
    pub fn new(channel: String, data: Vec<u8>) -> Self {
        Self {
            channel: channel,
            data: data,
        }
    }

    pub fn get_channel(&self) -> String {
        self.channel.clone()
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
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
