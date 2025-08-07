use std::io::{Write, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketOut, PacketWriter, PacketSend};

const UNWRAP_ERROR: &str = "KnownServerPacksPacket: Unexpected error while reading value.";

#[derive(Clone, Debug)]
pub struct KnownClientPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

#[derive(Debug)]
pub struct KnownClientPacksPacket {
    packs: Vec<KnownClientPack>,
}

impl KnownClientPacksPacket {
    #[inline]
    pub fn new(packs: Vec<KnownClientPack>) -> Self {
        Self { packs }
    }

    pub fn get_packs(&self) -> Vec<KnownClientPack> {
        self.packs.clone()
    }
}

impl Packet for KnownClientPacksPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Write + Seek> PacketOut<T> for KnownClientPacksPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.packs.len() as i32);
        for pack in &self.packs {
            writer.write_str(&pack.namespace);
            writer.write_str(&pack.id);
            writer.write_str(&pack.version);
        }
    }
}

impl PacketSend for KnownClientPacksPacket {}
