use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

#[derive(PacketSerde, Clone, Debug)]
pub enum HandshakeRequest {  // by default enums start from zero, so we have to specify values here
    STATUS = 1,
    LOGIN = 2,
    TRANSFER = 3,
}

pub fn new_handshake_start_packet(protocol: i32, hostname: &str, port: u16, next_state: HandshakeRequest) -> ServerboundHandshakePacket {
    ServerboundHandshakePacket::HandshakeStart {
        protocol: protocol.into(),
        hostname: hostname.into(),
        port: port.into(),
        next_state: next_state,
    }
}

// ###### Generic Serverbound Handshake Packet ######

#[derive(PacketSerde, Clone, Debug)]
pub enum ServerboundHandshakePacket {
    HandshakeStart {
        protocol: types::VarInt,
        hostname: types::String,
        port: types::UnsignedShort,
        next_state: HandshakeRequest,
    },
}

impl GenericPacket for ServerboundHandshakePacket {}
