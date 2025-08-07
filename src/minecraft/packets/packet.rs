// packet implementation based on https://minecraft.wiki/w/Java_Edition_protocol/Packets?oldid=2789623
use crate::minecraft::types::{self, MinecraftType};
use crate::utils::ansi::string::AnsiString;
use crate::utils::logging::{get_log_level, get_logger};
use crate::utils::{PacketReadable, PacketWritable};
use flate2::Compression;
use flate2::bufread::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{Cursor, SeekFrom, prelude::*};

use super::get_threshold;

#[derive(Clone, Debug)]
pub struct PacketContainer {
    packet_id: i32,
    packet_data: Vec<u8>,
}

impl PacketContainer {
    #[inline]
    pub fn new(packet_id: i32, packet_data: Vec<u8>) -> Self {
        Self {
            packet_id,
            packet_data,
        }
    }

    pub fn to_string(&self) -> String {
        if self.packet_data.len() > 100 {
            let data = self.packet_data.as_slice()[0..100].to_vec();
            let data: Vec<String> = data.iter().map(|x| format!("{:02x}", x)).collect();
            format!(
                "Packet(ID={:#02x}, DATA=[{} ...])",
                self.packet_id,
                data.join(" ")
            )
        } else {
            let data = self.packet_data.clone();
            let data: Vec<String> = data.iter().map(|x| format!("{:02x}", x)).collect();
            format!(
                "Packet(ID={:#02x}, DATA=[{}])",
                self.packet_id,
                data.join(" ")
            )
        }
    }

    pub fn as_stream(&self) -> Cursor<Vec<u8>> {
        let mut stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        stream.write_all(&mut self.packet_data.clone()).unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();
        stream
    }

    pub fn get_id(&self) -> i32 {
        self.packet_id
    }

    fn get_raw_packet(&self) -> Vec<u8> {
        // create a memory stream
        let mut packet_stream: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        // write packet id
        types::VarInt::from_i32(self.packet_id).write(&mut packet_stream);
        // write packet data
        packet_stream
            .write_all(&mut self.packet_data.clone())
            .expect("Error writing packet data.");
        // go back to the start of the memory stream
        packet_stream
            .seek(SeekFrom::Start(0))
            .expect("Error reading packet data.");
        // read the memory stream
        let mut packet_ = Vec::new();
        packet_stream
            .read_to_end(&mut packet_)
            .expect("Error reading packet data.");
        // return
        packet_
    }

    pub fn write_without_compression(&self, stream: &mut impl Write) {
        // encode raw packet
        let mut packet = self.get_raw_packet();
        // write packet length as varint
        types::Length::from_i32(packet.len() as i32).write(stream);
        // write packet
        stream
            .write_all(&mut packet)
            .expect("Error writing packet data.");
    }

    fn get_compressed_packet(&self) -> (Vec<u8>, bool) {
        let threshold: i32 = get_threshold();
        // encode raw packet (Packet ID + Packet Data)
        let packet = self.get_raw_packet();
        let data_length = packet.len();
        // compress packet if needed
        if data_length >= threshold as usize {
            let mut compressed_packet: Vec<u8> = Vec::new();
            // compress packet id + packet data
            let mut zlib_encoder = ZlibEncoder::new(Cursor::new(packet), Compression::default());
            zlib_encoder
                .read_to_end(&mut compressed_packet)
                .expect("error while compressing packet!");
            (compressed_packet, true)
        } else {
            (packet, false)
        }
    }

