use std::io::{Read, Seek, Write};

use crate::minecraft::{packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend,
    PacketWriter,
}, types};

#[derive(Debug)]
pub struct RemoveResourcePackPacket {
    uuid: Option<u128>,
}

impl RemoveResourcePackPacket {
    #[inline]
    pub fn new(uuid: Option<u128>) -> Self {
        Self { uuid: uuid }
    }
}

impl Packet for RemoveResourcePackPacket {
    const ID: i32 = 0x08;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for RemoveResourcePackPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            uuid: reader.read_raw::<types::Optional<types::UUID>>().into_option()
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for RemoveResourcePackPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_raw_consume(types::Optional::<types::UUID>::from(self.uuid));
    }
}

impl PacketRecv for RemoveResourcePackPacket {}
impl PacketSend for RemoveResourcePackPacket {}
