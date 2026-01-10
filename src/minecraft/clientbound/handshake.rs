use packet_serde_derive::PacketSerde;

use crate::minecraft::packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable};

// Placeholder for clientbound handshake packets if needed in the future
// Currently, there are no clientbound packets in the handshake phase

// ###### Generic Clientbound Handshake Packet ######

#[derive(PacketSerde, Debug, Clone)]
pub struct ClientboundHandshakePacket {
    // placeholder
}

impl GenericPacket for ClientboundHandshakePacket {}