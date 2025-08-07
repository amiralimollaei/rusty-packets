use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}