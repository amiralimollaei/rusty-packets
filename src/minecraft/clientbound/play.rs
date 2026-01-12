use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

#[derive(PacketSerde, Debug, Clone)]
pub struct AwardStatistic {
    pub category_id: types::VarInt,
    pub statistic_id: types::VarInt,
    pub value: types::VarInt,
}

#[derive(PacketSerde, Debug, Clone)]
pub struct ChunkBiomeData {
    pub chunk_z: types::Int,
    pub chunk_x: types::Int,
    pub chunk_data: types::ByteArray,
}

#[derive(PacketSerde, Debug, Clone)]
pub struct CommandSuggestionMatch {
    pub match_: types::String,
    pub tooltip: types::Optional<types::NBTValue>, // optional text component
}

#[derive(PacketSerde, Debug, Clone)]
pub struct BlockEntityData {
    // The X and Z coordinates of the block entity, packed into a single byte.
    // The X coordinate is stored in the upper 4 bits, and the Z coordinate is stored in the lower 4 bits.
    pub packed_xz: types::UnsignedByte,
    pub y: types::UnsignedShort, // The height relative to the world
    pub type_: types::VarInt,    // The type of block entity
    pub data: types::NBTValue,   // The block entity's data, without the X, Y, and Z values
}

#[derive(PacketSerde, Debug, Clone)]
pub struct MapIcon {
    pub type_: types::VarInt,
    pub x: types::Byte, // Map coordinates: -128 for furthest left, +127 for furthest right
    pub z: types::Byte, // Map coordinates: -128 for highest, +127 for lowest
    pub direction: types::Byte,
    pub display_name: types::Optional<types::NBTValue>, // Optional Text Component
}

#[derive(Debug, Clone)]
pub struct MapColorPatch {
    pub columns: types::UnsignedByte,      // Number of columns updated
    pub rows: Option<types::UnsignedByte>, // Only if Columns is more than 0; number of rows updated
    pub x: Option<types::UnsignedByte>, // Only if Columns is more than 0; x offset of the westernmost column
    pub z: Option<types::UnsignedByte>, // Only if Columns is more than 0; z offset of the northernmost row
    pub data: Option<types::Array<types::ByteArray>>, // Only if Columns is more than 0
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
pub enum FilterType {
    PassThrough,
    FullyFiltered,
    // Specifies the indexes at which characters in the original message string should be replaced
    // with the # symbol (i.e. filtered) by the Notchian client
    PartiallyFiltered(types::BitSet),
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
        properties: types::Array<types::SingedProperty>,
    },
    InitializeChat {
        signature_data: types::Optional<SignatureData>,
    },
    GameMode(types::VarInt),
    Listed(types::Boolean),
    Ping(types::VarInt),
    DisplayName(types::Optional<types::NBTValue>),
}

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdates {
    pub actions_mask: types::Byte,
    pub player_actions: Vec<(types::UUID, PlayerInfoUpdateAction)>,
}

impl PacketReadable for PlayerInfoUpdates {
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
                    0 => PlayerInfoUpdateAction::AddPlayer {
                        name: types::String::read(stream),
                        properties: types::Array::<types::SingedProperty>::read(stream),
                    },
                    1 => PlayerInfoUpdateAction::InitializeChat {
                        signature_data: types::Optional::<SignatureData>::read(stream),
                    },
                    2 => PlayerInfoUpdateAction::GameMode(types::VarInt::read(stream)),
                    3 => PlayerInfoUpdateAction::Listed(types::Boolean::read(stream)),
                    4 => PlayerInfoUpdateAction::Ping(types::VarInt::read(stream)),
                    5 => PlayerInfoUpdateAction::DisplayName(
                        types::Optional::<types::NBTValue>::read(stream),
                    ),
                    _ => {
                        panic!()
                    }
                };
                player_actions.push((player_uuid, player_info_update_action));
            }
        }
        Self {
            actions_mask: actions_mask,
            player_actions: player_actions,
        }
    }
}

