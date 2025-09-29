use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketSerde, PacketReadable, PacketWritable},
    types,
};


#[derive(PacketSerde, Debug, Clone)]
pub struct LoginStartPacket {
    pub username: types::String,
    pub uuid: types::UUID,
}

impl Packet for LoginStartPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Login;
}

#[derive(PacketSerde, Clone, Debug)]
pub struct EncryptionResponsePacket {
    pub shared_secret: types::ByteArray,   // Shared Secret value, encrypted with the server's public key.
    pub verify_token: types::ByteArray,    // Verify Token value, encrypted with the same public key as the shared secret.
}

impl Packet for EncryptionResponsePacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Login;
}

#[derive(PacketSerde, Debug, Clone)]
pub struct LoginPluginResponsePacket {
    pub message_id: types::VarInt,
    pub successful: types::Boolean,
    pub data: types::UnsizedByteArray,
}

impl Packet for LoginPluginResponsePacket {
    const ID: i32 = 0x02;
    const PHASE: ConnectionState = ConnectionState::Login;
}


#[derive(PacketSerde, Clone, Copy, Debug)]
pub struct LoginAcknowledgedPacket;

impl Packet for LoginAcknowledgedPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Login;
}


#[derive(PacketSerde, Clone, Debug)]
pub struct CookieResponsePacket {
    key: types::String,
    payload: types::Optional<types::ByteArray>,
}

impl CookieResponsePacket {
    #[inline]
    pub fn new(key: String, payload: Option<&[u8]>) -> Self {
        Self {
            key: key.into(),
            payload: payload.into(),
        }
    }
}

impl Packet for CookieResponsePacket {
    const ID: i32 = 0x04;
    const PHASE: ConnectionState = ConnectionState::Login;
}