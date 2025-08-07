use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct CookieResponsePacket {
    pub key: types::String,
    pub payload: types::Optional<types::Array<types::UnsignedByte>>,
}

impl Packet for CookieResponsePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}