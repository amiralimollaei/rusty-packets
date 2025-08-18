use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct DisconnectPacket {
    pub reason: types::NBTValue,
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}