    // https://wiki.vg/Protocol#Packet_format
    pub fn write_with_compression(&self, stream: &mut impl Write) {
        let (mut packet_cmp, is_compressed) = self.get_compressed_packet();
        let mut packet_stream: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(packet_cmp.len()));
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
        let mut actual_packet: Vec<u8> = Vec::with_capacity(packet_stream.get_ref().len());
        packet_stream
            .seek(SeekFrom::Start(0))
            .expect("Error seeking packet data.");
        packet_stream
            .read_to_end(&mut actual_packet)
            .expect("Error reading packet data.");
        // write actual length as varint
        types::Length::from_i32(actual_packet.len() as i32).write(stream);
        // write data length + packet id + data
        stream
            .write_all(&mut actual_packet)
            .expect("Error writing packet data.");
    }

    pub fn send(&self, stream: &mut impl Write) {
        let threshold: i32 = get_threshold();
        if threshold > 0 {
            self.write_with_compression(stream)
        } else {
            self.write_without_compression(stream)
        }

        let mut ansi_string = AnsiString::empty();
        ansi_string = ansi_string + AnsiString::new_colorless("[");
        ansi_string = ansi_string + AnsiString::new_fore("🠩", Some((0, 255, 0)));
        ansi_string = ansi_string + AnsiString::new_colorless("] ");
        ansi_string = ansi_string + AnsiString::new_colorless(self.to_string().as_str());
        get_logger().debug(ansi_string);
    }

    #[inline]
    pub fn from_stream_without_compression(stream: &mut impl Read) -> Self {
        // read packet length as varint
        let packet_length = types::Length::read(stream).get_value();
        // read packet
        let mut packet: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        for _ in 0..packet_length {
            let mut bytes: [u8; 1] = [0];
            stream
                .read_exact(&mut bytes)
                .expect("Error reading packet data.");
            packet.write_all(&mut bytes).unwrap();
        }
        // go back to the start of packet
        packet.seek(SeekFrom::Start(0)).unwrap();
        // read packet id as varint
        let packet_id = types::VarInt::read(&mut packet);
        // read the memory stream
        let mut packet_data = Vec::new();
        packet.read_to_end(&mut packet_data).unwrap();

        Self::new(packet_id.get_value(), packet_data)
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
        // read packet id as varint
        let packet_id = types::VarInt::read(&mut packet_stream);
        // read the memory stream
        let mut packet_data = Vec::new();
        packet_stream.read_to_end(&mut packet_data).unwrap();

        Self::new(packet_id.get_value(), packet_data)
    }

    #[inline]
    pub fn recv(stream: &mut impl Read) -> Self {
        let compressed: bool = get_threshold() > 0;
        let packet = if compressed {
            Self::from_stream_with_compression(stream)
        } else {
            Self::from_stream_without_compression(stream)
        };

        let mut ansi_string = AnsiString::empty();
        ansi_string = ansi_string + AnsiString::new_colorless("[");
        ansi_string = ansi_string + AnsiString::new_fore("🠯", Some((255, 0, 0)));
        ansi_string = ansi_string + AnsiString::new_colorless("] ");
        ansi_string = ansi_string + AnsiString::new_colorless(packet.to_string().as_str());
        get_logger().debug(ansi_string);

        packet
    }

    pub fn ger_reader(&mut self) -> PacketReader<Cursor<Vec<u8>>> {
        PacketReader::from_stream(self.as_stream())
    }
}

#[derive(Clone, Debug)]
pub struct PacketWriter<T: Write + Seek> {
    stream: T,
}

impl<R: Write + Seek> PacketWriter<R> {
    #[inline]
    pub fn from_stream(stream: R) -> Self {
        Self { stream: stream }
    }

    pub fn finish(&mut self) {
        // reserved for forward compatibility
    }
    pub fn write<T: MinecraftType, U: Into<T>>(&mut self, value: U) {
        let value_raw: T = value.into();
        value_raw.write(&mut self.stream);
    }
    pub fn write_raw_consume<T: MinecraftType>(&mut self, value: T) {
        value.write(&mut self.stream);
    }
    pub fn write_raw<T: MinecraftType>(&mut self, value: &T) {
        value.write(&mut self.stream);
    }
    pub fn write_boolean(&mut self, value: bool) {
        types::Boolean::from(value).write(&mut self.stream);
    }
    pub fn write_byte(&mut self, value: i8) {
        types::Byte::from(value).write(&mut self.stream);
    }
    pub fn write_ubyte(&mut self, value: u8) {
        types::UnsignedByte::from(value).write(&mut self.stream);
    }
    pub fn write_short(&mut self, value: i16) {
        types::Short::from(value).write(&mut self.stream);
    }
    pub fn write_ushort(&mut self, value: u16) {
        types::UnsignedShort::from(value).write(&mut self.stream);
    }
    pub fn write_int(&mut self, value: i32) {
        types::Int::from(value).write(&mut self.stream);
    }
    pub fn write_long(&mut self, value: i64) {
        types::Long::from(value).write(&mut self.stream);
    }
    pub fn write_float(&mut self, value: f32) {
        types::Float::from(value).write(&mut self.stream);
    }
    pub fn write_double(&mut self, value: f64) {
        types::Double::from(value).write(&mut self.stream);
    }
    pub fn write_varint(&mut self, value: i32) {
        types::VarInt::from(value).write(&mut self.stream);
    }
    pub fn write_varlong(&mut self, value: i64) {
        types::VarLong::from(value).write(&mut self.stream);
    }
    pub fn write_length(&mut self, value: i32) {
        types::Length::from(value).write(&mut self.stream);
    }
    pub fn write_position(&mut self, x: i32, y: i16, z: i32) {
        types::Position::from((x, y, z)).write(&mut self.stream);
    }
    pub fn write_angle(&mut self, value: u8) {
        types::Angle::from(value).write(&mut self.stream);
    }
    pub fn write_uuid(&mut self, value: u128) {
        types::UUID::from(value).write(&mut self.stream);
    }
    pub fn write_string(&mut self, value: String) {
        types::String::from(value).write(&mut self.stream);
    }
    pub fn write_str(&mut self, value: &str) {
        types::String::from(value).write(&mut self.stream);
    }
    pub fn write_identifier_string(&mut self, value: String) {
        types::Identifier::from(value).write(&mut self.stream);
    }
    pub fn write_identifier_str(&mut self, value: &str) {
        types::Identifier::from(value).write(&mut self.stream);
    }
    pub fn write_nbt(&mut self, value: types::NBTValue) {
        value.write(&mut self.stream);
    }
    pub fn write_from_buffer(&mut self, buf: &[u8]) {
        self.stream.write(buf).unwrap();
    }
    pub fn write_ubyte_array(&mut self, array: Vec<u8>) {
        self.write_varint(array.len() as i32);
        self.stream.write(array.as_slice()).unwrap();
    }
}

