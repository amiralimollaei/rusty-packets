use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct ChangeDifficultyPacket {
    pub difficulty: types::UnsignedByte,  // 0: peaceful, 1: easy, 2: normal, 3: hard.
    pub is_locked: types::Boolean         
}

impl Packet for ChangeDifficultyPacket {
    const ID: i32 = 0x0B;
    const PHASE: ConnectionState = ConnectionState::Play;
}
