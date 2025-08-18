use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::packet::{ConnectionState, Packet, PacketReadable, PacketWritable};

#[derive(MinecraftType, Clone, Copy, Debug)]
pub struct LoginAcknowledgedPacket;

impl Packet for LoginAcknowledgedPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Login;
}
