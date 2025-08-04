use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct ConfigFeatureFlagsPacket {
    feature_flags: Vec<String>
}

impl ConfigFeatureFlagsPacket {
    #[inline]
    pub fn new(feature_flags: Vec<String>) -> Self {
        Self { feature_flags: feature_flags }
    }
}

impl Packet for ConfigFeatureFlagsPacket {
    const ID: i32 = 0x0C;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for ConfigFeatureFlagsPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let feature_flag_count = reader.read_varint() as usize;
        let mut feature_flags: Vec<String> = Vec::with_capacity(feature_flag_count);
        for _ in 0..feature_flag_count {
            feature_flags.push(reader.read_identifier());
        }
        Self::new(feature_flags)
    }
}

impl<T: Write + Seek> PacketOut<T> for ConfigFeatureFlagsPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.feature_flags.len() as i32);
        for identifier in &self.feature_flags {
            writer.write_identifier_str(identifier.as_str());
        }
    }
}

impl PacketRecv for ConfigFeatureFlagsPacket {}
impl PacketSend for ConfigFeatureFlagsPacket {}
