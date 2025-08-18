use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};


#[derive(MinecraftType, Clone, Debug)]
pub struct RemoveResourcePackPacket {
    pub uuid: types::Optional<types::UUID>,
}

impl Packet for RemoveResourcePackPacket {
    const ID: i32 = 0x08;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}