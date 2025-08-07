use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct LoginPluginResponsePacket {
    message_id: types::VarInt,
    successful: types::Boolean,
    data: types::UnsizedByteArray,
}

impl LoginPluginResponsePacket {
    #[inline]
    pub fn new(message_id: i32, successful: bool, data: Vec<u8>) -> Self {
        Self {
            message_id: message_id.into(),
            successful: successful.into(),
            data: data.into(),
        }
    }
}

impl Packet for LoginPluginResponsePacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Login;
}
