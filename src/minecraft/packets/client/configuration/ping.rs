use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct PingPacket {
    timestamp: i64,
}

impl PingPacket {
    #[inline]
    pub fn new(timestamp: i64) -> Self {
        Self {
            timestamp: timestamp,
        }
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl Packet for PingPacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for PingPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            timestamp: reader.read_long(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for PingPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_long(self.timestamp);
    }
}

impl PacketRecv for PingPacket {}
impl PacketSend for PingPacket {}
