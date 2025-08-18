use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct CookieRequest {
    pub key: types::Identifier,
}

impl Packet for CookieRequest {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Login;
}