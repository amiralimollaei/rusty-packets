use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::packet::{ConnectionState, Packet, PacketReadable, PacketWritable};

#[derive(MinecraftType, Clone, Copy)]
pub struct ConfigurationAcknowledgedPacket;

impl Packet for ConfigurationAcknowledgedPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}
