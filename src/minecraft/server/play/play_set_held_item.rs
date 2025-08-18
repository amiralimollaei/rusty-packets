use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct SetHeldItemPacket {
    pub slot: types::Byte,
}

impl Packet for SetHeldItemPacket {
    const ID: i32 = 0x53;
    const PHASE: ConnectionState = ConnectionState::Play;
}