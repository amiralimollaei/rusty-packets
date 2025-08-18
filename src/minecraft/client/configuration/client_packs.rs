use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct KnownClientPack {
    pub namespace: types::String,
    pub id: types::String,
    pub version: types::String,
}

#[derive(MinecraftType, Clone, Debug)]
pub struct KnownClientPacksPacket {
    pub packs: types::Array<KnownClientPack>,
}

impl Packet for KnownClientPacksPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}