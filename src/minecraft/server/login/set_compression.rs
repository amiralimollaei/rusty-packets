use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct SetCompressionPacket {
    pub threshold: types::VarInt,
}

impl Packet for SetCompressionPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Login;
}