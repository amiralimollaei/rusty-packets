use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct LoginPluginResponsePacket {
    pub message_id: types::VarInt,
    pub successful: types::Boolean,
    pub data: types::UnsizedByteArray,
}

impl Packet for LoginPluginResponsePacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Login;
}
