use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct CookieResponsePacket {
    key: types::String,
    payload: types::Optional<types::ByteArray>,
}

impl CookieResponsePacket {
    #[inline]
    pub fn new(key: String, payload: Option<&[u8]>) -> Self {
        Self {
            key: key.into(),
            payload: payload.into(),
        }
    }
}

impl Packet for CookieResponsePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Login;
}