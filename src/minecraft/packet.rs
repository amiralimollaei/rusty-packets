// packet implementation based on https://minecraft.wiki/w/Java_Edition_protocol/Packets?oldid=2789623
use crate::minecraft::types;
use crate::utils::ansi::string::AnsiString;
use crate::utils::logging::get_logger;

use flate2::Compression;
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::fmt::Debug;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

pub trait PacketWritable
where
    Self: Sized,
{
    fn write(&self, stream: &mut impl Write);
    fn to_bytes(&self) -> Vec<u8> {
        // create a memory stream
        let mut stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        self.write(&mut stream);
        // go back to the start of the memory stream
        stream.seek(SeekFrom::Start(0)).unwrap();
        // return the memory
        stream.into_inner()
    }
}

pub trait PacketReadable
where
    Self: Sized,
{
    fn read(stream: &mut impl Read) -> Self;
    fn from_bytes(bytes: Vec<u8>) -> Self {
        // create a memory stream
        let mut stream: Cursor<Vec<u8>> = Cursor::new(bytes);
        // read the memory stream
        Self::read(&mut stream)
    }
}

pub trait PacketSerde: PacketReadable + PacketWritable {}

#[derive(Clone, Debug)]
pub struct RawPacket {
    // Raw Packet Data (Packet ID as VarInt and Packet Data as a ByteArray)
    raw_data: Vec<u8>,
}

