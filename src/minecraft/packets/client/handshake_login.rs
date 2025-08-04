use std::io::{Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketOut, PacketSend, PacketWriter};

#[derive(Clone)]
pub enum HandshakeRequest {
    STATUS,
    LOGIN(String, u128),
    TRANSFER,
}

#[derive(Clone)]
pub struct HandshakeLoginPacket {
    username: String,
    uuid: u128,
}

impl HandshakeLoginPacket {
    #[inline]
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username: username,
            uuid: uuid,
        }
    }
}

impl Packet for HandshakeLoginPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Handshaking;
}

impl<T: Write + Seek> PacketOut<T> for HandshakeLoginPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_str(self.username.as_str());
        writer.write_uuid(self.uuid);
    }
}

impl PacketSend for HandshakeLoginPacket {}