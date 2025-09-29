use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketSerde, PacketReadable, PacketWritable},
    types,
};


#[derive(PacketSerde, Debug, Clone)]
pub struct RequestPacket;

impl Packet for RequestPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Status;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct PingPacket {
    pub timestamp: types::Long,
}

impl Packet for PingPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Status;
}