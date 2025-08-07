use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct PlayerAbilitiesPacket {
    flags: u8,                    // 0x01: Invulnerable, 0x02: Flying, 0x04: Allow Flying, 0x08: Creative Mode (Instant Break)	.
    flying_speed: f32,            // 0.05 by default.
    field_of_view_modifier: f32   // Modifies the field of view, like a speed potion. A Notchian server will use the same value as the movement speed sent in the Update Attributes packet, which defaults to 0.1 for players.       
}

impl PlayerAbilitiesPacket {
    #[inline]
    pub fn new(flags: u8, flying_speed: f32, field_of_view_modifier: f32) -> Self {
        Self { flags, flying_speed, field_of_view_modifier }
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn get_flying_speed(&self) -> f32 {
        self.flying_speed
    }

    pub fn get_field_of_view_modifier(&self) -> f32 {
        self.field_of_view_modifier
    }
}

impl Packet for PlayerAbilitiesPacket {
    const ID: i32 = 0x38;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for PlayerAbilitiesPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let flags = reader.read_ubyte();
        let flying_speed = reader.read_float();
        let field_of_view_modifier = reader.read_float();
        Self { flags, flying_speed, field_of_view_modifier }
    }
}

impl<T: Write + Seek> PacketOut<T> for PlayerAbilitiesPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_ubyte(self.flags);
        writer.write_float(self.flying_speed);
        writer.write_float(self.field_of_view_modifier);
    }
}

impl PacketRecv for PlayerAbilitiesPacket {}
impl PacketSend for PlayerAbilitiesPacket {}
