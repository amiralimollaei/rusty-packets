use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

// ###### Generic Serverbound Status Packet ######

#[derive(PacketSerde, Clone, Debug)]
pub enum ServerboundStatusPacket {
    StatusRequest,
    Ping { timestamp: types::Long },
}

impl GenericPacket for ServerboundStatusPacket {}
