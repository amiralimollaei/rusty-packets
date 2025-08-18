use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct LoginStartPacket {
    pub username: types::String,
    pub uuid: types::UUID,
}

impl Packet for LoginStartPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Login;
}