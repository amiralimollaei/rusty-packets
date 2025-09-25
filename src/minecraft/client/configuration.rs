use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientMainHand {
    Left = 0,
    Right = 1,
}

#[derive(MinecraftType, Clone, Debug)]
pub struct ClientInformationPacket {
    pub locale: types::String,                 // String: max 16 characters
    pub view_distance: types::Byte,            // Byte: for some reason this HAD TO BE SIGNED
    pub chat_mode: types::VarInt, // VarInt Enum: 0: enabled, 1: commands only, 2: hidden
    pub chat_colors: types::Boolean, // Boolean: can the chat be colored?
    pub skin_parts: types::UnsignedByte, // Unsigned Byte: parts of skin that are visible (7 bit bitflag)
    pub main_hand: types::VarInt,        // VarInt Enum: 0: left, 1: right
    pub text_filtering: types::Boolean, // Boolean: Enables filtering of text on signs and written book titles
    pub allow_server_listings: types::Boolean, // Boolean: Servers usually list online players, this option should let you not show up in that list
}

impl Packet for ClientInformationPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct CookieResponsePacket {
    pub key: types::String,
    pub payload: types::Optional<types::ByteArray>,
}

impl Packet for CookieResponsePacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ServerboundPluginMessagePacket {
    pub channel: types::Identifier,
    pub data: types::UnsizedByteArray,
}

impl Packet for ServerboundPluginMessagePacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Copy)]
pub struct AcknowledgeFinishConfigurationPacket;

impl Packet for AcknowledgeFinishConfigurationPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ServerboundKeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for ServerboundKeepAlivePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct PongPacket {
    pub timestamp: types::Int,
}

impl Packet for PongPacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ResourcePackResponsePacket {
    pub uuid: types::UUID,
    // TODO: make varint enum
    pub result: types::VarInt, // VarInt Enum: 0: accepted, 1: declined, 2: failed download, 3: successfully loaded etc.
}

impl Packet for ResourcePackResponsePacket {
    const ID: i32 = 0x06;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(MinecraftType, Clone, Debug)]
pub struct ServerboundKnownPacksPacket {
    pub namespace: types::String,
    pub id: types::String,
    pub version: types::String,
}

#[derive(MinecraftType, Clone, Debug)]
pub struct KnownClientPacksPacket {
    pub packs: types::Array<ServerboundKnownPacksPacket>,
}

impl Packet for KnownClientPacksPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}
