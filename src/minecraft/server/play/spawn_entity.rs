use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Debug, Clone)]
pub struct SpawnEntityPacket {
    pub entity_id: types::VarInt,    // VarInt: A unique integer ID mostly used in the protocol to identify the entity.
    pub entity_uuid: types::UUID, // A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
    pub entity_type: types::VarInt,  // VarInt: ID in the minecraft:entity_type registry.
    pub x: types::Float,            // Double: entity x position
    pub y: types::Float,            // Double: entity y position
    pub z: types::Float,            // Double: entity z position
    pub pitch: types::Angle,         // To get the real pitch, you must divide this by (256.0F / 360.0F)
    pub yaw: types::Angle,           // To get the real yaw, you must divide this by (256.0F / 360.0F)
    pub head_yaw: types::Angle,      // Only used by living entities, where the head of the entity may differ from the general body rotation.
    pub data: types::VarInt,         // Meaning dependent on the value of the Type field, see Object Data for details.
    pub velocity_x: types::Short,   // Same units as Set Entity Velocity.
    pub velocity_y: types::Short,   // Same units as Set Entity Velocity.
    pub velocity_z: types::Short,   // Same units as Set Entity Velocity.
}

impl Packet for SpawnEntityPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Play;
}