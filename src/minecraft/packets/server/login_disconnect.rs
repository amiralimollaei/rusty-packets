use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

#[derive(Debug)]
pub struct LoginDisconnectPacket {
    reason: String,
}

impl LoginDisconnectPacket {
    #[inline]
    pub fn new(reason: String) -> Self {
        Self { reason: reason }
    }

    pub fn get_reason(&self) -> &String {
        &self.reason
    }
}

impl Packet for LoginDisconnectPacket {
    const ID: i32 = 0;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for LoginDisconnectPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { reason: reader.read_string() }
    }
}

impl<T: Write + Seek> PacketOut<T> for LoginDisconnectPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_str(self.reason.as_str());
    }
}

impl PacketRecv for LoginDisconnectPacket {}
impl PacketSend for LoginDisconnectPacket {}
