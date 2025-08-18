use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct AddResourcePackPacket {
    pub uuid: types::UUID,
    pub url: types::String,
    pub hash: types::String,
    pub forced: types::Boolean,
    pub prompt_message: types::Optional<types::NBTValue>,
}


impl Packet for AddResourcePackPacket {
    const ID: i32 = 0x09;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}