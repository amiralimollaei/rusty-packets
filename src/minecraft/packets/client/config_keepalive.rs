use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct ServerBoundKeepAlivePacket {
    keepalive_id: i64,
}

impl ServerBoundKeepAlivePacket {
    #[inline]
    pub fn new(keepalive_id: i64) -> Self {
        Self { keepalive_id }
    }

    pub fn get_id(&self) -> &i64 {
        &self.keepalive_id
    }
}

impl Packet for ServerBoundKeepAlivePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ServerBoundKeepAlivePacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            keepalive_id: reader.read_long()
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for ServerBoundKeepAlivePacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_long(self.keepalive_id);
    }
}

impl PacketRecv for ServerBoundKeepAlivePacket {}
impl PacketSend for ServerBoundKeepAlivePacket {}
