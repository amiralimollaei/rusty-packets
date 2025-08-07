use std::io::{Read, Seek, Write};

use crate::minecraft::types::NBTValue;
use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct DisconnectPacket {
    reason: NBTValue,             // an NBT Tag containing a single string
}

impl DisconnectPacket {
    #[inline]
    pub fn new(reason: NBTValue) -> Self {
        Self { reason: reason }
    }

    pub fn get_reason(&self) -> &NBTValue {
        &self.reason
    }
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x1D;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for DisconnectPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { reason: reader.read_nbt() }
    }
}

impl<T: Write + Seek> PacketOut<T> for DisconnectPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_nbt(self.reason.clone());
    }
}

impl PacketRecv for DisconnectPacket {}
impl PacketSend for DisconnectPacket {}
