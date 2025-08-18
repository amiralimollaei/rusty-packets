use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: types::String,
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Login;
}