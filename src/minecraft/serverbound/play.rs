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
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x18;
    const PHASE: ConnectionState = ConnectionState::Play;
}