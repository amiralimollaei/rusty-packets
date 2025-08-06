use std::io::{Read, Seek, Write};

use crate::minecraft::{packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
}, types};

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
        let feature_flags_raw: types::Array<types::Identifier> = reader.read_raw();
        let mut feature_flags: Vec<String> = Vec::with_capacity(feature_flags_raw.len());
        for feature_flag in feature_flags_raw.iter() {
            feature_flags.push(feature_flag.get_value());
        }
        Self::new(feature_flags)
    }
}

impl<T: Write + Seek> PacketOut<T> for ConfigFeatureFlagsPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        let mut feature_flags_raw: Vec<types::Identifier> = Vec::with_capacity(self.feature_flags.len());
        for identifier in &self.feature_flags {
            feature_flags_raw.push(types::Identifier::from_string(identifier.clone()));
        }
        writer.write_raw(types::Array::new(feature_flags_raw));
    }
}

impl PacketRecv for ConfigFeatureFlagsPacket {}
impl PacketSend for ConfigFeatureFlagsPacket {}
