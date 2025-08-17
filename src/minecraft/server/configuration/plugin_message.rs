use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct PluginMessagesPacket {
    pub channel: types::Identifier,
    pub data: types::UnsizedByteArray,
}

impl Packet for PluginMessagesPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}