use std::io::{Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketOut, PacketSend, PacketWriter};

#[derive(Clone)]
pub struct HandshakeStatusPacket;

impl HandshakeStatusPacket {
    #[inline]
    pub fn new() -> Self {
        Self { }
    }
}

impl Packet for HandshakeStatusPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Handshaking;
}

impl<T: Write + Seek> PacketOut<T> for HandshakeStatusPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketSend for HandshakeStatusPacket {}
