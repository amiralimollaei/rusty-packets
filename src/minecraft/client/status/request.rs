use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::packets::{ConnectionState, Packet};

#[derive(MinecraftType, Debug, Clone)]
pub struct RequestPacket;

impl RequestPacket {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Packet for RequestPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Status;
}
