use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

#[derive(PacketSerde, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(PacketSerde, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientMainHand {
    Left,
    Right,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct ServerboundKnownPack {
    pub namespace: types::String,
    pub id: types::String,
    pub version: types::String,
}

#[derive(PacketSerde, Clone, Debug)]
pub enum ResourcePackResult {
    Accepted,
    Declined,
    FailedDownload,
    SuccessfullyLoaded,
}

// ###### Generic Serverbound Configuration Packet ######

#[derive(PacketSerde, Clone, Debug)]
pub enum ServerboundConfigurationPacket {
    ClientInformation {
        locale: types::String,                  // String: max 16 characters
        view_distance: types::Byte,             // Byte: for some reason this HAD TO BE SIGNED
        chat_mode: ClientChatMode,              // VarInt Enum: 0: enabled, 1: commands only, 2: hidden
        chat_colors: types::Boolean,            // Boolean: can the chat be colored?
        skin_parts: types::UnsignedByte,        // Unsigned Byte: parts of skin that are visible (7 bit bitflag)
        main_hand: ClientMainHand,              // VarInt Enum: 0: left, 1: right
        text_filtering: types::Boolean,         // Boolean: Enables filtering of text on signs and written book titles
        allow_server_listings: types::Boolean,  // Boolean: Servers usually list online players, this option should let you not show up in that list
    },
    CookieResponse {
        key: types::String,
        payload: types::Optional<types::ByteArray>,
    },
    ServerboundPluginMessage {
        channel: types::Identifier,
        data: types::UnsizedByteArray,
    },
    AcknowledgeFinishConfiguration,
    ServerboundKeepAlive {
        keepalive_id: types::Long,
    },
    Pong {
        timestamp: types::Int,
    },
    ResourcePackResponse {
        uuid: types::UUID,
        result: ResourcePackResult,  // VarInt Enum: 0: accepted, 1: declined, 2: failed download, 3: successfully loaded etc.
    },
    KnownClientPacks {
        packs: types::Array<ServerboundKnownPack>,
    },
}

impl GenericPacket for ServerboundConfigurationPacket {}
