use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
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