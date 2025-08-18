mod packet;

use std::io::{Cursor, Read, Seek, Write};

pub use packet::{PacketContainer, PacketReader, PacketWriter, PacketWritable, PacketReadable};

use crate::minecraft::types::MinecraftType;

static mut THRESHOLD: i32 = -1;

pub fn get_threshold() -> i32 {
    unsafe { THRESHOLD }
}

pub fn set_threshold(thr: i32) {
    unsafe { THRESHOLD = thr }
}

pub trait Packet {
    const ID: i32;
    const PHASE: ConnectionState;
}

pub trait PacketIn<T: Read + Seek>: Packet {
    fn read(reader: &mut PacketReader<T>) -> Self;
}

pub trait PacketRecv: PacketIn<Cursor<Vec<u8>>> {
    #[inline]
    fn recv<S: Read>(stream: &mut S) -> Self where Self: Sized {
        let mut packet_container = PacketContainer::recv(stream);
        let mut packet_reader = packet_container.ger_reader();
        Self::read(&mut packet_reader)
    }

    fn from_packet(packet: PacketContainer) -> Self where Self: Sized {
        Self::read(&mut PacketReader::from_stream(packet.as_stream()))
    }
}

impl<U: Packet + MinecraftType> PacketRecv for U {}

impl<T: Read + Seek, U: Packet + MinecraftType> PacketIn<T> for U {
    fn read(reader: &mut PacketReader<T>) -> Self {
        reader.read_raw::<Self>()
    }
}

pub trait PacketOut<T: Write + Seek>: Packet {
    fn write(&self, writer: &mut PacketWriter<T>);
}

pub trait PacketSend: for<'a> PacketOut<&'a mut Cursor<Vec<u8>>> {
    #[inline]
    fn send<S: Write>(&self, stream: &mut S) {
        let mut packet_stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let mut packet_writer = PacketWriter::from_stream(&mut packet_stream);
        self.write(&mut packet_writer);
        PacketContainer::new(
            Self::ID,
            packet_stream.get_ref().clone()
        ).send(stream);
    }
}

impl<U: Packet + MinecraftType> PacketSend for U {}

impl<T: Write + Seek, U: Packet + MinecraftType> PacketOut<T> for U {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_raw::<Self>(&self);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}
