use packet_serde_derive::PacketSerde;
use serde::de;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketSerde, PacketWritable},
    types::{self, Array, NBTValue},
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


#[derive(PacketSerde, Debug, Clone)]
pub struct SetContainerContentPacket {
    pub window_id: types::UnsignedByte,
    pub state_id: types::VarInt,
    pub slots: types::Array<types::Slot>,
    pub carriedi_item: types::Slot  // Item being dragged with the mouse.
}

impl Packet for SetContainerContentPacket {
    const ID: i32 = 0x13;
    const PHASE: ConnectionState = ConnectionState::Play;
}


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


#[derive(PacketSerde, Debug, Clone)]
pub struct SetContainerSlotPacket {
    pub window_id: types::UnsignedByte,
    pub state_id: types::VarInt,
    pub slot: types::Short,  // The slot that should be updated.
    pub slot_data: types::Slot
}

impl Packet for SetContainerSlotPacket {
    const ID: i32 = 0x15;
    const PHASE: ConnectionState = ConnectionState::Play;
}


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
pub struct DisguisedChatMessagePacket {
    pub message: types::NBTValue,      // Text Component: This is used as the content parameter when formatting the message on the client.
    pub chat_type: types::VarInt,      // The type of chat in the minecraft:chat_type registry, defined by the Registry Data packet.
    pub sender_name: types::NBTValue,  // This is used as the sender parameter when formatting the message on the client.
    pub target_name: types::Optional<types::NBTValue>
}

