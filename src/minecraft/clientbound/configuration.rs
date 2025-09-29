use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketSerde, PacketReadable, PacketWritable},
    types,
};


#[derive(PacketSerde, Clone, Debug)]
pub struct CookieRequestPacket {
    pub key: types::String,
}

impl Packet for CookieRequestPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct ClientboundPluginMessagePacket {
    pub channel: types::Identifier,
    pub data: types::UnsizedByteArray,
}

impl Packet for ClientboundPluginMessagePacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct DisconnectPacket {
    pub reason: types::NBTValue,
}

impl Packet for DisconnectPacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct ConfigurationFinishPacket;

impl Packet for ConfigurationFinishPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct KeepAlivePacket {
    pub keepalive_id: types::Long,
}

impl Packet for KeepAlivePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct PingPacket {
    pub timestamp: types::Int,
}

impl Packet for PingPacket {
    const ID: i32 = 0x05;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct ResetChatPacket;

impl Packet for ResetChatPacket {
    const ID: i32 = 0x06;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct RegistryEntry {
    pub id: types::Identifier,
    pub data: types::Optional<types::NBTValue>
}

#[derive(PacketSerde, Debug, Clone)]
pub struct RegistryDataPacket {
    pub registry_id: types::Identifier,
    pub entries: types::Array<RegistryEntry>,
}

impl Packet for RegistryDataPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct RemoveResourcePackPacket {
    pub uuid: types::Optional<types::UUID>,
}

impl Packet for RemoveResourcePackPacket {
    const ID: i32 = 0x08;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct AddResourcePackPacket {
    pub uuid: types::UUID,
    pub url: types::String,
    pub hash: types::String,
    pub forced: types::Boolean,
    pub prompt_message: types::Optional<types::NBTValue>,
}


impl Packet for AddResourcePackPacket {
    const ID: i32 = 0x09;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct StoreCookiePacket {
    pub key: types::Identifier,
    pub payload: types::ByteArray,
}

impl Packet for StoreCookiePacket {
    const ID: i32 = 0x0A;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct TransferPacket {
    pub host: types::String,
    pub port: types::VarInt,
}

impl Packet for TransferPacket {
    const ID: i32 = 0x0B;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct FeatureFlagsPacket {
    pub feature_flags: types::Array<types::String>
}

impl Packet for FeatureFlagsPacket {
    const ID: i32 = 0x0C;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct RegistryTag {
    pub name: types::Identifier,
    pub entries: types::Array<types::VarInt>, // Numeric IDs of the given type (block, item, etc.). This list replaces the previous list of IDs for the given tag. If some preexisting tags are left unmentioned, a warning is printed.
}

#[derive(PacketSerde, Clone, Debug)]
pub struct RegistryTagMap {
    pub registry: types::Identifier, // Registry identifier (Vanilla expects tags for the registries minecraft:block, minecraft:item, minecraft:fluid, minecraft:entity_type, and minecraft:game_event)
    pub tagsmap: types::Array<RegistryTag>
}

#[derive(PacketSerde, Clone, Debug)]
pub struct UpdateTagsPacket {
    pub feature_flags: types::Array<types::String>
}

impl Packet for UpdateTagsPacket {
    const ID: i32 = 0x0D;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct ClientboundKnownPacksPacket {
    pub namespace: types::String,
    pub id: types::String,
    pub version: types::String,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct KnownServerPacksPacket {
    pub packs: types::Array<ClientboundKnownPacksPacket>,
}

impl Packet for KnownServerPacksPacket {
    const ID: i32 = 0x0E;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct CustomReportDetail {
    pub title: types::String,
    pub description: types::String,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct CustomReportDetailsPacket {
    pub details: types::Array<CustomReportDetail>,
}

impl Packet for CustomReportDetailsPacket {
    const ID: i32 = 0x0F;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct ServerLink {
    pub label: types::Or<types::VarInt, types::NBTValue>, // VarInt for predefined labels, NBT for custom ones
    pub url: types::String,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct ServerLinksPacket {
    pub links: types::Array<ServerLink>,
}

impl Packet for ServerLinksPacket {
    const ID: i32 = 0x10;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}