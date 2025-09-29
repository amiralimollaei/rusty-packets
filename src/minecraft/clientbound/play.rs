use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketSerde, PacketReadable, PacketWritable},
    types,
};


#[derive(PacketSerde, Debug, Clone)]
pub struct BundleDelimiterPacket;

impl Packet for BundleDelimiterPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SpawnEntityPacket {
    pub entity_id: types::VarInt,      // A unique integer ID mostly used in the protocol to identify the entity.
    pub entity_uuid: types::UUID,      // A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
    pub entity_type: types::VarInt,    // ID in the minecraft:entity_type registry.
    pub position: types::FloatVec3,    // entity x y z position encoded as float
    pub pitch: types::Angle,           // To get the real pitch, you must divide this by (256.0F / 360.0F)
    pub yaw: types::Angle,             // To get the real yaw, you must divide this by (256.0F / 360.0F)
    pub head_yaw: types::Angle,        // Only used by living entities, where the head of the entity may differ from the general body rotation.
    pub data: types::VarInt,           // Meaning dependent on the value of the Type field, see Object Data for details.
    pub velocity_x: types::ShortVec3,  // entity x y z velocity encoded as float
}

impl Packet for SpawnEntityPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SpawnExperienceOrbPacket {
    pub entity_id: types::VarInt,      // A unique integer ID mostly used in the protocol to identify the entity.
    pub position: types::DoubleVec3,   // entity x y z position encoded as Double
    pub count: types::Short,           // The amount of experience this orb will reward once collected.
}

impl Packet for SpawnExperienceOrbPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct EntityAnimationPacket {
    pub entity_id: types::VarInt,   // A unique integer ID mostly used in the protocol to identify the entity.
    pub animation: types::UnsignedByte,    // 0->Swing main arm, 1->UNDEFINED, 2->Leave bed, 3->Swing offhand, 4->Critical effect, 5->Magic critical effect
}

impl Packet for EntityAnimationPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct AwardStatistic {
    pub category_id: types::VarInt,
    pub statistic_id: types::VarInt,
    pub value: types::VarInt,
}

#[derive(PacketSerde, Debug, Clone)]
pub struct AwardStatisticsPacket {
    pub statistics: types::Array<AwardStatistic>,
}

impl Packet for AwardStatisticsPacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct AcknowledgeBlockChangePacket {
    pub sequence_id: types::VarInt,
}

impl Packet for AcknowledgeBlockChangePacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetBlockDestroyStagePacket {
    pub entity_id: types::VarInt,
    pub location: types::Position,
    pub destroy_stage: types::Byte,
}

