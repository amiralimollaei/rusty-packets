use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketSerde, PacketReadable, PacketWritable},
    types,
};


#[derive(Clone, Debug)]
pub enum HandshakeRequest {
    STATUS = 1,
    LOGIN = 2,
    TRANSFER = 3,
}

#[derive(PacketSerde, Clone, Debug)]
pub struct HandshakeStartPacket {
    protocol: types::VarInt,
    hostname: types::String,
    port: types::UnsignedShort,
    next_state: types::VarInt,
}

impl HandshakeStartPacket {
    #[inline]
    pub fn new(protocol: i32, hostname: &str, port: u16, next_state: HandshakeRequest) -> Self {
        Self {
            protocol: protocol.into(),
            hostname: hostname.into(),
            port: port.into(),
            next_state: (next_state as i32).into()
        }
    }
}

impl Packet for HandshakeStartPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Handshaking;
}
