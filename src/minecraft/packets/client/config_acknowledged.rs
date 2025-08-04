use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::minecraft::packets::PacketRecv;
use crate::minecraft::packets::PacketSend;
use crate::minecraft::packets::{Packet, PacketIn, PacketOut, ConnectionState, PacketWriter, PacketReader};

#[derive(Clone, Copy)]
pub struct AcknowledgeFinishConfigPacket;

impl AcknowledgeFinishConfigPacket {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Packet for AcknowledgeFinishConfigPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for AcknowledgeFinishConfigPacket {
    fn read(_: &mut PacketReader<T>) -> Self {
        Self::new()
    }
}

impl<T: Write + Seek> PacketOut<T> for AcknowledgeFinishConfigPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // there is nothing to write
    }
}

impl PacketRecv for AcknowledgeFinishConfigPacket {}
impl PacketSend for AcknowledgeFinishConfigPacket {}
