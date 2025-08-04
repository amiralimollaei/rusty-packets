use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::minecraft::packets::PacketRecv;
use crate::minecraft::packets::PacketSend;
use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketWriter,
};

#[derive(Clone)]
pub struct SetCompressionPacket {
    threshold: i32,
}

impl SetCompressionPacket {
    #[inline]
    pub fn new(threshold: i32) -> Self {
        Self {
            threshold: threshold,
        }
    }

    pub fn get_threshold(&self) -> i32 {
        self.threshold
    }
}

impl Packet for SetCompressionPacket {
    const ID: i32 = 3;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for SetCompressionPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            threshold: reader.read_varint(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for SetCompressionPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.threshold);
    }
}

impl PacketRecv for SetCompressionPacket {}
impl PacketSend for SetCompressionPacket {}
