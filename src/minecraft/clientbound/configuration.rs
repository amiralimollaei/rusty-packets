use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

#[derive(PacketSerde, Debug, Clone)]
pub struct RegistryEntry {
    pub id: types::Identifier,
    pub data: types::Optional<types::NBTValue>,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct RegistryTag {
    pub name: types::Identifier,
    pub entries: types::Array<types::VarInt>, // Numeric IDs of the given type (block, item, etc.). This list replaces the previous list of IDs for the given tag. If some preexisting tags are left unmentioned, a warning is printed.
}

#[derive(PacketSerde, Clone, Debug)]
pub struct RegistryTagMap {
    pub registry: types::Identifier, // Registry identifier (Vanilla expects tags for the registries minecraft:block, minecraft:item, minecraft:fluid, minecraft:entity_type, and minecraft:game_event)
    pub tagsmap: types::Array<RegistryTag>,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct ClientboundKnownPacksPacket {
    pub namespace: types::String,
    pub id: types::String,
    pub version: types::String,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct CustomReportDetail {
    pub title: types::String,
    pub description: types::String,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct ServerLink {
    pub label: types::Or<types::VarInt, types::NBTValue>, // VarInt for predefined labels, NBT for custom ones
    pub url: types::String,
}

// ###### Generic Clientbound Configuration Packet ######

#[derive(PacketSerde, Clone, Debug)]
pub enum ClientboundConfigurationPacket {
    CookieRequest {
        key: types::String,
    },
    PluginMessage {
        channel: types::Identifier,
        data: types::UnsizedByteArray,
    },
    Disconnect {
        reason: types::NBTValue,
    },
    ConfigurationFinish,
    KeepAlive {
        keepalive_id: types::Long,
    },
    Ping {
        timestamp: types::Int,
    },
    ResetChat,
    RegistryData {
        registry_id: types::Identifier,
        entries: types::Array<RegistryEntry>,
    },
    RemoveResourcePack {
        uuid: types::Optional<types::UUID>,
    },
    AddResourcePack {
        uuid: types::UUID,
        url: types::String,
        hash: types::String,
        forced: types::Boolean,
        prompt_message: types::Optional<types::NBTValue>,
    },
    StoreCookie {
        key: types::Identifier,
        payload: types::ByteArray,
    },
    Transfer {
        host: types::String,
        port: types::VarInt,
    },
    FeatureFlags {
        flags: types::Array<types::String>,
    },
    UpdateTags {
        tags: types::Array<RegistryTagMap>,
    },
    KnownServerPacks {
        packs: types::Array<ClientboundKnownPacksPacket>,
    },
    CustomReportDetails {
        details: types::Array<CustomReportDetail>,
    },
    ServerLinks {
        links: types::Array<ServerLink>,
    },
}

impl GenericPacket for ClientboundConfigurationPacket {}
