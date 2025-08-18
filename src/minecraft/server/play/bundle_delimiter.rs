use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::packets::{ConnectionState, Packet, PacketReadable, PacketWritable};

#[derive(MinecraftType, Debug, Clone)]
pub struct BundleDelimiterPacket;

impl Packet for BundleDelimiterPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Play;
}