impl PacketWritable for PlayerInfoUpdates {
    fn write(&self, stream: &mut impl std::io::Write) {
        self.actions_mask.write(stream);
        types::VarInt::from_i32(self.player_actions.len() as i32).write(stream);
        for (player_uuid, player_info_update_action) in &self.player_actions {
            player_uuid.write(stream);
            match player_info_update_action {
                PlayerInfoUpdateAction::AddPlayer { name, properties } => {
                    name.write(stream);
                    properties.write(stream);
                }
                PlayerInfoUpdateAction::InitializeChat { signature_data } => {
                    signature_data.write(stream);
                }
                PlayerInfoUpdateAction::GameMode(var_int) => {
                    var_int.write(stream);
                }
                PlayerInfoUpdateAction::Listed(boolean) => {
                    boolean.write(stream);
                }
                PlayerInfoUpdateAction::Ping(var_int) => {
                    var_int.write(stream);
                }
                PlayerInfoUpdateAction::DisplayName(optional) => {
                    optional.write(stream);
                }
            }
        }
    }
}

impl PacketSerde for PlayerInfoUpdates {}

#[derive(PacketSerde, Debug, Clone)]
pub enum LootAtPoint {
    Feet,
    Eyes,
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
pub enum RecipeBookUpdate {
    Init {
        recipe_book_data: RecipeBookData,
        recipe_ids: types::Array<types::Identifier>,
        init_recipe_ids: types::Array<types::Identifier>,
    },
    Add {
        recipe_book_data: RecipeBookData,
        recipe_ids: types::Array<types::Identifier>,
    },
    Remove {
        recipe_book_data: RecipeBookData,
        recipe_ids: types::Array<types::Identifier>,
    },
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
    OptionalBlockState(types::VarInt), // 0 for absent (air is unrepresentable)
    NBT(types::NBTValue),
    Particle(types::ParticleEnum),
    Particles(types::Array<types::ParticleEnum>),
    VillagerData {
        villager_type: types::VarInt,
        villager_profession: types::VarInt,
        villager_level: types::VarInt,
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
    Quaternion(types::FloatVec4),
}

#[derive(PacketSerde, Debug, Clone)]
pub struct EntityMetadataEntry {
    pub index: types::UnsignedByte,
    pub value: EntityMetadataValue,
}

#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub metadata: Vec<EntityMetadataEntry>,
}

impl PacketReadable for EntityMetadata {
    fn read(stream: &mut impl std::io::Read) -> Self {
        let mut metadata = Vec::new();
        loop {
            let metadata_entry_index = types::UnsignedByte::read(stream);
            if metadata_entry_index.get_value() == 0xFF {
                break;
            }
            let metadata_entry_value = EntityMetadataValue::read(stream);
            metadata.push(EntityMetadataEntry {
                index: metadata_entry_index,
                value: metadata_entry_value,
            });
        }
        Self {
            metadata,
        }
    }
}

impl PacketWritable for EntityMetadata {
    fn write(&self, stream: &mut impl std::io::Write) {
        for metadata_entry in &self.metadata {
            metadata_entry.write(stream);
        }
        types::UnsignedByte::new(0xFF).write(stream);
    }
}

impl PacketSerde for EntityMetadata {}

// ###### Generic Clientbound Play Packet ######

#[derive(PacketSerde, Debug, Clone)]
pub struct PlaceholderPacket {
    pub data: types::UnsizedByteArray,
}

#[derive(PacketSerde, Debug, Clone)]
pub enum ClientboundPlayPacket {
    BundleDelimiter,
    SpawnEntity {
        entity_id: types::VarInt, // A unique integer ID mostly used in the protocol to identify the entity.
        entity_uuid: types::UUID, // A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
        entity_type: types::VarInt, // ID in the minecraft:entity_type registry.
        position: types::FloatVec3, // entity x y z position encoded as float
        pitch: types::Angle, // To get the real pitch, you must divide this by (256.0F / 360.0F)
        yaw: types::Angle,   // To get the real yaw, you must divide this by (256.0F / 360.0F)
        head_yaw: types::Angle, // Only used by living entities, where the head of the entity may differ from the general body rotation.
        data: types::VarInt, // Meaning dependent on the value of the Type field, see Object Data for details.
        velocity: types::ShortVec3, // entity x y z velocity encoded as float
    },
    SpawnExperienceOrb {
        entity_id: types::VarInt, // A unique integer ID mostly used in the protocol to identify the entity.
        position: types::DoubleVec3, // entity x y z position encoded as Double
        count: types::Short,      // The amount of experience this orb will reward once collected.
    },
    EntityAnimation {
        entity_id: types::VarInt, // A unique integer ID mostly used in the protocol to identify the entity.
        animation: types::UnsignedByte, // 0->Swing main arm, 1->UNDEFINED, 2->Leave bed, 3->Swing offhand, 4->Critical effect, 5->Magic critical effect
    },
    AwardStatistics {
        statistics: types::Array<AwardStatistic>,
    },
    AcknowledgeBlockChange {
        sequence_id: types::VarInt,
    },
    SetBlockDestroyStage {
        entity_id: types::VarInt,
        location: types::Position,
        destroy_stage: types::Byte,
    },
    BlockEntityData {
        location: types::Position,
        type_: types::VarInt,
        nbt_data: types::NBTValue,
    },
    BlockAction {
        location: types::Position,
        action_id: types::UnsignedByte,
        action_parameter: types::UnsignedByte,
        block_type: types::VarInt,
    },
    BlockUpdate {
        location: types::Position,
        block_id: types::VarInt,
    },
    BossBar {
        uuid: types::UUID,
        action: types::VarInt,
        action_data: types::UnsizedByteArray, // TODO: this data should be parsed based on the value of action
    },
    ChangeDifficulty {
        difficulty: types::UnsignedByte, // 0: peaceful, 1: easy, 2: normal, 3: hard.
        is_locked: types::Boolean,
    },
    ChunkBatchFinished {
        batch_size: types::VarInt,
    },
    ChunkBatchStart,
    ChunkBiomes {
        chunks: types::Array<ChunkBiomeData>,
    },
    ClearTitles {
        reset: types::Boolean,
    },
    CommandSuggestionsResponse {
        id: types::VarInt,
        start: types::VarInt,
        length: types::VarInt,
        matches: types::Array<CommandSuggestionMatch>,
    },
    Commands {
        //reset: types::Array<GraphNode>, // TODO implelemnt GraphNode
        //root_index: types::VarInt,
        data: types::UnsizedByteArray,
    },
    CloseContainer {
        window_id: types::UnsignedByte,
    },
    SetContainerContent {
        window_id: types::UnsignedByte,
        state_id: types::VarInt,
        slots: types::Array<types::Slot>,
        carriedi_item: types::Slot, // Item being dragged with the mouse.
    },
    SetContainerProperty {
        window_id: types::VarInt,
        property: types::Short, // The meaning of the Property field depends on the type of the window.
        value: types::Short,
    },
    SetContainerSlot {
        window_id: types::UnsignedByte,
        state_id: types::VarInt,
        slot: types::Short, // The slot that should be updated.
        slot_data: types::Slot,
    },
    CookieRequest {
        key: types::Identifier,
    },
    SetCooldown {
        item_id: types::VarInt,
        cooldown_ticks: types::VarInt,
    },
    ChatSuggestions {
        action: types::VarInt, // 0: Add, 1: Remove, 2: Set
        entries: types::Array<types::String>,
    },
    ClientboundPluginMessage {
        channel: types::Identifier,
        data: types::UnsizedByteArray,
    },
    DamageEvent {
        entity_id: types::VarInt,
        source_type_id: types::VarInt,
        source_cause_id: types::VarInt,
        source_direct_id: types::VarInt,
        position: types::Optional<types::DoubleVec3>,
    },
    DebugSample {
        samples: types::Array<types::Long>, // Array of type-dependent samples.
    },
    DeleteMessage {
        message_id: types::VarInt,
        signature: types::FixedSizeByteArray<256>,
    },
    Disconnect {
        reason: types::NBTValue, // an NBT Tag containing a single string
    },
    DisguisedChatMessage {
        message: types::NBTValue, // Text Component: This is used as the content parameter when formatting the message on the client.
        chat_type: types::VarInt, // The type of chat in the minecraft:chat_type registry, defined by the Registry Data packet.
        sender_name: types::NBTValue, // This is used as the sender parameter when formatting the message on the client.
        target_name: types::Optional<types::NBTValue>,
    },
    EntityEvent {
        entity_id: types::Int,
        entity_status: types::Byte,
    },
    Explosion {
        position: types::DoubleVec3,
        // If the strength is greater or equal to 2.0, or the block interaction is not 0 (keep),
        // large explosion particles are used. Otherwise, small explosion particles are used.
        strength: types::Float,
        // Each record is 3 signed bytes long; the 3 bytes are the XYZ (respectively) signed offsets of affected blocks.
        records: types::Array<types::ByteVec3>,
        player_motion: types::FloatVec3, // velocity of the player being pushed by the explosion.
        block_interaction: types::VarInt,
        small_explosion_particle: types::ParticleEnum,
        large_explosion_particle: types::ParticleEnum,
        explotion_sound: types::IdOr<types::SoundEvent>,
    },
    UnloadChunk {
        chunk_x: types::Int, // Block coordinate divided by 16 (rounded down)
        chunk_z: types::Int, // Block coordinate divided by 16 (rounded down)
    },
    GameEvent {
        event: types::UnsignedByte,
        value: types::Float,
    },
    OpenHorseScreen {
        window_id: types::UnsignedByte,
        slot_count: types::VarInt,
        entity_id: types::Int,
    },
    HurtAnimation {
        entity_id: types::Int,
        yaw: types::Float,
    },
    InitializeWorldBorder {
        x: types::Double,
        z: types::Double,
        old_diameter: types::Double, // Current length of a single side of the world border, in meters.
        new_diameter: types::Double, // Target length of a single side of the world border, in meters.
        // Number of real-time milliseconds until New Diameter is reached. It appears that Notchian server does
        // not sync world border speed to game ticks, so it gets out of sync with server lag. If the world border
        // is not moving, this is set to 0.
        speed: types::VarLong,
        portal_teleport_boundary: types::VarInt,
        warning_blocks: types::VarInt, // In meters.
        warning_time: types::VarInt,   // In seconds as set by /worldborder warning time.
    },
    KeepAlive {
        keepalive_id: types::Long,
    },
    ChunkDataAndUpdateLight {
        chunk_x: types::Int, // Block coordinate divided by 16 (rounded down)
        chunk_z: types::Int, // Block coordinate divided by 16 (rounded down)
        heightmaps: types::NBTValue,
        data: types::ByteArray,
        block_entities: types::Array<BlockEntityData>,
        sky_light_mask: types::BitSet,
        block_light_mask: types::BitSet,
        empty_sky_light_mask: types::BitSet,
        empty_block_light_mask: types::BitSet,
        sky_light_arrays: types::Array<types::ByteArray>,
        block_light_arrays: types::Array<types::ByteArray>,
    },
    WorldEvent {
        event: types::Int,
        position: types::Position,
        data: types::Int,
        disable_relative_volume: types::Boolean,
    },
    Particle {
        long_distance: types::Boolean, // If true, particle distance increases from 256 to 65536.
        position: types::DoubleVec3,
        offset: types::FloatVec3, // This is added to the X position after being multiplied by random.nextGaussian().
        max_speed: types::Float,
        particle_count: types::Int, // The number of particles to create.
        disable_relative_volume: types::Boolean,
        particle: types::ParticleEnum,
    },
    UpdateLight {
        chunk_x: types::VarInt,
        chunk_z: types::VarInt,
        sky_light_mask: types::BitSet,
        block_light_mask: types::BitSet,
        empty_sky_light_mask: types::BitSet,
        empty_block_light_mask: types::BitSet,
        sky_light_arrays: types::Array<types::ByteArray>,
        block_light_arrays: types::Array<types::ByteArray>,
    },
    Login {
        /*
        entity_id: types::Int,                                        // The player's Entity ID (EID).
        is_harcore: types::Boolean,
        dimensions: types::Array<types::String>,                      // Identifiers for all dimensions on the server.
        max_players: types::VarInt,                                   // Was once used by the client to draw the player list, but now is ignored.
        view_distance: types::VarInt,                                 // Render distance (2-32).
        simulation_distance: types::VarInt,                           // The distance that the client will process specific things, such as entities.
        reduced_debug_info: types::Boolean,                           // If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
        enable_respawn_screen: types::Boolean,                        // Set to false when the doImmediateRespawn gamerule is true.
        do_limited_crafting: types::Boolean,                          // Whether players can only craft recipes they have already unlocked. Currently unused by the client.
        dimension_type: types::VarInt,                                // The ID of the type of dimension in the `minecraft:dimension_type` registry, defined by the Registry Data packet.
        dimension_name: types::String,                                // Name of the dimension being spawned into.
        hashed_seed: types::Long,                                     // First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
        game_mode: types::UnsignedByte,                               // 0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
        previous_game_mode: types::Byte,                              // -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed)
        is_debug: types::Boolean,                                     // True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks.
        is_flat: types::NBTValue,                                     // True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63.
        death_dimension_name: types::Optional<types::Identifier>,     // Name of the dimension the player died in.
        death_location: types::Optional<types::Position>,             // The location that the player died at.
        portal_cooldown: types::VarInt,                               // The number of ticks until the player can use the portal again.
        enforces_secure_chat: types::Boolean
        */
        // TODO: fix login packet structure not matching what the server sends us
        data: types::UnsizedByteArray,
    },
    MapData {
        map_id: types::VarInt,
        scale: types::Byte,
        locked: types::Boolean,
        icons: types::Optional<types::Array<MapIcon>>,
        color_patch: MapColorPatch,
    },
    MerchantOffers {
        window_id: types::VarInt, // The ID of the window that is open; this is an int rather than a byte.
        trades: types::Array<MerchantTrade>,
        merchant_level: MerchantLevel, // Appears on the trade GUI
        experience: types::VarInt, // Total experience for this villager (always 0 for the wandering trader).
        // True if this is a regular villager; false for the wandering trader. When false, hides
        // the villager level and some other GUI elements.
        is_regular_merchant: types::Boolean,
        // True for regular villagers and false for the wandering trader. If true, the "Villagers
        // restock up to two times per day." message is displayed when hovering over disabled trades.
        can_restock: types::Boolean,
    },
    UpdateEntityPosition {
        entity_id: types::VarInt, // A unique integer ID mostly used in the protocol to identify the entity.
        delta: types::ShortVec3,  // Change in X position as `current * 4096 - prev * 4096`
        on_ground: types::Boolean, // Whether the entity is on the ground.
    },
    UpdateEntityPositionAndRotation {
        entity_id: types::VarInt, // A unique integer ID mostly used in the protocol to identify the entity.
        delta: types::ShortVec3,  // Change in X position as `current * 4096 - prev * 4096`
        yaw: types::Angle,
        pitch: types::Angle,
        on_ground: types::Boolean, // Whether the entity is on the ground.
    },
    UpdateEntityRotation {
        entity_id: types::VarInt, // A unique integer ID mostly used in the protocol to identify the entity.
        yaw: types::Angle,
        pitch: types::Angle,
        on_ground: types::Boolean, // Whether the entity is on the ground.
    },
    MoveVehicle {
        position: types::DoubleVec3,
        yaw: types::Float,
        pitch: types::Float,
    },
    OpenBook {
        hand: types::VarInt,
    },
    OpenScreen {
        window_id: types::VarInt,
        window_type: types::VarInt,
        window_title: types::NBTValue, // Text Component
    },
    OpenSignEditor {
        location: types::Position,
        is_front_text: types::Boolean,
    },
    Ping {
        id: types::Int,
    },
    PingResponse {
        payload: types::Long,
    },
    PlaceGhostRecipe {
        window_id: types::VarInt,
        recipe: types::Identifier,
    },
    PlayerAbilities {
        flags: types::UnsignedByte, // 0x01: Invulnerable, 0x02: Flying, 0x04: Allow Flying, 0x08: Creative Mode (Instant Break)	.
        flying_speed: types::Float, // 0.05 by default.
        field_of_view_modifier: types::Float, // Modifies the field of view, like a speed potion. A Notchian server will use the same value as the movement speed sent in the Update Attributes packet, which defaults to 0.1 for players.
    },
    PlayerChatMessage {
        sender: types::UUID,
        index: types::VarInt,
        message_signature: types::Optional<types::FixedSizeByteArray<256>>,
        message: types::String,
        timestamp: types::Long,
        salt: types::Long,
        previous_messages: types::Array<types::IdOr<types::FixedSizeByteArray<256>>>,
        unsigned_content: types::NBTValue,
        filter_type: FilterType,
        chat_type: types::VarInt,
        sender_name: types::NBTValue,
        target_name: types::Optional<types::NBTValue>,
    },
    EndCombat {
        duration: types::VarInt, // Length of the combat in ticks.
    },
    EnterCombat,
    CombatDeath {
        player_id: types::VarInt, // Entity ID of the player that died (should match the client's entity ID).
        message: types::NBTValue,
    },
    PlayerInfoRemove {
        players: types::Array<types::UUID>, // UUIDs of players to remove from the player list.
    },
    PlayerInfoUpdate { 
        updates: PlayerInfoUpdates 
    },
    LookAt {
        point: LootAtPoint,
        target: types::DoubleVec3,
        entity: types::Optional<(types::VarInt, LootAtPoint)>,
    },
    SynchronizePlayerPosition {
        location: types::Location,  // contains the location of a player
        flags: types::Byte, // When the value of the this byte masked is zero the field is absolute, otherwise relative.
        teleport_id: types::VarInt, // VarInt: the client should respond with the same id
    },
    UpdateRecipeBook {
        update: RecipeBookUpdate
    },
    RemoveEntities {
        entity_ids: types::Array<types::VarInt>,
    },
    RemoveEntityEffect {
        entity_id: types::VarInt,
        effect_id: types::VarInt,
    },
    ResetScore {
        entity_name: types::String,
        objective_name: types::Optional<types::String>,
    },
    RemoveResourcePack {
        uuid: types::Optional<types::UUID>,
    },
    AddResourcePack {
        uuid: types::UUID,
        url: types::String,
        hash: types::String,
        is_forced: types::Boolean,
        prompt_message: types::Optional<types::NBTValue>,
    },
    Respawn {
        dimension_type: types::VarInt,
        dimention_name: types::Identifier,
        hashed_seed: types::Long,
        game_mode: types::UnsignedByte,
        prev_game_mode: types::UnsignedByte,
        is_debug: types::Boolean,
        is_flat: types::Boolean,
        death_location: types::Optional<(types::Identifier, types::Position)>,
        portal_cooldown: types::VarInt,
        data_kept: types::Byte,
    },
    SetHeadRotation {
        entity_id: types::VarInt,
        head_yaw: types::Angle, // New angle, not a delta.
    },
    UpdateSectionBlocks {
        chunk_section_position: types::Long,
        // Each entry is composed of the block state id, shifted left by 12, and the relative
        // block position in the chunk section (4 bits for x, z, and y, from left to right).
        blocks_array: types::Array<types::VarLong>,
    },
    SelectAdvancementsTab {
        id: types::Optional<types::Identifier>,
    },
    ServerData {
        motd: types::NBTValue,
        icon: types::Optional<types::ByteArray>,
    },
    SetActionBarText {
        text: types::NBTValue,
    },
    SetBorderCenter {
        x: types::Double,
        z: types::Double,
},
    SetBorderLerpSize {
        old_diameter: types::Double,
        new_diameter: types::Double,
        speed: types::VarLong,
    },
    SetBorderSize{
        diameter: types::Double,
    },
    SetBorderWarningDelay {
        delay: types::VarInt,
    },
    SetBorderWarningDistance {
        distance: types::VarInt,
    },
    SetCamera {
        camera_id: types::VarInt,
    },
    SetHeldItem {
        slot: types::Byte,
    },
    SetCenterChunk {
        x: types::VarInt,
        z: types::VarInt,
    },
    SetRenderDistance {
        view_distance: types::VarInt,
    },
    SetDefaultSpawnPosition {
        location: types::Position,
        angle: types::Float,
    },
    DisplayObjective {
        // The position of the scoreboard.
        //   0: list,
        //   1: sidebar,
        //   2: below name,
        //   3 - 18: team specific sidebar, indexed as 3 + team color.
        position: types::VarInt,
        // The unique name for the scoreboard to be displayed.
        score_name: types::String,
    },
    SetEntityMetadata {
        entity_id: types::VarInt,
        metadata: EntityMetadata
    },
    // TODO: implement the rest of the packets
    PlaceholderPacket59(PlaceholderPacket),
    PlaceholderPacket5A(PlaceholderPacket),
    PlaceholderPacket5B(PlaceholderPacket),
    PlaceholderPacket5C(PlaceholderPacket),
    PlaceholderPacket5D(PlaceholderPacket),
    PlaceholderPacket5E(PlaceholderPacket),
    PlaceholderPacket5F(PlaceholderPacket),
    PlaceholderPacket60(PlaceholderPacket),
    PlaceholderPacket61(PlaceholderPacket),
    PlaceholderPacket62(PlaceholderPacket),
    PlaceholderPacket63(PlaceholderPacket),
    PlaceholderPacket64(PlaceholderPacket),
    PlaceholderPacket65(PlaceholderPacket),
    PlaceholderPacket66(PlaceholderPacket),
    PlaceholderPacket67(PlaceholderPacket),
    PlaceholderPacket68(PlaceholderPacket),
    PlaceholderPacket69(PlaceholderPacket),
    PlaceholderPacket6A(PlaceholderPacket),
    PlaceholderPacket6B(PlaceholderPacket),
    PlaceholderPacket6C(PlaceholderPacket),
    PlaceholderPacket6D(PlaceholderPacket),
    PlaceholderPacket6E(PlaceholderPacket),
    PlaceholderPacket6F(PlaceholderPacket),
    PlaceholderPacket70(PlaceholderPacket),
    PlaceholderPacket71(PlaceholderPacket),
    PlaceholderPacket72(PlaceholderPacket),
    PlaceholderPacket73(PlaceholderPacket),
    PlaceholderPacket74(PlaceholderPacket),
    PlaceholderPacket75(PlaceholderPacket),
    PlaceholderPacket76(PlaceholderPacket),
    PlaceholderPacket77(PlaceholderPacket),
    PlaceholderPacket78(PlaceholderPacket),
    PlaceholderPacket79(PlaceholderPacket),
    PlaceholderPacket7A(PlaceholderPacket),
    PlaceholderPacket7B(PlaceholderPacket),
    PlaceholderPacket7C(PlaceholderPacket),
}

impl GenericPacket for ClientboundPlayPacket {}
