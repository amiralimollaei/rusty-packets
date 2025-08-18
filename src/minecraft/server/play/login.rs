use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

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