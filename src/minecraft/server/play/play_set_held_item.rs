use std::io::{Read, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketReader, PacketRecv};

#[derive(Debug)]
pub struct SetHeldItemPacket {
    slot: i8,
}

impl SetHeldItemPacket {
    #[inline]
    pub fn new(slot: i8) -> Self {
        Self { slot: slot }
    }

    pub fn get_slot(&self) -> i8 {
        self.slot
    }
}

impl Packet for SetHeldItemPacket {
    const ID: i32 = 0x53;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for SetHeldItemPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            slot: reader.read_byte(),
        }
    }
}

impl PacketRecv for SetHeldItemPacket {}
