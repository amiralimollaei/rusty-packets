use std::io::{Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketOut, PacketSend, PacketWriter};

#[derive(Clone)]
pub struct RequestPacket;

impl RequestPacket {
    #[inline]
    pub fn new() -> Self {
        Self { }
    }
}

impl Packet for RequestPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Status;
}

impl<T: Write + Seek> PacketOut<T> for RequestPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketSend for RequestPacket {}
