use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct FinishConfigurationPacket;

impl FinishConfigurationPacket {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Packet for FinishConfigurationPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for FinishConfigurationPacket {
    fn read(_: &mut PacketReader<T>) -> Self {
        Self::new()
    }
}

impl<T: Write + Seek> PacketOut<T> for FinishConfigurationPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketRecv for FinishConfigurationPacket {}
impl PacketSend for FinishConfigurationPacket {}
