use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::packet::{ConnectionState, Packet, PacketReadable, PacketWritable};

#[derive(MinecraftType, Clone, Debug)]
pub struct ResetChatPacket;

impl Packet for ResetChatPacket {
    const ID: i32 = 0x06;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}
