use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

// ###### Generic Serverbound Login Packet ######

#[derive(PacketSerde, Clone, Debug)]
pub enum ServerboundLoginPacket {
    LoginStart {
        username: types::String,
        uuid: types::UUID,
    },
    EncryptionResponse {
        shared_secret: types::ByteArray, // Shared Secret value, encrypted with the server's public key.
        verify_token: types::ByteArray, // Verify Token value, encrypted with the same public key as the shared secret.
    },
    LoginPluginResponse {
        message_id: types::VarInt,
        successful: types::Boolean,
        data: types::UnsizedByteArray,
    },
    LoginAcknowledged,
    CookieResponse {
        key: types::Identifier,
        payload: types::Optional<types::ByteArray>,
    },
}

impl GenericPacket for ServerboundLoginPacket {}
