use minecraft_type_derive::MinecraftType;
use crate::minecraft::types::MinecraftType;

use crate::minecraft::packets::{ConnectionState, Packet, PacketRecv, PacketSend};
use crate::minecraft::types;
use crate::utils::{PacketReadable, PacketWritable};

const UNWRAP_ERROR: &str = "RegistryDataPacket: Unexpected error while reading value.";

#[derive(MinecraftType, Debug, Clone)]
pub struct RegistryEntry {
    id: types::Identifier,
    data: types::Optional<types::NBTValue>
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