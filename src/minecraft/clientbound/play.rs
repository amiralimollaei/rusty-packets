use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketSerde, PacketWritable}, types
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
pub struct ExplosionPacket {
    pub position: types::DoubleVec3,
    // If the strength is greater or equal to 2.0, or the block interaction is not 0 (keep),
    // large explosion particles are used. Otherwise, small explosion particles are used.
    pub strength: types::Float,
    // Each record is 3 signed bytes long; the 3 bytes are the XYZ (respectively) signed offsets of affected blocks.
    pub records: types::Array<types::ByteVec3>,
    pub player_motion: types::FloatVec3, // velocity of the player being pushed by the explosion.
    pub block_interaction: types::VarInt,
    pub small_explosion_particle: types::ParticleEnum,
    pub large_explosion_particle: types::ParticleEnum,
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
    pub particle: types::ParticleEnum,
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
    pub unsigned_content: types::NBTValue,
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
pub struct SignatureData {
    pub session_id: types::UUID,
    pub expiration_time: types::Long,
    pub public_key: types::ByteArray,
    pub public_key_signature: types::ByteArray,
}

#[derive(Debug, Clone)]
pub enum PlayerInfoUpdateAction {
    AddPlayer {
        name: types::String,
        properties: types::Array<types::SingedProperty>
    },
    InitializeChat {
        signature_data: types::Optional<SignatureData>
    },
    GameMode(types::VarInt),
    Listed(types::Boolean),
    Ping(types::VarInt),
    DisplayName(types::Optional<types::NBTValue>),
}

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdatePacket {
    pub actions_mask: types::Byte,
    pub player_actions: Vec<(types::UUID, PlayerInfoUpdateAction)>,
}

impl PacketReadable for PlayerInfoUpdatePacket {
    fn read(stream: &mut impl std::io::Read) -> Self {
        let actions_mask = types::Byte::read(stream);
        let actions_mask_u8 = actions_mask.get_u8();
        let num_players = types::VarInt::read(stream);
        let num_players_usize = num_players.get_value() as usize;
        let mut player_actions = Vec::with_capacity(num_players_usize);
        for _ in 0..num_players_usize {
            for shift in 0..6 {
                let is_action_present = (actions_mask_u8 & (1u8 >> shift)) != 0;
                if !is_action_present {
                    continue;
                }
                let player_uuid = types::UUID::read(stream);
                let player_info_update_action = match shift {
                    0 => {
                        PlayerInfoUpdateAction::AddPlayer {
                            name: types::String::read(stream),
                            properties: types::Array::<types::SingedProperty>::read(stream)
                        }
                    }
                    1 => {
                        PlayerInfoUpdateAction::InitializeChat { 
                            signature_data: types::Optional::<SignatureData>::read(stream)
                        }
                    }
                    2 => {
                        PlayerInfoUpdateAction::GameMode(types::VarInt::read(stream))
                    }
                    3 => {
                        PlayerInfoUpdateAction::Listed(types::Boolean::read(stream))
                    }
                    4 => {
                        PlayerInfoUpdateAction::Ping(types::VarInt::read(stream))
                    }
                    5 => {
                        PlayerInfoUpdateAction::DisplayName(types::Optional::<types::NBTValue>::read(stream))
                    }
                    _ => {
                        panic!()
                    }
                };
                player_actions.push((player_uuid, player_info_update_action));
            }
        }
        Self {
            actions_mask: actions_mask,
            player_actions: player_actions
        }
    }
}

impl PacketWritable for PlayerInfoUpdatePacket {
    fn write(&self, stream: &mut impl std::io::Write) {
        self.actions_mask.write(stream);
        types::VarInt::from_i32(self.player_actions.len() as i32).write(stream);
        for (player_uuid, player_info_update_action) in &self.player_actions {
            player_uuid.write(stream);
            match player_info_update_action {
                PlayerInfoUpdateAction::AddPlayer { name, properties } => {
                    name.write(stream);
                    properties.write(stream);
                },
                PlayerInfoUpdateAction::InitializeChat { signature_data } => {
                    signature_data.write(stream);
                },
                PlayerInfoUpdateAction::GameMode(var_int) => {
                    var_int.write(stream);
                },
                PlayerInfoUpdateAction::Listed(boolean) => {
                    boolean.write(stream);
                },
                PlayerInfoUpdateAction::Ping(var_int) => {
                    var_int.write(stream);
                },
                PlayerInfoUpdateAction::DisplayName(optional) => {
                    optional.write(stream);
                },
            }
        }
    }
}

impl PacketSerde for PlayerInfoUpdatePacket {}

impl Packet for PlayerInfoUpdatePacket {
    const ID: i32 = 0x3E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub enum LootAtPoint {
    Feet,
    Eyes
}

#[derive(PacketSerde, Debug, Clone)]
pub struct LookAtPacket {
    pub point: LootAtPoint,
    pub target: types::DoubleVec3,
    pub entity: types::Optional<(types::VarInt, LootAtPoint)>
}

impl Packet for LookAtPacket {
    const ID: i32 = 0x3F;
    const PHASE: ConnectionState = ConnectionState::Play;
}

#[derive(PacketSerde, Debug, Clone)]
pub struct SynchronizePlayerPositionPacket {
    pub location: types::Location,            // contains the location of a player
    pub flags: types::Byte,            // When the value of the this byte masked is zero the field is absolute, otherwise relative.
    pub teleport_id: types::VarInt,    // VarInt: the client should respond with the same id
}

impl Packet for SynchronizePlayerPositionPacket {
    const ID: i32 = 0x40;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct RecipeBookData {
    pub crafting_recipe_book_open: types::Boolean,
    pub crafting_recipe_book_filter_active: types::Boolean,
    pub smelting_recipe_book_open: types::Boolean,
    pub smelting_recipe_book_filter_active: types::Boolean,
    pub blast_furnace_recipe_book_open: types::Boolean,
    pub blast_furnace_recipe_book_filter_active: types::Boolean,
    pub smoker_recipe_book_open: types::Boolean,
    pub smoker_recipe_book_filter_active: types::Boolean,
}


#[derive(PacketSerde, Debug, Clone)]
pub enum UpdateRecipeBookPacket {
    Init {
        recipe_book_data: RecipeBookData,
        recipe_ids: types::Array<types::Identifier>,
        init_recipe_ids: types::Array<types::Identifier>
    },
    Add {
        recipe_book_data: RecipeBookData,
        recipe_ids: types::Array<types::Identifier>,
    },
    Remove {
        recipe_book_data: RecipeBookData,
        recipe_ids: types::Array<types::Identifier>,
    }
}

impl Packet for UpdateRecipeBookPacket {
    const ID: i32 = 0x41;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct RemoveEntitiesPacket {
    pub entity_ids: types::Array<types::VarInt>,
}

impl Packet for RemoveEntitiesPacket {
    const ID: i32 = 0x42;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct RemoveEntityEffectPacket {
    pub entity_id: types::VarInt,
    pub effect_id: types::VarInt,
}

impl Packet for RemoveEntityEffectPacket {
    const ID: i32 = 0x43;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ResetScorePacket {
    pub entity_name: types::String,
    pub objective_name: types::Optional<types::String>,
}

impl Packet for ResetScorePacket {
    const ID: i32 = 0x44;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct RemoveResourcePackPacket {
    pub uuid: types::Optional<types::UUID>,
}

impl Packet for RemoveResourcePackPacket {
    const ID: i32 = 0x45;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct AddResourcePackPacket {
    pub uuid: types::UUID,
    pub url: types::String,
    pub hash: types::String,
    pub is_forced: types::Boolean,
    pub prompt_message: types::Optional<types::NBTValue>
}

impl Packet for AddResourcePackPacket {
    const ID: i32 = 0x46;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct RespawnPacket {
    pub dimension_type: types::VarInt,
    pub dimention_name: types::Identifier,
    pub hashed_seed: types::Long,
    pub game_mode: types::UnsignedByte,
    pub prev_game_mode: types::UnsignedByte,
    pub is_debug: types::Boolean,
    pub is_flat: types::Boolean,
    pub death_location: types::Optional<(types::Identifier, types::Position)>,
    pub portal_cooldown: types::VarInt,
    pub data_kept: types::Byte,
}

impl Packet for RespawnPacket {
    const ID: i32 = 0x47;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetHeadRotationPacket {
    pub entity_id: types::VarInt,
    pub head_yaw: types::Angle,  // New angle, not a delta.
}

impl Packet for SetHeadRotationPacket {
    const ID: i32 = 0x48;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct UpdateSectionBlocksPacket {
    pub chunk_section_position: types::Long,
    // Each entry is composed of the block state id, shifted left by 12, and the relative
    // block position in the chunk section (4 bits for x, z, and y, from left to right).
    pub blocks_array: types::Array<types::VarLong>,
}

impl Packet for UpdateSectionBlocksPacket {
    const ID: i32 = 0x49;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SelectAdvancementsTabPacket {
    pub id: types::Optional<types::Identifier>,
}

impl Packet for SelectAdvancementsTabPacket {
    const ID: i32 = 0x4A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ServerDataPacket {
    pub motd: types::NBTValue,
    pub icon: types::Optional<types::ByteArray>
}

impl Packet for ServerDataPacket {
    const ID: i32 = 0x4B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetActionBarTextPacket {
    pub text: types::NBTValue,
}

impl Packet for SetActionBarTextPacket {
    const ID: i32 = 0x4C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetBorderCenterPacket {
    pub x: types::Double,
    pub z: types::Double
}

impl Packet for SetBorderCenterPacket {
    const ID: i32 = 0x4D;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetBorderLerpSizePacket {
    pub old_diameter: types::Double,
    pub new_diameter: types::Double,
    pub speed: types::VarLong,
}

impl Packet for SetBorderLerpSizePacket {
    const ID: i32 = 0x4E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetBorderSizePacket {
    pub diameter: types::Double,
}

impl Packet for SetBorderSizePacket {
    const ID: i32 = 0x4F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetBorderWarningDelayPacket {
    pub delay: types::VarInt,
}

impl Packet for SetBorderWarningDelayPacket {
    const ID: i32 = 0x50;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetBorderWarningDistancePacket {
    pub distance: types::VarInt,
}

impl Packet for SetBorderWarningDistancePacket {
    const ID: i32 = 0x51;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetCameraPacket {
    pub camera_id: types::VarInt,
}

impl Packet for SetCameraPacket {
    const ID: i32 = 0x52;
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

#[derive(PacketSerde, Debug, Clone)]
pub struct SetCenterChunkPacket {
    pub x: types::VarInt,
    pub z: types::VarInt
}

impl Packet for SetCenterChunkPacket {
    const ID: i32 = 0x54;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetRenderDistancePacket {
    pub view_distance: types::VarInt,
}

impl Packet for SetRenderDistancePacket {
    const ID: i32 = 0x55;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct SetDefaultSpawnPositionPacket {
    pub location: types::Position,
    pub angle: types::Float
}

impl Packet for SetDefaultSpawnPositionPacket {
    const ID: i32 = 0x56;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct DisplayObjectivePacket {
    // The position of the scoreboard. 
    //   0: list, 
    //   1: sidebar, 
    //   2: below name, 
    //   3 - 18: team specific sidebar, indexed as 3 + team color.
    pub position: types::VarInt,
    // The unique name for the scoreboard to be displayed.
    pub score_name: types::String
}

impl Packet for DisplayObjectivePacket {
    const ID: i32 = 0x57;
    const PHASE: ConnectionState = ConnectionState::Play;
}

#[derive(PacketSerde, Debug, Clone)]
pub enum EntityMetadataValue {
    Byte(types::Byte),
    VarInt(types::VarInt),
    VarLong(types::VarLong),
    Float(types::Float),
    String(types::String),
    TextComponent(types::NBTValue),
    OptionalTextComponent(types::Optional<types::NBTValue>),
    Slot(types::Slot),
    Boolean(types::Boolean),
    Rotations(types::FloatVec3),
    Position(types::Position),
    OptionalPosition(types::Optional<types::Position>),
    Direction(types::VarInt),
    OptionalUUID(types::Optional<types::UUID>),
    BlockState(types::VarInt),
    OptionalBlockState(types::VarInt),  // 0 for absent (air is unrepresentable)
    NBT(types::NBTValue),
    Particle(types::ParticleEnum),
    Particles(types::Array<types::ParticleEnum>),
    VillagerData {
        villager_type: types::VarInt,
        villager_profession: types::VarInt,
        villager_level: types::VarInt
    },
    OptionalVarint(types::OptionalVarInt),
    Pose(types::EntityPose),
    CatVariant(types::VarInt),
    WolfVariant(types::IdOr<types::WolfVariant>),
    FrogVariant(types::VarInt),
    OptionalGlobalPosition(types::Optional<types::GlobalPosition>),
    PaintingVariant(types::IdOr<types::PaintingVariant>),
    SnifferState(types::VarInt),
    ArmadilloState(types::VarInt),
    Vector3(types::FloatVec3),
    Quaternion(types::FloatVec4)
}

#[derive(PacketSerde, Debug, Clone)]
pub struct EntityMetadataEntry {
    pub index: types::UnsignedByte,
    pub value: EntityMetadataValue
}

#[derive(Debug, Clone)]
pub struct SetEntityMetadataPacket {
    pub entity_id: types::VarInt,
    pub metadata: Vec<EntityMetadataEntry>
}

impl PacketReadable for SetEntityMetadataPacket {
    fn read(stream: &mut impl std::io::Read) -> Self {
        let entity_id = types::VarInt::read(stream);
        let mut metadata = Vec::new();
        loop {
            let metadata_entry_index = types::UnsignedByte::read(stream);
            if metadata_entry_index.get_value() == 0xFF {
                break;
            }
            let metadata_entry_value = EntityMetadataValue::read(stream);
            metadata.push(EntityMetadataEntry {
                index: metadata_entry_index,
                value: metadata_entry_value
            });
        };
        Self {
            entity_id: entity_id, 
            metadata: metadata
        }
    }
}

impl PacketWritable for SetEntityMetadataPacket {
    fn write(&self, stream: &mut impl std::io::Write) {
        self.entity_id.write(stream);
        for metadata_entry in &self.metadata {
            metadata_entry.write(stream);
        }
        types::UnsignedByte::new(0xFF).write(stream);
    }
}

impl PacketSerde for SetEntityMetadataPacket {}

impl Packet for SetEntityMetadataPacket {
    const ID: i32 = 0x58;
    const PHASE: ConnectionState = ConnectionState::Play;
}