use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct PingPacket {
    timestamp: types::Long,
}

impl Packet for PingPacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}