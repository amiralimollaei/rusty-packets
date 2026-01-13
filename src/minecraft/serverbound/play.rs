use generic_packet_derive::GenericPacket;
use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

#[derive(PacketSerde, Clone, Debug)]
pub struct CommandArgumentSignature {
    pub name: types::String, // The name of the argument that is signed by the following signature.
    pub timestamp: types::FixedSizeByteArray<256>, // The signature that verifies the argument. Always 256 bytes and is not length-prefixed.
}

#[derive(PacketSerde, Clone, Debug)]
pub struct SessionPublicKey {
    pub expires_at: types::Long, // The time at which the public key expires, in milliseconds since Unix epoch.
    pub public_key: types::ByteArray, // A byte array of an X.509-encoded public key, Maximum length in Notchian server is 512 bytes.
    // The signature consists of the player UUID, the key expiration timestamp, and the public key data.
    // These values are hashed using SHA-1 and signed using Mojang's private RSA key. Maximum length in Notchian server is 4096 bytes.
    pub key_signature: types::ByteArray,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct ChangedSlot {
    pub slot_number: types::Byte,
    pub slot: types::Slot, // New data for this slot, in the client's opinion.
}

#[derive(PacketSerde, Clone, Debug)]
pub enum HandEnum {
    MainHand,
    OffHand,
}

#[derive(PacketSerde, Clone, Debug)]
pub enum InteractionEnum {
    Interact {
        hand: HandEnum,
    },
    Attack,
    InteractAt {
        target: types::FloatVec3,
        hand: HandEnum,
    },
}

#[derive(PacketSerde, Clone, Debug)]
pub enum SeenAdvancementAction {
    OpenedTab { tab_id: types::Identifier },
    ClosedScreen,
}

#[derive(PacketSerde, Clone, Debug)]
pub enum MirrorEnum {
    None,
    LeftRight,
    FrontBack,
}

#[derive(PacketSerde, Clone, Debug)]
pub enum StructureBlockModeEnum {
    Save,
    Load,
    Corner,
    Data
}

#[derive(PacketSerde, Clone, Debug)]
pub enum RotationEnum {
    None,
    Clockwise90,
    Clockwise180,
    Counterclockwise90
}

#[derive(PacketSerde, Clone, Debug)]
#[discriminant_type(types::Byte)]
pub enum DifficultyEnum {
    Peaceful,
    Easy,
    Normal,
    Hard
}

#[derive(PacketSerde, Clone, Debug)]
pub enum ChatModeEnum {
    Enabled,
    CommandsOnly,
    Hidden
}

// ###### Generic Serverbound Play Packet ######

#[derive(PacketSerde, GenericPacket, Clone, Debug)]
pub enum ServerboundPlayPacket {
    ConfirmTeleportation {
        teleport_id: types::VarInt, // The ID given by the Synchronize Player Position packet.
    },
    QueryBlockEntityTag {
        transaction_id: types::VarInt, // An incremental ID so that the client can verify that the response matches.
        location: types::Position,     // The location of the block to check.
    },
    ChangeDifficulty {
        new_difficulty: DifficultyEnum,
    },
    AcknowledgeMessage {
        message_count: types::VarInt,
    },
    ChatCommand {
        command: types::String, // The command typed by the client.
    },
    SignedChatCommand {
        command: types::String, // The command typed by the client.
        timestamp: types::Long, // The timestamp that the command was executed.
        salt: types::Long,      // The salt for the following argument signatures.
        // The signatures for the command arguments, The maximum length in Notchian server is 8.
        argument_signatures: types::Array<CommandArgumentSignature>,
        message_count: types::VarInt,
        acknowledged: types::FixedSizeBitSet<3>, // Whether the client has acknowledged the command. Always 20 bits (3 bytes)
    },
    ChatMessage {
        message: types::String, // The message typed by the client.
        timestamp: types::Long, // The timestamp that the message was executed.
        salt: types::Long,      // The salt used to verify the signature hash.
        // The signature used to verify the chat message's authentication. When present, always 256 bytes and not length-prefixed.
        signature: types::Optional<types::FixedSizeByteArray<256>>,
        message_count: types::VarInt,
        acknowledged: types::FixedSizeBitSet<3>, // Whether the client has acknowledged the message. Always 20 bits (3 bytes)
    },
    PlayerSession {
        session_id: types::UUID,      // The player's session UUID.
        public_key: SessionPublicKey, // The player's public key.
    },
    ChunkBatchReceived {
        chunk_per_tick: types::Float, // Desired chunks per tick.
    },
    ClientStatus {
        action_id: types::VarInt, // 0: perform respawn, 1: request stats
    },
    ClientInformation {
        locale: types::String,                 // String: max 16 characters
        view_distance: types::Byte,            // Byte: for some reason this HAD TO BE SIGNED
        chat_mode: ChatModeEnum, // VarInt Enum: 0: enabled, 1: commands only, 2: hidden
        chat_colors: types::Boolean, // Boolean: can the chat be colored?
        skin_parts: types::UnsignedByte, // Unsigned Byte: parts of skin that are visible (7 bit bitflag)
        main_hand: types::VarInt,        // VarInt Enum: 0: left, 1: right
        text_filtering: types::Boolean, // Boolean: Enables filtering of text on signs and written book titles
        allow_server_listings: types::Boolean, // Boolean: Servers usually list online players, this option should let you not show up in that list
    },
    CommandSuggestionsRequest {
        // The id of the transaction that the server will send back to the client in the response of this packet.
        // Client generates this and increments it each time it sends another tab completion that doesn't get a response.
        transaction_id: types::VarInt,
        text: types::String, // All text behind the cursor without the / (e.g. to the left of the cursor in left-to-right languages like English).
    },
    AcknowledgeConfiguration,
    ClickContainerButton {
        window_id: types::Byte, // The ID of the window sent by Open Screen.
        button_id: types::Byte, // Meaning depends on window type.
    },
    ClickContainer {
        // The ID of the window which was clicked. 0 for player inventory.
        // The server ignores any packets targeting a Window ID other than the current one, including ignoring 0 when any other window is open.
        window_id: types::Byte,
        state_id: types::Byte, // The last received State ID from either a Set Container Slot or a Set Container Content packet.
        slot: types::Short,    // The clicked slot number
        button: types::Byte,   // The button used in the click
        mode: types::VarInt,
        changed_slots: types::Array<ChangedSlot>, // Maximum length for Notchian server is 128 slots.
        carried_item: types::Slot, // Item carried by the cursor. Has to be empty (item ID = -1) for drop mode, otherwise nothing will happen.
    },
    CloseContainer {
        window_id: types::UnsignedByte, // This is the ID of the window that was closed. 0 for player inventory.
    },
    ChangeContainerSlotState {
        // This packet is sent by the client when toggling the state of a Crafter.
        slot_id: types::VarInt, // This is the ID of the slot that was changed.
        window_id: types::UnsignedByte, // This is the ID of the window that was changed.
        state: types::Boolean,  // The new state of the slot. True for enabled, false for disabled.
    },
    CookieResponse {
        key: types::Identifier,
        payload: types::Optional<types::ByteArray>, // The payload is only present if the cookie exists on the client.
    },
    ServerboundPluginMessage {
        channel: types::Identifier,
        data: types::UnsizedByteArray, // Any data, depending on the channel.
    },
    DebugSampleSubscription {
        sample_type: types::VarInt, // The type of debug sample to subscribe to. Can be one of the following: 0 - Tick time
    },
    EditBook {
        slot: types::VarInt, // The hotbar slot where the written book is located
        entries: types::Array<types::String>, // Text from each page. Maximum string length is 8192 chars.
        title: types::Optional<types::String>, // Title of book. present if book is being signed, absent if book is being edited.
    },
    QueryEntityTag {
        transaction_id: types::VarInt, // An incremental ID so that the client can verify that the response matches.
        entity_id: types::VarInt,      // The ID of the entity to query.
    },
    Interact {
        entity_id: types::VarInt, // The ID of the entity to interact.
        interaction: InteractionEnum,
        is_sneaking: types::Boolean,
    },
    JigsawGenerate {
        location: types::Position,
        levels: types::VarInt,
        keep_jigsaws: types::Boolean,
    },
    KeepAlive {
        keepalive_id: types::Long,
    },
    LockDifficulty {
        locked: types::Boolean,
    },
    SetPlayerPosition {
        position: types::DoubleVec3, // the value for Y is the Absolute feet position, normally Head Y - 1.62.
        on_ground: types::Boolean,
    },
    SetPlayerPositionAndRotation {
        position: types::DoubleVec3, // the value for Y is the Absolute feet position, normally Head Y - 1.62.
        yaw: types::Float,
        ptch: types::Float,
        on_ground: types::Boolean,
    },
    SetPlayerRotation {
        yaw: types::Float,
        ptch: types::Float,
        on_ground: types::Boolean,
    },
    SetPlayerOnGround {
        on_ground: types::Boolean,
    },
    MoveVehicle {
        position: types::DoubleVec3, // Absolute position
        yaw: types::Float,
        ptch: types::Float,
    },
    PaddleBoat {
        left_paddle_turning: types::Boolean,
        right_paddle_turning: types::Boolean,
    },
    PickItem {
        slot: types::VarInt,
    },
    PingRequest {
        payload: types::Long,
    },
    PlaceRecipe {
        window_id: types::Byte,
        recipe: types::Identifier,
        make_all: types::Boolean,
    },
    PlayerAbilities {
        flags: types::Byte, // Bit mask. 0x02: is flying.
    },
    PlayerAction {
        status: types::VarInt,     // The action the player is taking against the block
        location: types::Position, // Block position
        face: types::Byte,         // The face being hit
        sequence: types::VarInt,   // Block change sequence number
    },
    PlayerCommand {
        entity_id: types::VarInt,  // Player ID
        action_id: types::VarInt,  // The ID of the action.
        jump_boost: types::VarInt, // Only used by the “start jump with horse” action, in which case it ranges from 0 to 100. In all other cases it is 0.
    },
    PlayerInput {
        sideways: types::Float,     // Positive to the left of the player.
        forward: types::Float,      // Positive forward.
        flags: types::UnsignedByte, // Bit mask. 0x1: jump, 0x2: unmount.
    },
    Pong {
        id: types::Int,
    },
    ChangeRecipeBookSettings {
        book_id: types::VarInt,
        book_open: types::Boolean,
        filter_active: types::Boolean,
    },
    SetSeenRecipe {
        recipe_id: types::Identifier,
    },
    RenameItem {
        item_name: types::String,
    },
    ResourcePackResponse {
        uuid: types::UUID,
        result: types::VarInt,
    },
    SeenAdvancements {
        action: SeenAdvancementAction,
    },
    SelectTrade {
        selected_slot: types::VarInt,
    },
    SetBeaconEffect {
        primary_effect: types::Optional<types::VarInt>,
        secondary_effect: types::Optional<types::VarInt>,
    },
    SetHeldItem {
        slot: types::Short,
    },
    ProgramCommandBlock {
        location: types::Position,
        command: types::String,
        mode: types::VarInt, // 0 -> One of SEQUENCE, 1 -> AUTO, 2 -> REDSTONE
        flags: types::Byte, // 0x01: Track Output (if false, the output of the previous command will not be stored within the command block); 0x02: Is conditional; 0x04: Automatic.
    },
    ProgramCommandBlockMinecart {
        entity_id: types::VarInt,
        command: types::String,
        track_output: types::Boolean, // If false, the output of the previous command will not be stored within the command block.
    },
    SetCreativeModeSlot {
        slot: types::Short,
        clicked_item: types::Slot,
    },
    ProgramJigsawBlock {
        location: types::Position, // Block entity location
        name: types::Identifier,
        target: types::Identifier,
        pool: types::Identifier,
        final_state: types::String, // "Turns into" on the GUI, final_state in NBT.
        joint_type: types::String,  // rollable if the attached piece can be rotated, else aligned.
        selection_priority: types::VarInt,
        placement_priority: types::VarInt,
    },
    ProgramStructureBlock {
        locatrion: types::Position, // Block entity location
        action: types::VarInt, // An additional action to perform beyond simply saving the given data
        mode: StructureBlockModeEnum,   // One of SAVE (0), LOAD (1), CORNER (2), DATA (3).
        name: types::String,
        offset: types::ByteVec3, // Between -48 and 48.
        size: types::ByteVec3,   // Between 0 and 48.
        mirror: MirrorEnum,   // One of NONE (0), LEFT_RIGHT (1), FRONT_BACK (2).
        rotation: RotationEnum, // One of NONE (0), CLOCKWISE_90 (1), CLOCKWISE_180 (2), COUNTERCLOCKWISE_90 (3).
        metadata: types::String,
        integrity: types::Float, // Between 0 and 1.
        seed: types::VarLong,
        flags: types::Byte, // 0x01: Ignore entities; 0x02: Show air; 0x04: Show bounding box.
    },
    UpdateSign {
        location: types::Position,     // Block Coordinates.
        is_front_text: types::Boolean, // Whether the updated text is in front or on the back of the sign
        line1: types::String,
        line2: types::String,
        line3: types::String,
        line4: types::String,
    },
    SwingArm {
        hand: HandEnum, // Hand used for the animation. 0: main hand, 1: off hand.
    },
    TeleportToEntity {
        target_player: types::UUID, // UUID of the player to teleport to (can also be an entity UUID).
    },
    UseItemOn {
        hand: HandEnum, // The hand from which the block is placed; 0: main hand, 1: off hand.
        location: types::Position,
        face: types::VarInt,
        // The position of the crosshair on the block, from 0 to 1 increasing from west to east, bottom to top and north to south.
        cursor_position: types::FloatVec3,
        inside_block: types::Boolean, // True when the player's head is inside of a block.
        sequence: types::VarInt,      // Block change sequence number
    },
    UseItem {
        hand: HandEnum, // Hand used for the animation. 0: main hand, 1: off hand.
        sequence: types::VarInt, // Block change sequence number
        yaw: types::Float,   // Player head rotation along the Y-Axis.
        pitch: types::Float, // Player head rotation along the X-Axis.
    },
}
