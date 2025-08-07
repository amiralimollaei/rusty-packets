use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct ChangeDifficultyPacket {
    difficulty: u8,  // 0: peaceful, 1: easy, 2: normal, 3: hard.
    is_locked: bool         
}

impl ChangeDifficultyPacket {
    #[inline]
    pub fn new(difficulty: u8, is_locked: bool) -> Self {
        Self { difficulty: difficulty, is_locked: is_locked }
    }

    pub fn get_difficulty(&self) -> u8 {
        self.difficulty
    }
}

impl Packet for ChangeDifficultyPacket {
    const ID: i32 = 0x0B;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for ChangeDifficultyPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self { difficulty: reader.read_ubyte(), is_locked: reader.read_boolean() }
    }
}

impl<T: Write + Seek> PacketOut<T> for ChangeDifficultyPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_ubyte(self.difficulty);
        writer.write_boolean(self.is_locked);
    }
}

impl PacketRecv for ChangeDifficultyPacket {}
impl PacketSend for ChangeDifficultyPacket {}
