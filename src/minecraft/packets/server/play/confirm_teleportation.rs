use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct ConfirmTeleportationPacket {
    teleport_id: i32, // VarInt: should be the same as sent by server
}

impl ConfirmTeleportationPacket {
    #[inline]
    pub fn new(teleport_id: i32) -> Self {
        Self { teleport_id }
    }

    pub fn get_teleport_id(&self) -> i32 {
        self.teleport_id
    }
}

impl Packet for ConfirmTeleportationPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for ConfirmTeleportationPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            teleport_id: reader.read_varint(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for ConfirmTeleportationPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.teleport_id);
    }
}

impl PacketRecv for ConfirmTeleportationPacket {}
impl PacketSend for ConfirmTeleportationPacket {}
