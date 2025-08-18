use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x26;
    const PHASE: ConnectionState = ConnectionState::Play;
}