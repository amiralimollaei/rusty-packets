use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct KnownServerPack {
    pub namespace: types::String,
    pub id: types::String,
    pub version: types::String,
}

#[derive(MinecraftType, Clone, Debug)]
pub struct KnownServerPacksPacket {
    pub packs: types::Array<KnownServerPack>,
}

impl Packet for KnownServerPacksPacket {
    const ID: i32 = 0x0E;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}