#[derive(Clone, Debug)]
pub struct PacketReader<T: Read + Seek> {
    stream: T,
}

impl<S: Read + Seek> PacketReader<S> {
    #[inline]
    pub fn from_stream(stream: S) -> Self {
        Self { stream: stream }
    }

    pub fn finish(&self) {
        // Reserved for forward compatibility
    }

    pub fn read_raw<T: MinecraftType>(&mut self) -> T {
        T::read(&mut self.stream)
    }

    pub fn read_boolean(&mut self) -> bool {
        types::Boolean::read(&mut self.stream).into()
    }
    pub fn read_boolean_raw(&mut self) -> types::Boolean {
        types::Boolean::read(&mut self.stream)
    }
    pub fn read_byte(&mut self) -> i8 {
        types::Byte::read(&mut self.stream).into()
    }
    pub fn read_byte_raw(&mut self) -> types::Byte {
        types::Byte::read(&mut self.stream)
    }
    pub fn read_ubyte(&mut self) -> u8 {
        types::UnsignedByte::read(&mut self.stream).into()
    }
    pub fn read_ubyte_raw(&mut self) -> types::UnsignedByte {
        types::UnsignedByte::read(&mut self.stream)
    }
    pub fn read_short(&mut self) -> i16 {
        types::Short::read(&mut self.stream).into()
    }
    pub fn read_short_raw(&mut self) -> types::Short {
        types::Short::read(&mut self.stream)
    }
    pub fn read_ushort(&mut self) -> u16 {
        types::UnsignedShort::read(&mut self.stream).into()
    }
    pub fn read_ushort_raw(&mut self) -> types::UnsignedShort {
        types::UnsignedShort::read(&mut self.stream)
    }
    pub fn read_int(&mut self) -> i32 {
        types::Int::read(&mut self.stream).into()
    }
    pub fn read_int_raw(&mut self) -> types::Int {
        types::Int::read(&mut self.stream)
    }
    pub fn read_long(&mut self) -> i64 {
        types::Long::read(&mut self.stream).into()
    }
    pub fn read_long_raw(&mut self) -> types::Long {
        types::Long::read(&mut self.stream)
    }
    pub fn read_float(&mut self) -> f32 {
        types::Float::read(&mut self.stream).into()
    }
    pub fn read_float_raw(&mut self) -> types::Float {
        types::Float::read(&mut self.stream)
    }
    pub fn read_double(&mut self) -> f64 {
        types::Double::read(&mut self.stream).into()
    }
    pub fn read_double_raw(&mut self) -> types::Double {
        types::Double::read(&mut self.stream)
    }
    pub fn read_varint(&mut self) -> i32 {
        types::VarInt::read(&mut self.stream).into()
    }
    pub fn read_varint_raw(&mut self) -> types::VarInt {
        types::VarInt::read(&mut self.stream)
    }
    pub fn read_varlong(&mut self) -> i64 {
        types::VarLong::read(&mut self.stream).into()
    }
    pub fn read_varlong_raw(&mut self) -> types::VarLong {
        types::VarLong::read(&mut self.stream)
    }
    pub fn read_length(&mut self) -> i32 {
        types::Length::read(&mut self.stream).into()
    }
    pub fn read_length_raw(&mut self) -> types::Length {
        types::Length::read(&mut self.stream)
    }
    pub fn read_position_raw(&mut self) -> types::Position {
        types::Position::read(&mut self.stream)
    }
    pub fn read_angle(&mut self) -> u8 {
        types::Angle::read(&mut self.stream).into()
    }
    pub fn read_angle_raw(&mut self) -> types::Angle {
        types::Angle::read(&mut self.stream)
    }
    pub fn read_uuid(&mut self) -> u128 {
        types::UUID::read(&mut self.stream).into()
    }
    pub fn read_uuid_raw(&mut self) -> types::UUID {
        types::UUID::read(&mut self.stream)
    }
    pub fn read_string(&mut self) -> String {
        types::String::read(&mut self.stream).into()
    }
    pub fn read_string_raw(&mut self) -> types::String {
        types::String::read(&mut self.stream)
    }
    pub fn read_identifier(&mut self) -> String {
        types::Identifier::read(&mut self.stream).into()
    }
    pub fn read_identifier_raw(&mut self) -> types::Identifier {
        types::Identifier::read(&mut self.stream)
    }
    pub fn read_nbt(&mut self) -> types::NBTValue {
        types::NBTValue::read(&mut self.stream)
    }
    pub fn read_ubyte_array(&mut self) -> Vec<u8> {
        let lenght = self.read_varint() as usize;
        let mut_slice: &mut [u8] = &mut vec![0; lenght];
        self.stream.read_exact(mut_slice).unwrap();
        mut_slice.to_vec()
    }

    pub fn read_to_end(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.stream.read_to_end(&mut buf).expect("Unable to read from stream");
        buf
    }
}
