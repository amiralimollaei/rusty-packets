use std::io::{Read, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketReader, PacketRecv};

const UNWRAP_ERROR: &str = "KnownServerPacksPacket: Unexpected error while reading value.";

#[derive(Clone, Debug)]
pub struct KnownServerPack {
    namespace: String,
    id: String,
    version: String,
}

#[derive(Debug)]
pub struct KnownServerPacksPacket {
    packs: Vec<KnownServerPack>,
}

impl KnownServerPacksPacket {
    #[inline]
    pub fn new(packs: Vec<KnownServerPack>) -> Self {
        Self { packs }
    }

    pub fn get_packs(&self) -> Vec<KnownServerPack> {
        self.packs.clone()
    }
}

impl Packet for KnownServerPacksPacket {
    const ID: i32 = 0x0E;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for KnownServerPacksPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let packs_count = reader.read_varint() as usize;
        let mut packs: Vec<KnownServerPack> = Vec::with_capacity(packs_count);
        for _ in 0..packs_count {
            packs.push(KnownServerPack {
                namespace: reader.read_string(),
                id: reader.read_string(),
                version: reader.read_string(),
            });
        }
        Self { packs: packs }
    }
}

impl PacketRecv for KnownServerPacksPacket {}
