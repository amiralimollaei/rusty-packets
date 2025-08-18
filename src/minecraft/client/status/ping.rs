use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct PingPacket {
    pub timestamp: types::Long,
}

impl Packet for PingPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Status;
}