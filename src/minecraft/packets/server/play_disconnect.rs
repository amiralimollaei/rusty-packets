use std::io::{Read, Seek, Write};

use crate::minecraft::types::NBTValue;
use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct PlayDisconnectPacket {
    reason: NBTValue,             // an NBT Tag containing a single string
}

impl PlayDisconnectPacket {
    #[inline]
    pub fn new(reason: NBTValue) -> Self {
        Self { reason: reason }
    }

    pub fn get_reason(&self) -> &NBTValue {
        &self.reason
    }
}

impl Packet for PlayDisconnectPacket {
    const ID: i32 = 0x1D;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for PlayDisconnectPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { reason: reader.read_nbt() }
    }
}

impl<T: Write + Seek> PacketOut<T> for PlayDisconnectPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_nbt(self.reason.clone());
    }
}

impl PacketRecv for PlayDisconnectPacket {}
impl PacketSend for PlayDisconnectPacket {}
