use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct ClientBoundKeepAlivePacket {
    keepalive_id: i64,
}

impl ClientBoundKeepAlivePacket {
    #[inline]
    pub fn new(keepalive_id: i64) -> Self {
        Self { keepalive_id }
    }

    pub fn get_id(&self) -> &i64 {
        &self.keepalive_id
    }
}

impl Packet for ClientBoundKeepAlivePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ClientBoundKeepAlivePacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self::new(reader.read_long())
    }
}

impl<T: Write + Seek> PacketOut<T> for ClientBoundKeepAlivePacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_long(self.keepalive_id);
    }
}

impl PacketRecv for ClientBoundKeepAlivePacket {}
impl PacketSend for ClientBoundKeepAlivePacket {}
