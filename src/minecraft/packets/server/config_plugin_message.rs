use std::io::{Read, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketReader, PacketRecv};

#[derive(Debug)]
pub struct ConfigPluginMessagesPacket {
    channel: String,
    data: Vec<u8>,
}

impl ConfigPluginMessagesPacket {
    #[inline]
    pub fn new(channel: String, data: Vec<u8>) -> Self {
        Self { channel: channel, data:data }
    }

    pub fn get_channel(&self) -> String {
        self.channel.clone()
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl Packet for ConfigPluginMessagesPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ConfigPluginMessagesPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            channel: 
            reader.read_identifier(),
            data: reader.read_to_end()
        }
    }
}

impl PacketRecv for ConfigPluginMessagesPacket {}