impl Packet for DisguisedChatMessagePacket {
    const ID: i32 = 0x1E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct EntityEventPacket {
    pub entity_id: types::Int,
    pub entity_status: types::Byte
}

impl Packet for EntityEventPacket {
    const ID: i32 = 0x1F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub enum VibrationPositionSourceEnum {
    Block {
        position: types::Position  // The position of the block the vibration originated from.
    },
    Entity {
        id: types::VarInt,        // The ID of the entity the vibration originated from. 
        eye_height: types::Float  // The height of the entity's eye relative to the entity. 
    }
}


#[derive(PacketSerde, Debug, Clone)]
pub enum ParticleEnum {
    AngryVillager,
    Block {
        block_state: types::VarInt  // The ID of the block state.
    },
    BlockMarker {
        block_state: types::VarInt  // The ID of the block state.
    },
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust {
        red: types::Float,    // The red RGB value, between 0 and 1. Divide actual RGB value by 255.
        green: types::Float,  // The green RGB value, between 0 and 1. Divide actual RGB value by 255.
        blue: types::Float,   // The blue RGB value, between 0 and 1. Divide actual RGB value by 255.
        scale: types::Float,  // The scale, will be clamped between 0.01 and 4.
    },
    DustColorTransition {
        from_red: types::Float,    // The start red RGB value, between 0 and 1. Divide actual RGB value by 255.
        from_green: types::Float,  // The start green RGB value, between 0 and 1. Divide actual RGB value by 255.
        from_blue: types::Float,   // The start blue RGB value, between 0 and 1. Divide actual RGB value by 255.
        to_red: types::Float,      // The end red RGB value, between 0 and 1. Divide actual RGB value by 255.
        to_green: types::Float,    // The end green RGB value, between 0 and 1. Divide actual RGB value by 255.
        to_blue: types::Float,     // The end blue RGB value, between 0 and 1. Divide actual RGB value by 255.
        scale: types::Float,       // The scale, will be clamped between 0.01 and 4.
    },
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect {
        color: types::Int,  // The ARGB components of the color encoded as an Int
    },
    ExplotionEmitter,
    Explosion,
    Gust,
    SmallGust,
    GustEmitterLarge,
    GustEmitterSmall,
    SonicBoom,
    FallingDust {
        block_state: types::VarInt  // The ID of the block state.
    },
    Firework,
    Fishing,
    Flame,
    Infested,
    CherryLeaves,
    SculkSoul,
    SculkCharge {
        roll: types::Float  // How much the particle will be rotated when displayed.
    },
    SculkChargePop,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item {
        item: types::Slot  // The item that will be used.
    },
    Vibration {
        position_source: VibrationPositionSourceEnum,  // the vibration source
        ticks: types::VarInt  // The amount of ticks it takes for the vibration to travel from its source to its destination.
    },
    ItemSlime,
    ItemCobweb,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    WhiteSmoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
    FallingSporeBlossom,
    Ash,
    CrimsonSpore,
    WarpedSpore,
    SporeBlossomAir,
    DrippingObsidianTear,
    FallingObsidianTear,
    LandingObsidianTear,
    ReversePortal,
    WhiteAsh,
    SmallFlame,
    SnowFlake,
    DrippingDripstoneLava,
    FallingDripstoneLava,
    DrippingDripstoneWater,
    FallingDripstoneWater,
    GlowSquidInk,
    Glow,
    WaxOn,
    WaxOff,
    ElectricSpark,
    Scrape,
    Shriek {
        delay: types::VarInt // The time in ticks before the particle is displayed
    },
    EggCrack,
    DustPlume,
    TrialSpawnerDetection,
    TrialSpawnerDetectionOminous,
    VaultConnection,
    DustPillar {
        block_state: types::VarInt  // The ID of the block state.
    },
    OminousSpawning,
    RaidOmen,
    TrialOmen,
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ExplosionPacket {
    pub position: types::DoubleVec3,
    // If the strength is greater or equal to 2.0, or the block interaction is not 0 (keep),
    // large explosion particles are used. Otherwise, small explosion particles are used.
    pub strength: types::Float,
    // Each record is 3 signed bytes long; the 3 bytes are the XYZ (respectively) signed offsets of affected blocks.
    pub records: types::Array<types::ByteVec3>,
    pub player_motion: types::FloatVec3, // velocity of the player being pushed by the explosion.
    pub block_interaction: types::VarInt,
    pub small_explosion_particle: ParticleEnum,
    pub large_explosion_particle: ParticleEnum,
    pub explotion_sound: types::IdOr<types::SoundEvent>,
}

impl Packet for ExplosionPacket {
    const ID: i32 = 0x20;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct UnloadChunkPacket {
    pub chunk_x: types::Int,  // Block coordinate divided by 16 (rounded down)
    pub chunk_z: types::Int,  // Block coordinate divided by 16 (rounded down)
}

impl Packet for UnloadChunkPacket {
    const ID: i32 = 0x21;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct GameEventPacket {
    pub event: types::UnsignedByte,
    pub value: types::Float,
}

impl Packet for GameEventPacket {
    const ID: i32 = 0x22;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct OpenHorseScreenPacket {
    pub window_id: types::UnsignedByte,
    pub slot_count: types::VarInt,
    pub entity_id: types::Int,
}

impl Packet for OpenHorseScreenPacket {
    const ID: i32 = 0x23;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct HurtAnimationPacket {
    pub entity_id: types::Int,
    pub yaw: types::Float,
}

impl Packet for HurtAnimationPacket {
    const ID: i32 = 0x24;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct InitializeWorldBorderPacket {
    pub x: types::Double,
    pub z: types::Double,
    pub old_diameter: types::Double,   // Current length of a single side of the world border, in meters.
    pub new_diameter: types::Double,   // Target length of a single side of the world border, in meters.
    // Number of real-time milliseconds until New Diameter is reached. It appears that Notchian server does
    // not sync world border speed to game ticks, so it gets out of sync with server lag. If the world border
    // is not moving, this is set to 0.
    pub speed: types::VarLong,
    pub portal_teleport_boundary: types::VarInt,
    pub warning_blocks: types::VarInt,  // In meters.
    pub warning_time: types::VarInt,    // In seconds as set by /worldborder warning time.
}

impl Packet for InitializeWorldBorderPacket {
    const ID: i32 = 0x25;
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
pub struct BlockEntityData {
    // The X and Z coordinates of the block entity, packed into a single byte.
    // The X coordinate is stored in the upper 4 bits, and the Z coordinate is stored in the lower 4 bits.
    pub packed_xz: types::UnsignedByte, 
    pub y: types::UnsignedShort,  // The height relative to the world
    pub type_: types::VarInt,     // The type of block entity
    pub data: types::NBTValue,    // The block entity's data, without the X, Y, and Z values
}

#[derive(PacketSerde, Debug, Clone)]
pub struct ChunkDataAndUpdateLightPacket {
    pub chunk_x: types::Int,  // Block coordinate divided by 16 (rounded down)
    pub chunk_z: types::Int,  // Block coordinate divided by 16 (rounded down)
    pub heightmaps: types::NBTValue,
    pub data: types::ByteArray,
    pub block_entities: types::Array<BlockEntityData>,
    pub sky_light_mask: types::BitSet,
    pub block_light_mask: types::BitSet,
    pub empty_sky_light_mask: types::BitSet,
    pub empty_block_light_mask: types::BitSet,
    pub sky_light_arrays: types::Array<types::ByteArray>,
    pub block_light_arrays: types::Array<types::ByteArray>,
}

impl Packet for ChunkDataAndUpdateLightPacket {
    const ID: i32 = 0x27;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct WorldEventPacket {
    pub event: types::Int,
    pub position: types::Position,
    pub data: types::Int,
    pub disable_relative_volume: types::Boolean
}

impl Packet for WorldEventPacket {
    const ID: i32 = 0x28;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ParticlePacket {
    pub long_distance: types::Boolean,  // If true, particle distance increases from 256 to 65536.
    pub position: types::DoubleVec3,
    pub offset: types::FloatVec3,       // This is added to the X position after being multiplied by random.nextGaussian().
    pub max_speed: types::Float,
    pub particle_count: types::Int,     // The number of particles to create.
    pub disable_relative_volume: types::Boolean,
    pub particle: ParticleEnum,
}

impl Packet for ParticlePacket {
    const ID: i32 = 0x29;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct UpdateLightPacket {
    pub chunk_x: types::VarInt,
    pub chunk_z: types::VarInt,
    pub sky_light_mask: types::BitSet,
    pub block_light_mask: types::BitSet,
    pub empty_sky_light_mask: types::BitSet,
    pub empty_block_light_mask: types::BitSet,
    pub sky_light_arrays: types::Array<types::ByteArray>,
    pub block_light_arrays: types::Array<types::ByteArray>,
}

impl Packet for UpdateLightPacket {
    const ID: i32 = 0x2A;
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
pub struct MapIcon {
    pub type_: types::VarInt,
    pub x: types::Byte,  // Map coordinates: -128 for furthest left, +127 for furthest right
    pub z: types::Byte,  // Map coordinates: -128 for highest, +127 for lowest
    pub direction: types::Byte,
    pub display_name: types::Optional<types::NBTValue>  // Optional Text Component
}

#[derive(Debug, Clone)]
pub struct MapColorPatch {
    pub columns: types::UnsignedByte,  // Number of columns updated
    pub rows: Option<types::UnsignedByte>,     // Only if Columns is more than 0; number of rows updated
    pub x: Option<types::UnsignedByte>,        // Only if Columns is more than 0; x offset of the westernmost column
    pub z: Option<types::UnsignedByte>,        // Only if Columns is more than 0; z offset of the northernmost row
    pub data: Option<types::Array<types::ByteArray>>,  // Only if Columns is more than 0
}

impl PacketReadable for MapColorPatch {
    fn read(stream: &mut impl std::io::Read) -> Self {
        let columns = types::UnsignedByte::read(stream);
        if columns.get_value() > 0 {
            let rows = types::UnsignedByte::read(stream);
            let x = types::UnsignedByte::read(stream);
            let z = types::UnsignedByte::read(stream);
            let data = types::Array::<types::ByteArray>::read(stream);
            MapColorPatch {
                columns,
                rows: Some(rows),
                x: Some(x),
                z: Some(z),
                data: Some(data),
            }
        } else {
            MapColorPatch {
                columns,
                rows: None,
                x: None,
                z: None,
                data: None,
            }
        }
    }
}

impl PacketWritable for MapColorPatch {
    fn write(&self, stream: &mut impl std::io::Write) {
        self.columns.write(stream);
        if self.columns.get_value() > 0 {
            self.rows.as_ref().unwrap().write(stream);
            self.x.as_ref().unwrap().write(stream);
            self.z.as_ref().unwrap().write(stream);
            self.data.as_ref().unwrap().write(stream);
        }
    }
}

impl PacketSerde for MapColorPatch {}

#[derive(PacketSerde, Debug, Clone)]
pub struct MapDataPacket {
    pub map_id: types::VarInt,
    pub scale: types::Byte,
    pub locked: types::Boolean,
    pub icons: types::Optional<types::Array<MapIcon>>,
    pub color_patch: MapColorPatch,
}

impl Packet for MapDataPacket {
    const ID: i32 = 0x2C;
    const PHASE: ConnectionState = ConnectionState::Play;
}

#[derive(PacketSerde, Debug, Clone)]
pub struct TradeItem {
    pub item_id: types::VarInt,
    pub item_count: types::VarInt,
    pub components: types::Array<types::StructuredComponent>,
}

#[derive(PacketSerde, Debug, Clone)]
pub struct MerchantTrade {
    // The first item the player has to supply for this villager trade.
    // The count of the item stack is the default "price" of this trade.
    pub input_item_1: TradeItem,
    // The item the player will receive from this villager trade.
    pub output_item: types::Slot,
    // The second item the player has to supply for this villager trade. May be an empty slot.
    pub input_item_2: TradeItem,
    // True if the trade is disabled; false if the trade is enabled.
    pub trade_disabled: types::Boolean,
    // Number of times the trade has been used so far. If equal to the maximum number of trades,
    // the client will display a red X.
    pub num_uses: types::Int,
    // Number of times this trade can be used before it's exhausted.
    pub max_num_uses: types::Int,
    // Amount of XP the villager will earn each time the trade is used.
    pub xp: types::Int,
    // Can be zero or negative. The number is added to the price when an item is discounted due
    // to player reputation or other effects.
    pub special_price: types::Int,
    // Can be low (0.05) or high (0.2). Determines how much demand, player reputation, and
    // temporary effects will adjust the price.
    pub price_multiplier: types::Float,
    // If positive, causes the price to increase. Negative values seem to be treated the same as zero.
    pub demand: types::Int,
}

#[derive(PacketSerde, Debug, Clone)]
pub enum MerchantLevel {
    Novice,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}

#[derive(PacketSerde, Debug, Clone)]
pub struct MerchantOffersPacket {
    pub window_id: types::VarInt,      // The ID of the window that is open; this is an int rather than a byte.
    pub trades: types::Array<MerchantTrade>,
    pub merchant_level: MerchantLevel, // Appears on the trade GUI
    pub experience: types::VarInt,     // Total experience for this villager (always 0 for the wandering trader).
    // True if this is a regular villager; false for the wandering trader. When false, hides
    // the villager level and some other GUI elements.
    pub is_regular_merchant: types::Boolean,  
    // True for regular villagers and false for the wandering trader. If true, the "Villagers
    // restock up to two times per day." message is displayed when hovering over disabled trades.
    pub can_restock: types::Boolean,
}

impl Packet for MerchantOffersPacket {
    const ID: i32 = 0x2D;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct UpdateEntityPositionPacket {
    pub entity_id: types::VarInt,   // A unique integer ID mostly used in the protocol to identify the entity.
    pub delta: types::ShortVec3,    // Change in X position as `current * 4096 - prev * 4096`
    pub on_ground: types::Boolean,  // Whether the entity is on the ground.
}

impl Packet for UpdateEntityPositionPacket {
    const ID: i32 = 0x2E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct UpdateEntityPositionAndRotationPacket {
    pub entity_id: types::VarInt,   // A unique integer ID mostly used in the protocol to identify the entity.
    pub delta: types::ShortVec3,    // Change in X position as `current * 4096 - prev * 4096`
    pub yaw: types::Angle,
    pub pitch: types::Angle,
    pub on_ground: types::Boolean,  // Whether the entity is on the ground.
}

impl Packet for UpdateEntityPositionAndRotationPacket {
    const ID: i32 = 0x2F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct UpdateEntityRotationPacket {
    pub entity_id: types::VarInt,   // A unique integer ID mostly used in the protocol to identify the entity.
    pub yaw: types::Angle,
    pub pitch: types::Angle,
    pub on_ground: types::Boolean,  // Whether the entity is on the ground.
}

impl Packet for UpdateEntityRotationPacket {
    const ID: i32 = 0x30;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct MoveVehiclePacket {
    pub position: types::DoubleVec3,
    pub yaw: types::Float,
    pub pitch: types::Float,
}

impl Packet for MoveVehiclePacket {
    const ID: i32 = 0x31;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct OpenBookPacket {
    pub hand: types::VarInt,
}

impl Packet for OpenBookPacket {
    const ID: i32 = 0x32;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct OpenScreenPacket {
    pub window_id: types::VarInt,
    pub window_type: types::VarInt,
    pub window_title: types::NBTValue, // Text Component
}

impl Packet for OpenScreenPacket {
    const ID: i32 = 0x33;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct OpenSignEditorPacket {
    pub location: types::Position,
    pub is_front_text: types::Boolean,
}

impl Packet for OpenSignEditorPacket {
    const ID: i32 = 0x34;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct PingPacket {
    pub id: types::Int,
}

impl Packet for PingPacket {
    const ID: i32 = 0x35;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct PingResponsePacket {
    pub payload: types::Long,
}

impl Packet for PingResponsePacket {
    const ID: i32 = 0x36;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct PlaceGhostRecipePacket {
    pub window_id: types::VarInt,
    pub recipe: types::Identifier,
}

impl Packet for PlaceGhostRecipePacket {
    const ID: i32 = 0x37;
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
pub enum FilterType {
    PassThrough,
    FullyFiltered,
    // Specifies the indexes at which characters in the original message string should be replaced
    // with the # symbol (i.e. filtered) by the Notchian client
    PartiallyFiltered(types::BitSet),
}

#[derive(PacketSerde, Debug, Clone)]
pub struct PlayerChatMessagePacket {
    pub sender: types::UUID,
    pub index: types::VarInt,
    pub message_signature: types::Optional<types::FixedSizeByteArray<256>>,
    pub message: types::String,
    pub timestamp: types::Long,
    pub salt: types::Long,
    pub previous_messages: types::Array<types::IdOr<types::FixedSizeByteArray<256>>>,
    pub unsigned_content: NBTValue,
    pub filter_type: FilterType,
    pub chat_type: types::VarInt,
    pub sender_name: types::NBTValue,
    pub target_name: types::Optional<types::NBTValue>
}

impl Packet for PlayerChatMessagePacket {
    const ID: i32 = 0x39;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct EndCombatPacket {
    pub duration: types::VarInt,  // Length of the combat in ticks.
}

impl Packet for EndCombatPacket {
    const ID: i32 = 0x3A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct EnterCombatPacket;

impl Packet for EnterCombatPacket {
    const ID: i32 = 0x3B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct CombatDeathPacket {
    pub player_id: types::VarInt,  // Entity ID of the player that died (should match the client's entity ID).
    pub message: types::NBTValue,
}

impl Packet for CombatDeathPacket {
    const ID: i32 = 0x3C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct PlayerInfoRemovePacket {
    pub players: types::Array<types::UUID>,  // UUIDs of players to remove from the player list.
}

impl Packet for PlayerInfoRemovePacket {
    const ID: i32 = 0x3D;
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