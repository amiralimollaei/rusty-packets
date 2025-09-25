use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};


#[derive(MinecraftType, Debug, Clone)]
pub struct BundleDelimiterPacket;

impl Packet for BundleDelimiterPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct SpawnEntityPacket {
    pub entity_id: types::VarInt,    // A unique integer ID mostly used in the protocol to identify the entity.
    pub entity_uuid: types::UUID,    // A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
    pub entity_type: types::VarInt,  // ID in the minecraft:entity_type registry.
    pub x: types::Float,             // entity x position
    pub y: types::Float,             // entity y position
    pub z: types::Float,             // entity z position
    pub pitch: types::Angle,         // To get the real pitch, you must divide this by (256.0F / 360.0F)
    pub yaw: types::Angle,           // To get the real yaw, you must divide this by (256.0F / 360.0F)
    pub head_yaw: types::Angle,      // Only used by living entities, where the head of the entity may differ from the general body rotation.
    pub data: types::VarInt,         // Meaning dependent on the value of the Type field, see Object Data for details.
    pub velocity_x: types::Short,    // Same units as Set Entity Velocity.
    pub velocity_y: types::Short,    // Same units as Set Entity Velocity.
    pub velocity_z: types::Short,    // Same units as Set Entity Velocity.
}

impl Packet for SpawnEntityPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct SpawnExperienceOrbPacket {
    pub entity_id: types::VarInt,   // A unique integer ID mostly used in the protocol to identify the entity.
    pub x: types::Double,           // entity x position
    pub y: types::Double,           // entity y position
    pub z: types::Double,           // entity z position
    pub count: types::Short,        // The amount of experience this orb will reward once collected.
}

impl Packet for SpawnExperienceOrbPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct EntityAnimationPacket {
    pub entity_id: types::VarInt,   // A unique integer ID mostly used in the protocol to identify the entity.
    pub animation: types::UnsignedByte,    // 0->Swing main arm, 1->UNDEFINED, 2->Leave bed, 3->Swing offhand, 4->Critical effect, 5->Magic critical effect
}

impl Packet for EntityAnimationPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct ChangeDifficultyPacket {
    pub difficulty: types::UnsignedByte,  // 0: peaceful, 1: easy, 2: normal, 3: hard.
    pub is_locked: types::Boolean         
}

impl Packet for ChangeDifficultyPacket {
    const ID: i32 = 0x0B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: types::NBTValue,             // an NBT Tag containing a single string
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x1D;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x26;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct LoginPacket {
    pub entity_id: types::Int,                                        // The player's Entity ID (EID).
    pub is_harcore: types::Boolean,
    pub dimensions: types::Array<types::String>,                      // Identifiers for all dimensions on the server.
    pub max_players: types::VarInt,                                   // Was once used by the client to draw the player list, but now is ignored.
    pub view_distance: types::VarInt,                                 // Render distance (2-32).
    pub simulation_distance: types::VarInt,                           // The distance that the client will process specific things, such as entities.
    pub reduced_debug_info: types::Boolean,                           // If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
    pub enable_respawn_screen: types::Boolean,                        // Set to false when the doImmediateRespawn gamerule is true.
    pub do_limited_crafting: types::Boolean,                          // Whether players can only craft recipes they have already unlocked. Currently unused by the client.
    pub dimension_type: types::VarInt,                                // The ID of the type of dimension in the `minecraft:dimension_type` registry, defined by the Registry Data packet.
	pub dimension_name: types::String,                                // Name of the dimension being spawned into.
    pub hashed_seed: types::Long,                                     // First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
    pub game_mode: types::UnsignedByte,                               // 0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
	pub previous_game_mode: types::Byte,                              // -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed)
    pub is_debug: types::Boolean,                                     // True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks.
    pub is_flat: types::NBTValue,                                     // True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63.
    pub death_dimension_name: types::Optional<types::Identifier>,     // Name of the dimension the player died in.
    pub death_location: types::Optional<types::Position>,             // The location that the player died at.
    pub portal_cooldown: types::VarInt,                               // The number of ticks until the player can use the portal again.
    pub enforces_secure_chat: types::Boolean
}

impl Packet for LoginPacket {
    const ID: i32 = 0x2B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    pub flags: types::UnsignedByte,             // 0x01: Invulnerable, 0x02: Flying, 0x04: Allow Flying, 0x08: Creative Mode (Instant Break)	.
    pub flying_speed: types::Float,             // 0.05 by default.
    pub field_of_view_modifier: types::Float    // Modifies the field of view, like a speed potion. A Notchian server will use the same value as the movement speed sent in the Update Attributes packet, which defaults to 0.1 for players.       
}

impl Packet for PlayerAbilitiesPacket {
    const ID: i32 = 0x38;
    const PHASE: ConnectionState = ConnectionState::Play;
}


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


#[derive(MinecraftType, Debug, Clone)]
pub struct SetHeldItemPacket {
    pub slot: types::Byte,
}

impl Packet for SetHeldItemPacket {
    const ID: i32 = 0x53;
    const PHASE: ConnectionState = ConnectionState::Play;
}