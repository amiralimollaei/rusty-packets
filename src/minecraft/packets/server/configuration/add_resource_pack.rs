use std::io::{Read, Seek, Write};

use crate::minecraft::{
    packets::{
        ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend,
        PacketWriter,
    },
    types,
};

#[derive(Debug)]
pub struct AddResourcePackPacket {
    uuid: u128,
    url: String,
    hash: String,
    forced: bool,
    prompt_message: Option<types::NBTValue>,
}

impl AddResourcePackPacket {
    #[inline]
    pub fn new(
        uuid: u128,
        url: String,
        hash: String,
        forced: bool,
        prompt_message: Option<types::NBTValue>,
    ) -> Self {
        Self {
            uuid: uuid,
            url: url,
            hash: hash,
            forced: forced,
            prompt_message: prompt_message,
        }
    }
}

impl Packet for AddResourcePackPacket {
    const ID: i32 = 0x09;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for AddResourcePackPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            uuid: reader.read_uuid(),
            url: reader.read_string(),
            hash: reader.read_string(),
            forced: reader.read_boolean(),
            prompt_message: reader.read_raw::<types::Optional<types::NBTValue>>().into()
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for AddResourcePackPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_uuid(self.uuid);
        writer.write_str(&self.url);
        writer.write_str(&self.hash);
        writer.write_boolean(self.forced);
        writer.write_raw_consume::<types::Optional<types::NBTValue>>(self.prompt_message.clone().into())
    }
}

impl PacketRecv for AddResourcePackPacket {}
impl PacketSend for AddResourcePackPacket {}