impl RawPacket {
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Self { raw_data: data }
    }

    pub fn to_stream(self) -> Cursor<Vec<u8>> {
        Cursor::new(self.raw_data)
    }

    pub fn to_stream_mut(&mut self) -> Cursor<&mut Vec<u8>> {
        Cursor::new(&mut self.raw_data)
    }

    pub fn to_string(&self) -> String {
        let (id, data) = self.parse();
        if data.len() > 100 {
            let data = &data[0..100];
            let data: Vec<String> = data.iter().map(|x| format!("{:02x}", x)).collect();
            format!("Packet(ID={:#02x}, DATA=[{} ...])", id, data.join(" "))
        } else {
            let data = &data;
            let data: Vec<String> = data.iter().map(|x| format!("{:02x}", x)).collect();
            format!("Packet(ID={:#02x}, DATA=[{}])", id, data.join(" "))
        }
    }

    fn get_data(&self) -> Vec<u8> {
        self.raw_data.clone()
    }

    fn parse(&self) -> (i32, Vec<u8>) {
        let stream = &mut Cursor::new(self.raw_data.clone());
        let id = types::VarInt::read(stream).get_value();
        let mut data = Vec::new();
        stream.read_to_end(&mut data).unwrap();
        (id, data)
    }

    fn get_data_ref(&self) -> &Vec<u8> {
        &self.raw_data
    }

    fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.raw_data
    }

    pub fn write_without_compression(&mut self, stream: &mut impl Write) {
        // write packet length as varint
        types::Length::from_i32(self.raw_data.len() as i32).write(stream);
        // write packet
        stream
            .write_all(&mut self.raw_data)
            .expect("Error writing packet data.");
    }

    fn get_compressed_packet(&mut self) -> Vec<u8> {
        // encode raw packet (Packet ID + Packet Data)
        let mut compressed_packet: Vec<u8> = Vec::with_capacity(self.raw_data.len());
        // compress packet id + packet data
        let mut zlib_encoder =
            ZlibEncoder::new(Cursor::new(&mut self.raw_data), Compression::default());
        zlib_encoder
            .read_to_end(&mut compressed_packet)
            .expect("error while compressing packet!");
        compressed_packet
    }

    // https://wiki.vg/Protocol#Packet_format
    pub fn write_with_compression(mut self, stream: &mut impl Write) {
        let is_compressed = self.raw_data.len() > get_compression_threshold() as usize;
        let mut packet_cmp = if is_compressed {
            self.get_compressed_packet()
        } else {
            self.raw_data
        };
        let mut packet_stream: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(packet_cmp.len() + 4));
        // write data length as varint
        types::Length::from_i32(if is_compressed {
            packet_cmp.len() as i32
        } else {
            0
        })
        .write(&mut packet_stream);
        // write packet id + data
        packet_stream
            .write_all(&mut packet_cmp)
            .expect("Error writing packet data.");
        let mut actual_packet: Vec<u8> = packet_stream.into_inner();
        // write actual length as varint
        types::Length::from_i32(actual_packet.len() as i32).write(stream);
        // write data length + packet id + data
        stream
            .write_all(&mut actual_packet)
            .expect("Error writing packet data.");
    }

    pub fn send(mut self, stream: &mut impl Write) {
        if get_logger().is_debug() {
            get_logger().debug(
                AnsiString::new_colorless("[")
                    + AnsiString::new_fore("🠩", (0, 255, 0))
                    + AnsiString::new_colorless("] ")
                    + AnsiString::new_colorless(&self.to_string()),
            );
        }
        let compression_enbaled = get_compression_threshold() > 0;
        if compression_enbaled {
            self.write_with_compression(stream)
        } else {
            self.write_without_compression(stream)
        }
    }

    #[inline]
    pub fn from_stream_without_compression(stream: &mut impl Read) -> Self {
        // read packet length as varint
        let packet_length = types::Length::read(stream).get_value();
        // read packet
        let mut data: Vec<u8> = Vec::with_capacity(packet_length as usize);
        for _ in 0..packet_length {
            let mut bytes: [u8; 1] = [0];
            stream
                .read_exact(&mut bytes)
                .expect("Error reading packet data.");
            data.write_all(&mut bytes).unwrap();
        }
        // construct instance
        Self::new(data)
    }

    #[inline]
    pub fn from_stream_with_compression(stream: &mut impl Read) -> Self {
        // read packet length as varint
        let packet_length = types::Length::read(stream).get_value();
        // read packet
        let mut packet: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(packet_length as usize));
        for _ in 0..packet_length {
            let mut bytes: [u8; 1] = [0];
            stream
                .read_exact(&mut bytes)
                .expect("Error reading packet data.");
            packet.write_all(&mut bytes).unwrap();
        }
        // go back to the start of packet
        packet.seek(SeekFrom::Start(0)).unwrap();
        // read data length as varint
        let data_length = types::VarInt::read(&mut packet).get_value();
        // read the rest of packet data and decompress if needed

        // store the actual decompressed packet data
        let packet_data_and_id = if data_length != 0 {
            // read the rest of packet and decompress
            let mut packet_bytes: Vec<u8> = Vec::with_capacity(data_length as usize);
            packet
                .read_to_end(&mut packet_bytes)
                .expect("Error reading packet data.");
            let mut actual_packet_id_and_data = Vec::new();
            let mut decompress_stream = ZlibDecoder::new(Cursor::new(packet_bytes));
            decompress_stream
                .read_to_end(&mut actual_packet_id_and_data)
                .expect("Error reading packet data.");
            actual_packet_id_and_data
        } else {
            // read the rest of packet and NOT decompress
            let mut packet_bytes: Vec<u8> = Vec::new();
            packet
                .read_to_end(&mut packet_bytes)
                .expect("Error reading packet data.");
            packet_bytes
        };

        let mut packet_stream = Cursor::new(packet_data_and_id);
        // read the memory stream
        let mut packet_data = Vec::new();
        packet_stream.read_to_end(&mut packet_data).unwrap();

        Self::new(packet_data)
    }

    #[inline]
    pub fn recv(stream: &mut impl Read) -> Self {
        let compressed: bool = get_compression_threshold() > 0;
        let packet = if compressed {
            Self::from_stream_with_compression(stream)
        } else {
            Self::from_stream_without_compression(stream)
        };

        if get_logger().is_debug() {
            get_logger().debug(
                AnsiString::new_colorless("[")
                    + AnsiString::new_fore("🠯", (255, 0, 0))
                    + AnsiString::new_colorless("] ")
                    + AnsiString::new_colorless(&packet.to_string()),
            );
        }

        packet
    }
}

static mut COMPRESSION_THRESHOLD: i32 = -1;

pub fn get_compression_threshold() -> i32 {
    unsafe { COMPRESSION_THRESHOLD }
}

pub fn set_compression_threshold(thr: i32) {
    unsafe { COMPRESSION_THRESHOLD = thr }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

pub trait GenericPacket: PacketSerde
where
    Self: Sized,
    Self: Debug,
{
    fn recv(stream: &mut impl Read) -> Self {
        Self::from_bytes(RawPacket::recv(stream).raw_data)
    }

    fn send(&self, stream: &mut impl Write) {
        // send the packet
        RawPacket {
            raw_data: self.to_bytes(),
        }
        .send(stream);
    }
}
