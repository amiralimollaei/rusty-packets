use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct CookieRequestPacket {
    pub key: types::String,
}

impl Packet for CookieRequestPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}