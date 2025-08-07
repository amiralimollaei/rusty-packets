use std::io::{Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketOut, PacketSend, PacketWriter};


#[derive(Clone)]
pub enum HandshakeRequest {
    STATUS = 1,
    LOGIN = 2,
    TRANSFER = 3,
}

#[derive(Clone)]
pub struct HandshakeStartPacket {
    protocol: i32,
    hostname: String,
    port: u16,
    next_state: HandshakeRequest,
}

impl HandshakeStartPacket {
    #[inline]
    pub fn new(protocol: i32, hostname: &str, port: u16, next_state: HandshakeRequest) -> Self {
        Self {
            protocol: protocol,
            hostname: hostname.to_string(),
            port: port,
            next_state: next_state
        }
    }
}

impl Packet for HandshakeStartPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Handshaking;
}

impl<T: Write + Seek> PacketOut<T> for HandshakeStartPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.protocol);
        writer.write_str(self.hostname.as_str());
        writer.write_ushort(self.port);
        writer.write_varint(self.next_state.clone() as i32);
    }
}

impl PacketSend for HandshakeStartPacket {}
