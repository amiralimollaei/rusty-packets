use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct PluginRequestPacket {
    pub message_id: types::VarInt,
    pub channel: types::Identifier,
    pub data: types::UnsizedByteArray,
}

impl Packet for PluginRequestPacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Login;
}
