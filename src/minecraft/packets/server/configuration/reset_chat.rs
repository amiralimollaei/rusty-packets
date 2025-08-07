use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct ResetChatPacket;

impl ResetChatPacket {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Packet for ResetChatPacket {
    const ID: i32 = 0x06;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ResetChatPacket {
    fn read(_: &mut PacketReader<T>) -> Self {
        Self::new()
    }
}

impl<T: Write + Seek> PacketOut<T> for ResetChatPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketRecv for ResetChatPacket {}
impl PacketSend for ResetChatPacket {}
