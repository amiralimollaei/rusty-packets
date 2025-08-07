use std::io::{Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketOut, PacketSend, PacketWriter};

#[derive(Clone)]
pub struct LoginStartPacket {
    username: String,
    uuid: u128,
}

impl LoginStartPacket {
    #[inline]
    pub fn new(username: String, uuid: u128) -> Self {
        Self {
            username: username,
            uuid: uuid,
        }
    }
}

impl Packet for LoginStartPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Write + Seek> PacketOut<T> for LoginStartPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_str(self.username.as_str());
        writer.write_uuid(self.uuid);
    }
}

impl PacketSend for LoginStartPacket {}