impl Packet for SetBlockDestroyStagePacket {
    const ID: i32 = 0x06;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct BlockEntityDataPacket {
    pub location: types::Position,
    pub type_: types::VarInt,
    pub nbt_data: types::NBTValue,
}

impl Packet for BlockEntityDataPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct BlockActionPacket {
    pub location: types::Position,
    pub action_id: types::UnsignedByte,
    pub action_parameter: types::UnsignedByte,
    pub block_type: types::VarInt,
}

impl Packet for BlockActionPacket {
    const ID: i32 = 0x08;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct BlockUpdatePacket {
    pub location: types::Position,
    pub block_id: types::VarInt,
}

impl Packet for BlockUpdatePacket {
    const ID: i32 = 0x09;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct BossBarPacket {
    pub uuid: types::UUID,
    pub action: types::VarInt,
    pub action_data: types::UnsizedByteArray // TODO: this data should be parsed based on the value of action
}

impl Packet for BossBarPacket {
    const ID: i32 = 0x0A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ChangeDifficultyPacket {
    pub difficulty: types::UnsignedByte,  // 0: peaceful, 1: easy, 2: normal, 3: hard.
    pub is_locked: types::Boolean         
}

impl Packet for ChangeDifficultyPacket {
    const ID: i32 = 0x0B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ChunkBatchFinishedPacket {
    pub batch_size: types::VarInt,
}

impl Packet for ChunkBatchFinishedPacket {
    const ID: i32 = 0x0C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ChunkBatchStartPacket;

impl Packet for ChunkBatchStartPacket {
    const ID: i32 = 0x0D;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ChunkBiomeData {
    pub chunk_z: types::Int,
    pub chunk_x: types::Int,
    pub chunk_data: types::ByteArray,
}

#[derive(PacketSerde, Debug, Clone)]
pub struct ChunkBiomesPacket {
    pub chunks: types::Array<ChunkBiomeData>,
}

impl Packet for ChunkBiomesPacket {
    const ID: i32 = 0x0E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ClearTitlesPacket {
    pub reset: types::Boolean,
}

impl Packet for ClearTitlesPacket {
    const ID: i32 = 0x0F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct CommandSuggestionMatch {
    pub match_: types::String,
    pub tooltip: types::Optional<types::NBTValue>, // optional text component
}

#[derive(PacketSerde, Debug, Clone)]
pub struct CommandSuggestionsResponsePacket {
    pub id: types::VarInt,
    pub start: types::VarInt,
    pub length: types::VarInt,
    pub matches: types::Array<CommandSuggestionMatch>,
}

impl Packet for CommandSuggestionsResponsePacket {
    const ID: i32 = 0x10;
    const PHASE: ConnectionState = ConnectionState::Play;
}

/*
#[derive(PacketSerde, Debug, Clone)]
pub struct CommandsPacket {
    pub reset: types::Array<GraphNode>, // TODO implelemnt GraphNode
    pub root_index: types::VarInt,
}

impl Packet for CommandsPacket {
    const ID: i32 = 0x11;
    const PHASE: ConnectionState = ConnectionState::Play;
}
*/

#[derive(PacketSerde, Debug, Clone)]
pub struct CloseContainerPacket {
    pub window_id: types::UnsignedByte,
}

impl Packet for CloseContainerPacket {
    const ID: i32 = 0x12;
    const PHASE: ConnectionState = ConnectionState::Play;
}

/*
#[derive(PacketSerde, Debug, Clone)]
pub struct SetContainerContentPacket {
    pub window_id: types::UnsignedByte,
    pub state_id: types::VarInt,
    pub slots: types::Array<Slot> // TODO implement Slot

}

impl Packet for SetContainerContentPacket {
    const ID: i32 = 0x13;
    const PHASE: ConnectionState = ConnectionState::Play;
}
*/


#[derive(PacketSerde, Debug, Clone)]
pub struct SetContainerPropertyPacket {
    pub window_id: types::VarInt,
    pub property: types::Short, // The meaning of the Property field depends on the type of the window.
    pub value: types::Short,
}

impl Packet for SetContainerPropertyPacket {
    const ID: i32 = 0x14;
    const PHASE: ConnectionState = ConnectionState::Play;
}

/*
#[derive(PacketSerde, Debug, Clone)]
pub struct SetContainerSlotPacket {
    pub window_id: types::UnsignedByte,
    pub state_id: types::VarInt,
    pub slot: types::Short,
    pub slot_data: Slot // TODO implement Slot

}

impl Packet for SetContainerSlotPacket {
    const ID: i32 = 0x15;
    const PHASE: ConnectionState = ConnectionState::Play;
}
*/

#[derive(PacketSerde, Debug, Clone)]
pub struct CookieRequestPacket {
    pub key: types::Identifier,
}

impl Packet for CookieRequestPacket {
    const ID: i32 = 0x16;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetCooldownPacket {
    pub item_id: types::VarInt,
    pub cooldown_ticks: types::VarInt,
}

impl Packet for SetCooldownPacket {
    const ID: i32 = 0x17;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ChatSuggestionsPacket {
    pub action: types::VarInt,   // 0: Add, 1: Remove, 2: Set
    pub entries: types::Array<types::String>,
}

impl Packet for ChatSuggestionsPacket {
    const ID: i32 = 0x18;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ClientboundPluginMessagePacket {
    pub channel: types::Identifier,
    pub data: types::UnsizedByteArray,
}

impl Packet for ClientboundPluginMessagePacket {
    const ID: i32 = 0x19;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct DamageEventPacket {
    pub entity_id: types::VarInt,
    pub source_type_id: types::VarInt,
    pub source_cause_id: types::VarInt,
    pub source_direct_id: types::VarInt,
    pub position: types::Optional<types::DoubleVec3>,
}

impl Packet for DamageEventPacket {
    const ID: i32 = 0x1A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct DebugSamplePacket {
    pub samples: types::Array<types::Long>, // Array of type-dependent samples.
}

impl Packet for DebugSamplePacket {
    const ID: i32 = 0x1B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct DeleteMessagePacket {
    pub message_id: types::VarInt,
    pub signature: types::FixedSizeByteArray<256>,
}

impl Packet for DeleteMessagePacket {
    const ID: i32 = 0x1C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct DisconnectPacket {
    pub reason: types::NBTValue,   // an NBT Tag containing a single string
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x1D;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x26;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
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


#[derive(PacketSerde, Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    pub flags: types::UnsignedByte,             // 0x01: Invulnerable, 0x02: Flying, 0x04: Allow Flying, 0x08: Creative Mode (Instant Break)	.
    pub flying_speed: types::Float,             // 0.05 by default.
    pub field_of_view_modifier: types::Float    // Modifies the field of view, like a speed potion. A Notchian server will use the same value as the movement speed sent in the Update Attributes packet, which defaults to 0.1 for players.       
}

impl Packet for PlayerAbilitiesPacket {
    const ID: i32 = 0x38;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SyncPlayerPositionPacket {
    pub location: types::Location,            // contains the location of a player
    pub flags: types::Byte,            // When the value of the this byte masked is zero the field is absolute, otherwise relative.
    pub teleport_id: types::VarInt,    // VarInt: the client should respond with the same id
}

impl Packet for SyncPlayerPositionPacket {
    const ID: i32 = 0x40;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetHeldItemPacket {
    pub slot: types::Byte,
}

impl Packet for SetHeldItemPacket {
    const ID: i32 = 0x53;
    const PHASE: ConnectionState = ConnectionState::Play;
}