use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::packets::{ConnectionState, Packet};
use crate::minecraft::types;
use crate::minecraft::types::MinecraftType;

#[derive(MinecraftType, Debug, Clone)]
pub struct RegistryEntry {
    pub id: types::Identifier,
    pub data: types::Optional<types::NBTValue>
}

#[derive(MinecraftType, Debug, Clone)]
pub struct RegistryDataPacket {
    pub registry_id: types::Identifier,
    pub entries: types::Array<RegistryEntry>,
}

impl Packet for RegistryDataPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}