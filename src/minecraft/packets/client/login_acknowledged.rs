use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::minecraft::packets::PacketRecv;
use crate::minecraft::packets::PacketSend;
use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketWriter,
};

#[derive(Clone, Copy)]
pub struct LoginAcknowledgedPacket;

impl LoginAcknowledgedPacket {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Packet for LoginAcknowledgedPacket {
    const ID: i32 = 0x03;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for LoginAcknowledgedPacket {
    fn read(_: &mut PacketReader<T>) -> Self {
        Self::new()
    }
}

impl<T: Write + Seek> PacketOut<T> for LoginAcknowledgedPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketRecv for LoginAcknowledgedPacket {}
impl PacketSend for LoginAcknowledgedPacket {}
