use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct Location {
    pub x: types::Double,
    pub y: types::Double,
    pub z: types::Double,
    pub yaw: types::Float,
    pub pitch: types::Float,
}

#[derive(MinecraftType, Debug, Clone)]
pub struct SyncPlayerPositionPacket {
    pub location: Location,            // contains the location of a player
    pub flags: types::Byte,            // When the value of the this byte masked is zero the field is absolute, otherwise relative.
    pub teleport_id: types::VarInt,    // VarInt: the client should respond with the same id
}

impl Packet for SyncPlayerPositionPacket {
    const ID: i32 = 0x40;
    const PHASE: ConnectionState = ConnectionState::Play;
}