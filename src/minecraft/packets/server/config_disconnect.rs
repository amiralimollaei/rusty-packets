use std::io::{Read, Seek, Write};

use crate::minecraft::types::NBTValue;
use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

#[derive(Debug)]
pub struct ConfigDisconnectPacket {
    reason: NBTValue,
}

impl ConfigDisconnectPacket {
    #[inline]
    pub fn new(reason: NBTValue) -> Self {
        Self { reason }
    }

    pub fn get_reason(&self) -> &NBTValue {
        &self.reason
    }
}

impl Packet for ConfigDisconnectPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ConfigDisconnectPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { reason: reader.read_nbt() }
    }
}

impl<T: Write + Seek> PacketOut<T> for ConfigDisconnectPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_nbt(self.reason.clone());
    }
}

impl PacketRecv for ConfigDisconnectPacket {}
impl PacketSend for ConfigDisconnectPacket {}
