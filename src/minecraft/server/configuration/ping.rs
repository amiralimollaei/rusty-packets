use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct PingPacket {
    pub timestamp: types::Int,
}

impl Packet for PingPacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}