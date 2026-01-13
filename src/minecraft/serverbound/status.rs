use generic_packet_derive::GenericPacket;
use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{GenericPacket, PacketReadable, PacketSerde, PacketWritable},
    types,
};

// ###### Generic Serverbound Status Packet ######

#[derive(PacketSerde, GenericPacket, Clone, Debug)]
pub enum ServerboundStatusPacket {
    StatusRequest,
    Ping { timestamp: types::Long },
}