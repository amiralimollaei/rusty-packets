// packet implementation based on https://minecraft.wiki/w/Java_Edition_protocol/Packets?oldid=2789623
use crate::minecraft::types;
use crate::utils::ansi::string::AnsiString;
use crate::utils::logging::get_logger;
use crate::utils::read_n_bytes;

use flate2::Compression;
use flate2::read::ZlibDecoder;
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
        let mut packet_stream: Cursor<Vec<u8>> =
            Cursor::new(Vec::with_capacity(packet_cmp.len() + 4));
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
        Self {
            raw_data: read_n_bytes(stream, packet_length as usize)
                .expect("Error reading packet data."),
        }
    }

    #[inline]
    pub fn from_stream_with_compression(stream: &mut impl Read) -> Self {
        // read packet length as varint
        let packet_length = types::Length::read(stream).get_value();
        // read packet
        let mut raw_packet_stream = stream.take(packet_length as u64);
        // read data length as varint
        let data_length = types::VarInt::read(&mut raw_packet_stream).get_value();
        // read the rest of packet data and decompress if needed
        let is_compressed = data_length != 0;
        // store the actual decompressed packet data
        let mut raw_data = Vec::new();
        if is_compressed {
            ZlibDecoder::new(raw_packet_stream)
                .read_to_end(&mut raw_data)
                .expect("Error decompressing packet data.");
        } else {
            raw_packet_stream
                .read_to_end(&mut raw_data)
                .expect("Error reading packet data.");
        }

        Self::new(raw_data)
    }

    #[inline]
    pub fn recv(stream: &mut impl Read) -> Self {
        let compressed: bool = get_compression_threshold() > 0;
        let packet = if compressed {
            Self::from_stream_with_compression(stream)
        } else {
            Self::from_stream_without_compression(stream)
        };

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
        let raw_packet = RawPacket::recv(stream);
        if get_logger().is_debug() {
            get_logger().debug(
                AnsiString::new_colorless("[")
                    + AnsiString::new_fore("🠯", (255, 0, 0))
                    + AnsiString::new_colorless("] ")
                    + AnsiString::new_colorless(&Self::to_string(&raw_packet)),
            );
        }
        Self::from_bytes(raw_packet.raw_data)
    }

    fn send(&self, stream: &mut impl Write) {
        // send the packet
        let raw_packet = RawPacket {
            raw_data: self.to_bytes(),
        };
        if get_logger().is_debug() {
            get_logger().debug(
                AnsiString::new_colorless("[")
                    + AnsiString::new_fore("🠩", (0, 255, 0))
                    + AnsiString::new_colorless("] ")
                    + AnsiString::new_colorless(&Self::to_string(&raw_packet)),
            );
        }
        raw_packet.send(stream);
    }

    fn get_id(&self) -> i32;

    fn get_name(&self) -> std::string::String;

    fn get_name_by_id(id: i32) -> std::string::String;

    fn to_string(raw_packet: &RawPacket) -> std::string::String {
        let (id, data) = raw_packet.parse();
        if data.len() > 100 {
            let data = &data[0..100];
            let data: Vec<String> = data.iter().map(|x| format!("{:02x}", x)).collect();
            format!(
                "{}(ID={:#02x}, DATA=[{} ...])",
                Self::get_name_by_id(id),
                id,
                data.join(" ")
            )
        } else {
            let data = &data;
            let data: Vec<String> = data.iter().map(|x| format!("{:02x}", x)).collect();
            format!(
                "{}(ID={:#02x}, DATA=[{}])",
                Self::get_name_by_id(id),
                id,
                data.join(" ")
            )
        }
    }
}
