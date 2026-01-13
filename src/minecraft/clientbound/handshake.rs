use generic_packet_derive::GenericPacket;
use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

// Placeholder for clientbound handshake packets if needed in the future
// Currently, there are no clientbound packets in the handshake phase

// ###### Generic Clientbound Handshake Packet ######

#[derive(PacketSerde, GenericPacket, Debug, Clone)]
pub enum ClientboundHandshakePacket {
    Placeholder
}