use std::io::{Read, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketReader, PacketRecv};

#[derive(Debug)]
pub struct PlaySetHeldItemPacket {
    slot: i8,
}

impl PlaySetHeldItemPacket {
    #[inline]
    pub fn new(slot: i8) -> Self {
        Self { slot: slot }
    }

    pub fn get_slot(&self) -> i8 {
        self.slot
    }
}

impl Packet for PlaySetHeldItemPacket {
    const ID: i32 = 0x53;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for PlaySetHeldItemPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            slot: reader.read_byte(),
        }
    }
}

impl PacketRecv for PlaySetHeldItemPacket {}
