use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

#[derive(MinecraftType, Clone, Debug)]
pub struct FeatureFlagsPacket {
    pub feature_flags: types::Array<types::String>
}

impl Packet for FeatureFlagsPacket {
    const ID: i32 = 0x0C;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}