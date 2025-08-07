use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct ConfigurationFinishPacket;

impl ConfigurationFinishPacket {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Packet for ConfigurationFinishPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ConfigurationFinishPacket {
    fn read(_: &mut PacketReader<T>) -> Self {
        Self::new()
    }
}

impl<T: Write + Seek> PacketOut<T> for ConfigurationFinishPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketRecv for ConfigurationFinishPacket {}
impl PacketSend for ConfigurationFinishPacket {}
