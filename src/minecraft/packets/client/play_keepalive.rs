use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct PlayKeepAlivePacket {
    keepalive_id: i64,
}

impl PlayKeepAlivePacket {
    #[inline]
    pub fn new(keepalive_id: i64) -> Self {
        Self { keepalive_id }
    }

    pub fn get_id(&self) -> &i64 {
        &self.keepalive_id
    }
}

impl Packet for PlayKeepAlivePacket {
    const ID: i32 = 0x18;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for PlayKeepAlivePacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            keepalive_id: reader.read_long()
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for PlayKeepAlivePacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_long(self.keepalive_id);
    }
}

impl PacketRecv for PlayKeepAlivePacket {}
impl PacketSend for PlayKeepAlivePacket {}
