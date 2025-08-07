use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct LoginStartPacket {
    username: types::String,
    uuid: types::UUID,
}

impl LoginStartPacket {
    #[inline]
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username: username.into(),
            uuid: uuid.into(),
        }
    }
}

impl Packet for LoginStartPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Login;
}