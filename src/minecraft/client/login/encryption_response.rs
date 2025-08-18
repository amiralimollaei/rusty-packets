use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packets::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct EncryptionResponsePacket {
    pub shared_secret: types::ByteArray,   // Shared Secret value, encrypted with the server's public key.
    pub verify_token: types::ByteArray,    // Verify Token value, encrypted with the same public key as the shared secret.
}

impl Packet for EncryptionResponsePacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Login;
}