use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct LoginProperty {
    pub name: types::String,
    pub value: types::String,
    pub signature: types::Optional<types::String>,
}

#[derive(MinecraftType, Debug, Clone)]
pub struct LoginSuccessPacket {
    pub uuid: types::UUID,
    pub username: types::String,
    pub properties: types::Array<LoginProperty>,
}

impl Packet for LoginSuccessPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Login;
}