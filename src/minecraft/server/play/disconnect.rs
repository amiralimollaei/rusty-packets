use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: types::NBTValue,             // an NBT Tag containing a single string
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x1D;
    const PHASE: ConnectionState = ConnectionState::Play;
}
