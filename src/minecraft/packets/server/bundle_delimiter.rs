use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

#[derive(Debug)]
pub struct BundleDelimiterPacket;

impl BundleDelimiterPacket {
    #[inline]
    pub fn new() -> Self {
        Self { }
    }
}

impl Packet for BundleDelimiterPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for BundleDelimiterPacket {
    fn read(_: &mut PacketReader<T>) -> Self {
        Self::new()
    }
}

impl<T: Write + Seek> PacketOut<T> for BundleDelimiterPacket {
    fn write(&self, _: &mut PacketWriter<T>) {
        // nothing to write
    }
}

impl PacketRecv for BundleDelimiterPacket {}
impl PacketSend for BundleDelimiterPacket {}