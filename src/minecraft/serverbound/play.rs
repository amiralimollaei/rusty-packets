use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};


#[derive(MinecraftType, Clone, Debug)]
pub struct ConfirmTeleportationPacket {
    pub teleport_id: types::VarInt, // The ID given by the Synchronize Player Position packet.
}

impl Packet for ConfirmTeleportationPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct QueryBlockEntityTag {
    pub transaction_id: types::VarInt, // An incremental ID so that the client can verify that the response matches.
    pub location: types::Position,     // The location of the block to check.
}

impl Packet for QueryBlockEntityTag {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ChangeDifficultyPacket {
    pub new_difficulty: types::Byte, // 0: peaceful, 1: easy, 2: normal, 3: hard .
}

impl Packet for ChangeDifficultyPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct AcknowledgeMessagePacket {
    pub message_count: types::VarInt,
}

impl Packet for AcknowledgeMessagePacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ChatCommandPacket {
    pub command: types::String,  // The command typed by the client.
}

impl Packet for ChatCommandPacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct CommandArgumentSignature {
    pub name: types::String,     // The name of the argument that is signed by the following signature.
    pub timestamp: types::FixedSizeByteArray<256>,  // The signature that verifies the argument. Always 256 bytes and is not length-prefixed.
}

#[derive(MinecraftType, Clone, Debug)]
pub struct SignedChatCommandPacket {
    pub command: types::String,  // The command typed by the client.
    pub timestamp: types::Long,  // The timestamp that the command was executed.
    pub salt: types::Long,       // The salt for the following argument signatures.
    // The signatures for the command arguments, The maximum length in Notchian server is 8.
    pub argument_signatures: types::Array<CommandArgumentSignature>,
    pub message_count: types::VarInt,
    pub acknowledged: types::FixedSizeBitSet<3>, // Whether the client has acknowledged the command. Always 20 bits (3 bytes)
}

impl Packet for SignedChatCommandPacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ChatMessagePacket {
    pub message: types::String,  // The message typed by the client.
    pub timestamp: types::Long,  // The timestamp that the message was executed.
    pub salt: types::Long,       // The salt used to verify the signature hash.
    // The signature used to verify the chat message's authentication. When present, always 256 bytes and not length-prefixed.
    pub signature: types::Optional<types::FixedSizeByteArray<256>>,
    pub message_count: types::VarInt,
    pub acknowledged: types::FixedSizeBitSet<3>, // Whether the client has acknowledged the message. Always 20 bits (3 bytes)
}

impl Packet for ChatMessagePacket {
    const ID: i32 = 0x06;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SessionPublicKey {
    pub expires_at: types::Long, // The time at which the public key expires, in milliseconds since Unix epoch.
    pub public_key: types::ByteArray, // A byte array of an X.509-encoded public key, Maximum length in Notchian server is 512 bytes.
    // The signature consists of the player UUID, the key expiration timestamp, and the public key data.
    // These values are hashed using SHA-1 and signed using Mojang's private RSA key. Maximum length in Notchian server is 4096 bytes.
    pub key_signature: types::ByteArray,
}

#[derive(MinecraftType, Clone, Debug)]
pub struct PlayerSessionPacket {
    pub session_id: types::UUID, // The player's session UUID.
    pub public_key: SessionPublicKey, // The player's public key.
}

impl Packet for PlayerSessionPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ChunkBatchReceivedPacket {
    pub chunk_per_tick: types::Float, // Desired chunks per tick.
}

impl Packet for ChunkBatchReceivedPacket {
    const ID: i32 = 0x08;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ClientStatusPacket {
    pub action_id: types::VarInt, // 0: perform respawn, 1: request stats
}

impl Packet for ClientStatusPacket {
    const ID: i32 = 0x09;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ClientInformationPacket {
    pub locale: types::String,                 // String: max 16 characters
    pub view_distance: types::Byte,            // Byte: for some reason this HAD TO BE SIGNED
    pub chat_mode: types::VarInt,              // VarInt Enum: 0: enabled, 1: commands only, 2: hidden
    pub chat_colors: types::Boolean,           // Boolean: can the chat be colored?
    pub skin_parts: types::UnsignedByte,       // Unsigned Byte: parts of skin that are visible (7 bit bitflag)
    pub main_hand: types::VarInt,              // VarInt Enum: 0: left, 1: right
    pub text_filtering: types::Boolean,        // Boolean: Enables filtering of text on signs and written book titles
    pub allow_server_listings: types::Boolean, // Boolean: Servers usually list online players, this option should let you not show up in that list
}

impl Packet for ClientInformationPacket {
    const ID: i32 = 0x0A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct CommandSuggestionsRequestPacket {
    // The id of the transaction that the server will send back to the client in the response of this packet.
    // Client generates this and increments it each time it sends another tab completion that doesn't get a response.
    pub transaction_id: types::VarInt,
    pub text: types::String, // All text behind the cursor without the / (e.g. to the left of the cursor in left-to-right languages like English).
}

impl Packet for CommandSuggestionsRequestPacket {
    const ID: i32 = 0x0B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct AcknowledgeConfigurationPacket;

impl Packet for AcknowledgeConfigurationPacket {
    const ID: i32 = 0x0C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ClickContainerButtonPacket {
    pub window_id: types::Byte, // The ID of the window sent by Open Screen.
    pub button_id: types::Byte, // Meaning depends on window type.
}

impl Packet for ClickContainerButtonPacket {
    const ID: i32 = 0x0D;
    const PHASE: ConnectionState = ConnectionState::Play;
}

/*
#[derive(MinecraftType, Clone, Debug)]
pub struct ChangedSlot {
    pub slot_number: types::Byte,
    pub slot: Slot,               // New data for this slot, in the client's opinion.
}

#[derive(MinecraftType, Clone, Debug)]
pub struct ClickContainerPacket {
    // The ID of the window which was clicked. 0 for player inventory.
    // The server ignores any packets targeting a Window ID other than the current one, including ignoring 0 when any other window is open.
    pub window_id: types::Byte,
    pub state_id: types::Byte,  // The last received State ID from either a Set Container Slot or a Set Container Content packet.
    pub slot: types::Short,     // The clicked slot number
    pub button: types::Byte,    // The button used in the click
    pub mode: types::VarInt,
    pub changed_slots: types::Array<ChangedSlot>, // Maximum length for Notchian server is 128 slots.
    pub carried_item: Slot,     // Item carried by the cursor. Has to be empty (item ID = -1) for drop mode, otherwise nothing will happen.
}

impl Packet for ClickContainerPacket {
    const ID: i32 = 0x0E;
    const PHASE: ConnectionState = ConnectionState::Play;
}
*/

#[derive(MinecraftType, Clone, Debug)]
pub struct CloseContainerPacket {
    pub window_id: types::UnsignedByte, // This is the ID of the window that was closed. 0 for player inventory.
}

impl Packet for CloseContainerPacket {
    const ID: i32 = 0x0F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ChangeContainerSlotStatePacket {
    // This packet is sent by the client when toggling the state of a Crafter.
    pub slot_id: types::VarInt, // This is the ID of the slot that was changed.
    pub window_id: types::UnsignedByte, // This is the ID of the window that was changed.
    pub state: types::Boolean,  // The new state of the slot. True for enabled, false for disabled.
}

impl Packet for ChangeContainerSlotStatePacket {
    const ID: i32 = 0x10;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct CookieResponsePacket {
    pub key: types::Identifier,
    pub payload: types::Optional<types::ByteArray>, // The payload is only present if the cookie exists on the client.
}

impl Packet for CookieResponsePacket {
    const ID: i32 = 0x11;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ServerboundPluginMessagePacket {
    pub channel: types::Identifier,
    pub data: types::UnsizedByteArray, // Any data, depending on the channel.
}

impl Packet for ServerboundPluginMessagePacket {
    const ID: i32 = 0x12;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct DebugSampleSubscriptionPacket {
    pub sample_type: types::VarInt, // The type of debug sample to subscribe to. Can be one of the following: 0 - Tick time
}

impl Packet for DebugSampleSubscriptionPacket {
    const ID: i32 = 0x13;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct EditBookPacket {
    pub slot: types::VarInt, // The hotbar slot where the written book is located
    pub entries: types::Array<types::String>, // Text from each page. Maximum string length is 8192 chars.
    pub title: types::Optional<types::String> // Title of book. present if book is being signed, absent if book is being edited.

}

impl Packet for EditBookPacket {
    const ID: i32 = 0x14;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct QueryEntityTagPacket {
    pub transaction_id: types::VarInt, // An incremental ID so that the client can verify that the response matches.
    pub entity_id: types::VarInt // The ID of the entity to query.
}

impl Packet for QueryEntityTagPacket {
    const ID: i32 = 0x15;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
#[discriminant_type(types::VarInt)]
pub enum InteractionEnum {
    Interact {hand: types::VarInt},
    Attack,
    InteractAt {target: types::FloatVec3, hand: types::VarInt},
}


#[derive(MinecraftType, Clone, Debug)]
pub struct InteractPacket {
    pub entity_id: types::VarInt,      // The ID of the entity to interact.
    pub interaction: InteractionEnum,
    pub is_sneaking: types::Boolean,
}

impl Packet for InteractPacket {
    const ID: i32 = 0x16;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct JigsawGeneratePacket {
    pub location: types::Position,
    pub levels: types::VarInt,
    pub keep_jigsaws: types::Boolean,
}

impl Packet for JigsawGeneratePacket {
    const ID: i32 = 0x17;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x18;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct LockDifficultyPacket {
    pub locked: types::Boolean,
}

impl Packet for LockDifficultyPacket {
    const ID: i32 = 0x19;
    const PHASE: ConnectionState = ConnectionState::Play;
}

#[derive(MinecraftType, Clone, Debug)]
pub struct SetPlayerPositionPacket {
    pub position: types::DoubleVec3, // the value for Y is the Absolute feet position, normally Head Y - 1.62.
    pub on_ground: types::Boolean
}

impl Packet for SetPlayerPositionPacket {
    const ID: i32 = 0x1A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SetPlayerPositionAndRotationPacket {
    pub position: types::DoubleVec3, // the value for Y is the Absolute feet position, normally Head Y - 1.62.
    pub yaw: types::Float,
    pub ptch: types::Float,
    pub on_ground: types::Boolean
}

impl Packet for SetPlayerPositionAndRotationPacket {
    const ID: i32 = 0x1B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SetPlayerRotationPacket {
    pub yaw: types::Float,
    pub ptch: types::Float,
    pub on_ground: types::Boolean
}

impl Packet for SetPlayerRotationPacket {
    const ID: i32 = 0x1C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SetPlayerOnGroundPacket {
    pub on_ground: types::Boolean
}

impl Packet for SetPlayerOnGroundPacket {
    const ID: i32 = 0x1D;
    const PHASE: ConnectionState = ConnectionState::Play;
}

#[derive(MinecraftType, Clone, Debug)]
pub struct MoveVehiclePacket {
    pub position: types::DoubleVec3, // Absolute position
    pub yaw: types::Float,
    pub ptch: types::Float,
}

impl Packet for MoveVehiclePacket {
    const ID: i32 = 0x1E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PaddleBoatPacket {
    pub left_paddle_turning: types::Boolean,
    pub right_paddle_turning: types::Boolean,
}

impl Packet for PaddleBoatPacket {
    const ID: i32 = 0x1F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PickItemPacket {
    pub slot: types::VarInt,
}

impl Packet for PickItemPacket {
    const ID: i32 = 0x20;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PingRequestPacket {
    pub payload: types::Long,
}

impl Packet for PingRequestPacket {
    const ID: i32 = 0x21;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PlaceRecipePacket {
    pub window_id: types::Byte,
    pub recipe: types::Identifier,
    pub make_all: types::Boolean
}

impl Packet for PlaceRecipePacket {
    const ID: i32 = 0x22;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PlayerAbilitiesPacket {
    pub flags: types::Byte,    // Bit mask. 0x02: is flying.
}

impl Packet for PlayerAbilitiesPacket {
    const ID: i32 = 0x23;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PlayerActionPacket {
    pub status: types::VarInt,      // The action the player is taking against the block
    pub location: types::Position,  // Block position
    pub face: types::Byte,          // The face being hit
    pub sequence: types::VarInt,    // Block change sequence number
}

impl Packet for PlayerActionPacket {
    const ID: i32 = 0x24;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PlayerCommandPacket {
    pub entity_id: types::VarInt,   // Player ID
    pub action_id: types::VarInt,   // The ID of the action.
    pub jump_boost: types::VarInt,  // Only used by the “start jump with horse” action, in which case it ranges from 0 to 100. In all other cases it is 0.
}

impl Packet for PlayerCommandPacket {
    const ID: i32 = 0x25;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PlayerInputPacket {
    pub sideways: types::Float,      // Positive to the left of the player.
    pub forward: types::Float,       // Positive forward.
    pub flags: types::UnsignedByte,  // Bit mask. 0x1: jump, 0x2: unmount.
}

impl Packet for PlayerInputPacket {
    const ID: i32 = 0x26;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PongPacket {
    pub id: types::Int,
}

impl Packet for PongPacket {
    const ID: i32 = 0x27;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ChangeRecipeBookSettingsPacket {
    pub book_id: types::VarInt,
    pub book_open: types::Boolean,
    pub filter_active: types::Boolean,
}

impl Packet for ChangeRecipeBookSettingsPacket {
    const ID: i32 = 0x28;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SetSeenRecipePacket {
    pub recipe_id: types::Identifier,
}

impl Packet for SetSeenRecipePacket {
    const ID: i32 = 0x29;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct RenameItemPacket {
    pub item_name: types::String,
}

impl Packet for RenameItemPacket {
    const ID: i32 = 0x2A;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ResourcePackResponsePacket {
    pub uuid: types::UUID,
    pub result: types::VarInt,
}

impl Packet for ResourcePackResponsePacket {
    const ID: i32 = 0x2B;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub enum SeenAdvancementAction {
    OpenedTab { tab_id: types::Identifier },
    ClosedScreen
}

#[derive(MinecraftType, Clone, Debug)]
pub struct SeenAdvancementsPacket {
    pub action: SeenAdvancementAction,
}

impl Packet for SeenAdvancementsPacket {
    const ID: i32 = 0x2C;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SelectTradePacket {
    pub selected_slot: types::VarInt,
}

impl Packet for SelectTradePacket {
    const ID: i32 = 0x2D;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SetBeaconEffectPacket {
    pub primary_effect: types::Optional<types::VarInt>,
    pub secondary_effect: types::Optional<types::VarInt>,
}

impl Packet for SetBeaconEffectPacket {
    const ID: i32 = 0x2E;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SetHeldItemPacket {
    pub slot: types::Short
}

impl Packet for SetHeldItemPacket {
    const ID: i32 = 0x2F;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ProgramCommandBlockPacket {
    pub location: types::Position,
    pub command: types::String,
    pub mode: types::VarInt, // 0 -> One of SEQUENCE, 1 -> AUTO, 2 -> REDSTONE
    pub flags: types::Byte, // 0x01: Track Output (if false, the output of the previous command will not be stored within the command block); 0x02: Is conditional; 0x04: Automatic.
}

impl Packet for ProgramCommandBlockPacket {
    const ID: i32 = 0x30;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ProgramCommandBlockMinecartPacket {
    pub entity_id: types::VarInt,
    pub command: types::String,
    pub track_output: types::Boolean,  // If false, the output of the previous command will not be stored within the command block.
}

impl Packet for ProgramCommandBlockMinecartPacket {
    const ID: i32 = 0x31;
    const PHASE: ConnectionState = ConnectionState::Play;
}

/*
#[derive(MinecraftType, Clone, Debug)]
pub struct SetCreativeModeSlotPacket {
    pub slot: types::Short,
    pub clicked_item: Slot,
}

impl Packet for SetCreativeModeSlotPacket {
    const ID: i32 = 0x32;
    const PHASE: ConnectionState = ConnectionState::Play;
}
*/

#[derive(MinecraftType, Clone, Debug)]
pub struct ProgramJigsawBlockPacket {
    pub location: types::Position,   // Block entity location
    pub name: types::Identifier,
    pub target: types::Identifier,
    pub pool: types::Identifier,
    pub final_state: types::String,  // "Turns into" on the GUI, final_state in NBT.
    pub joint_type: types::String,   // rollable if the attached piece can be rotated, else aligned.
    pub selection_priority: types::VarInt,
    pub placement_priority: types::VarInt,
}

impl Packet for ProgramJigsawBlockPacket {
    const ID: i32 = 0x33;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ProgramStructureBlockPacket {
    pub locatrion: types::Position,  // Block entity location
    pub action: types::VarInt,       // An additional action to perform beyond simply saving the given data
    pub mode: types::VarInt,         // One of SAVE (0), LOAD (1), CORNER (2), DATA (3).
    pub name: types::String,
    pub offset: types::ByteVec3,     // Between -48 and 48.
    pub size: types::ByteVec3,       // Between 0 and 48.
    pub mirror: types::VarInt,       // One of NONE (0), LEFT_RIGHT (1), FRONT_BACK (2).
    pub rotation: types::VarInt,     // One of NONE (0), CLOCKWISE_90 (1), CLOCKWISE_180 (2), COUNTERCLOCKWISE_90 (3).
    pub metadata: types::String,
    pub integrity: types::Float,     // Between 0 and 1.
    pub seed: types::VarLong,
    pub flags: types::Byte,          // 0x01: Ignore entities; 0x02: Show air; 0x04: Show bounding box.
}

impl Packet for ProgramStructureBlockPacket {
    const ID: i32 = 0x34;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct UpdateSignPacket {
    pub location: types::Position,      // Block Coordinates.
    pub is_front_text: types::Boolean,  // Whether the updated text is in front or on the back of the sign
    pub line1: types::String,
    pub line2: types::String,
    pub line3: types::String,
    pub line4: types::String,
}

impl Packet for UpdateSignPacket {
    const ID: i32 = 0x35;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct SwingArmPacket {
    pub hand: types::VarInt,  // Hand used for the animation. 0: main hand, 1: off hand.
}

impl Packet for SwingArmPacket {
    const ID: i32 = 0x36;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct TeleportToEntityPacket {
    pub target_player: types::UUID,  // UUID of the player to teleport to (can also be an entity UUID).
}

impl Packet for TeleportToEntityPacket {
    const ID: i32 = 0x37;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct UseItemOnPacket {
    pub hand: types::VarInt,  // The hand from which the block is placed; 0: main hand, 1: off hand.
    pub location: types::Position,
    pub face: types::VarInt,
    // The position of the crosshair on the block, from 0 to 1 increasing from west to east, bottom to top and north to south. 
    pub cursor_position: types::FloatVec3,
    pub inside_block: types::Boolean,  // True when the player's head is inside of a block.
    pub sequence: types::VarInt,  // Block change sequence number
}

impl Packet for UseItemOnPacket {
    const ID: i32 = 0x38;
    const PHASE: ConnectionState = ConnectionState::Play;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct UseItemPacket {
    pub hand: types::VarInt,      // Hand used for the animation. 0: main hand, 1: off hand.
    pub sequence: types::VarInt,  // Block change sequence number
    pub yaw: types::Float,        // Player head rotation along the Y-Axis.
    pub pitch: types::Float       // Player head rotation along the X-Axis.
}

impl Packet for UseItemPacket {
    const ID: i32 = 0x39;
    const PHASE: ConnectionState = ConnectionState::Play;
}
