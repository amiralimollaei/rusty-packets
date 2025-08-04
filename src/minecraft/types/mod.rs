mod nbt;
pub use nbt::NBTValue;

use regex::Regex;

use std::f32::consts::PI;
use std::io::{Read, Write};

use crate::utils::{read_bytes, PacketReadable, PacketWritable};

const WRITE_ERROR: &str = "Error while writing to connection";
const READ_ERROR: &str = "Error while reading connection";

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Boolean {
    value: bool,
}

impl Boolean {
    #[inline]
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_bool(value: bool) -> Self {
        Self::new(value)
    }

    pub fn get_value(&self) -> bool {
        self.value
    }
}

impl PacketWritable for Boolean {
    fn write(&self, stream: &mut impl Write) {
        let byte: [u8; 1] = [if self.value { 1 } else { 0 }];
        stream.write_all(&byte).expect(WRITE_ERROR);
    }
}

impl PacketReadable for Boolean {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        Self::new(read_bytes(stream) == [1])
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Byte {
    value: i8,
}

impl Byte {
    const N_BYTES: usize = 1;

    #[inline]
    pub fn new(value: i8) -> Self {
        Self { value }
    }

    // convert the value to u8 as big-endian
    #[inline]
    pub fn from_u8(value: u8) -> Self {
        Self::new(i8::from_be_bytes(value.to_be_bytes()))
    }

    #[inline]
    pub fn from_i8(value: i8) -> Self {
        Self::new(value)
    }

    pub fn get_value(&self) -> i8 {
        self.value
    }

    pub fn get_u8(&self) -> u8 {
        u8::from_be_bytes(self.value.to_be_bytes())
    }
}

impl PacketWritable for Byte {
    fn write(&self, stream: &mut impl Write) {
        stream.write(&self.value.to_be_bytes()).expect(WRITE_ERROR);
    }
}

impl PacketReadable for Byte {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_u8(u8::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnsignedByte {
    value: u8,
}

impl UnsignedByte {
    const N_BYTES: usize = 1;

    #[inline]
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    // convert the value to u8 as big-endian
    #[inline]
    pub fn from_i8(value: i8) -> Self {
        Self::new(u8::from_be_bytes(value.to_be_bytes()))
    }

    #[inline]
    pub fn from_u8(value: u8) -> Self {
        Self::new(value)
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn get_i8(&self) -> i8 {
        i8::from_be_bytes(self.value.to_be_bytes())
    }
}

impl PacketWritable for UnsignedByte {
    fn write(&self, stream: &mut impl Write) {
        stream.write(&self.value.to_be_bytes()).expect(WRITE_ERROR);
    }
}

impl PacketReadable for UnsignedByte {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_u8(u8::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Short {
    value: i16,
}

impl Short {
    const N_BYTES: usize = 2;
    #[inline]
    pub fn new(value: i16) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_i16(value: i16) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u16(value: u16) -> Self {
        Self::new(i16::from_be_bytes(value.to_be_bytes()))
    }

    pub fn get_value(&self) -> i16 {
        self.value
    }

    pub fn get_u16(&self) -> u16 {
        u16::from_be_bytes(self.value.to_be_bytes())
    }
}

impl PacketWritable for Short {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for Short {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_u16(u16::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnsignedShort {
    value: u16,
}

impl UnsignedShort {
    const N_BYTES: usize = 2;
    #[inline]
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_u16(value: u16) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_i16(value: i16) -> Self {
        Self::new(u16::from_be_bytes(value.to_be_bytes()))
    }

    pub fn get_value(&self) -> u16 {
        self.value
    }

    pub fn get_i16(&self) -> i16 {
        i16::from_be_bytes(self.value.to_be_bytes())
    }
}

impl PacketWritable for UnsignedShort {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for UnsignedShort {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_u16(u16::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Int {
    value: i32,
}

impl Int {
    const N_BYTES: usize = 4;

    #[inline]
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_i32(value: i32) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u32(value: u32) -> Self {
        Self::new(i32::from_be_bytes(value.to_be_bytes()))
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl PacketWritable for Int {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for Int {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_i32(i32::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Long {
    value: i64,
}

impl Long {
    const N_BYTES: usize = 8;

    #[inline]
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_i64(value: i64) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u64(value: u64) -> Self {
        Self::new(i64::from_be_bytes(value.to_be_bytes()))
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl PacketWritable for Long {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for Long {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_i64(i64::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Float {
    value: f32,
}

impl Float {
    const N_BYTES: usize = 4;

    #[inline]
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_f32(value: f32) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u32(value: u32) -> Self {
        Self::new(f32::from_bits(value))
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }
}

impl PacketWritable for Float {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for Float {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_f32(f32::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Double {
    value: f64,
}

impl Double {
    const N_BYTES: usize = 8;

    #[inline]
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_f64(value: f64) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u64(value: u64) -> Self {
        Self::new(f64::from_bits(value))
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl PacketWritable for Double {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for Double {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // convert the value
        Self::from_f64(f64::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VarInt {
    value: i32,
}

impl VarInt {
    const SEGMENT_BITS: u32 = 0x7F;
    const CONTINUE_BIT: u8 = 0x80;

    #[inline]
    pub fn new(value: i32) -> Self {
        Self { value: value }
    }

    #[inline]
    pub fn from_i32(value: i32) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u32(value: u32) -> Self {
        Self::new(i32::from_be_bytes(value.to_be_bytes()))
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl PacketWritable for VarInt {
    fn write(&self, stream: &mut impl Write) {
        let mut value: u32 = u32::from_be_bytes(self.value.to_be_bytes());
        loop {
            if (value & !Self::SEGMENT_BITS) == 0 {
                stream.write_all(&mut [value as u8]).expect(WRITE_ERROR);
                break;
            }

            stream
                .write_all(&mut [(value & Self::SEGMENT_BITS) as u8 | Self::CONTINUE_BIT])
                .expect(WRITE_ERROR);

            // Note: >>> means that the sign bit is shifted with the rest of the number rather than being left alone
            value = value.wrapping_shr(7);
        }
    }
}

impl PacketReadable for VarInt {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        let mut value = 0;
        let mut position = 0;
        let mut bytes_count = 0;

        loop {
            let mut bytes: [u8; 1] = [0];
            stream.read_exact(&mut bytes).expect(READ_ERROR);
            bytes_count += 1;

            let byte = bytes[0];

            value |= (byte as u32 & Self::SEGMENT_BITS) << position;

            if (byte & Self::CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                panic!("VarInt is too big");
            };

            if bytes_count > 5 {
                panic!("VarInt data didn't end!");
            }
        }

        return Self::from_u32(value);
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VarLong {
    value: i64,
}

impl VarLong {
    const SEGMENT_BITS: u64 = 0x7F;
    const CONTINUE_BIT: u8 = 0x80;

    #[inline]
    pub fn new(value: i64) -> Self {
        Self { value: value }
    }

    #[inline]
    pub fn from_i64(value: i64) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u64(value: u64) -> Self {
        Self::new(i64::from_be_bytes(u64::to_be_bytes(value)))
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

impl PacketWritable for VarLong {
    fn write(&self, stream: &mut impl Write) {
        let mut value: u64 = if self.value >= 0 {
            self.value.unsigned_abs()
        } else {
            self.value.unsigned_abs() + 0x7FFFFFFFFFFFFFFF
        };
        loop {
            if (value & !Self::SEGMENT_BITS) == 0 {
                stream.write_all(&mut [value as u8]).expect(WRITE_ERROR);
                break;
            }

            stream
                .write_all(&mut [(value & Self::SEGMENT_BITS) as u8 | Self::CONTINUE_BIT])
                .expect(WRITE_ERROR);

            value = value.wrapping_shr(7);
        }
    }
}

impl PacketReadable for VarLong {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        let mut value = 0;
        let mut position = 0;
        let mut bytes_count = 0;

        loop {
            let mut bytes: [u8; 1] = [0];
            stream.read_exact(&mut bytes).expect(READ_ERROR);
            bytes_count += 1;

            let byte = bytes[0];

            value |= (byte as u64 & Self::SEGMENT_BITS) << position;

            if (byte & Self::CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 64 {
                panic!("VarInt is too big");
            };

            if bytes_count > 10 {
                panic!("VarInt data didn't end!");
            }
        }

        return Self::from_u64(value);
    }
}

// addresses the limitations of https://wiki.vg/Protocol#Packet_format
// it's VarInt but limited to 2^21 -1 max value and 3 bytes max size
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Length {
    value: i32,
}

impl Length {
    const SEGMENT_BITS: u32 = 0x7F;
    const CONTINUE_BIT: u8 = 0x80;
    const MAX_LENGTH: usize = 0x1FFFFF;
    const MAX_BYTES: usize = 3;

    #[inline]
    fn assert_range(value: i32) {
        // limiting bytes to 3 effectively removes the ability to encode negative numbers.
        if value < 0 {
            panic!("Length value can not be negative!");
        }
        if value as usize > Self::MAX_LENGTH {
            panic!("Length value too large!");
        }
    }

    #[inline]
    pub fn new(value: i32) -> Self {
        Self::assert_range(value);
        Self { value }
    }

    #[inline]
    pub fn from_i32(value: i32) -> Self {
        Self::new(value)
    }

    #[inline]
    pub fn from_u32(value: u32) -> Self {
        Self::new(i32::from_be_bytes(u32::to_be_bytes(value)))
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl PacketWritable for Length {
    fn write(&self, stream: &mut impl Write) {
        Self::assert_range(self.value);
        let mut value: u32 = self.value.unsigned_abs(); // we can not encode negative numbers
        let mut bytes_count = 0;
        loop {
            if (value & !Self::SEGMENT_BITS) == 0 {
                stream.write_all(&mut [value as u8]).expect(WRITE_ERROR);
                break;
            }

            stream
                .write_all(&mut [(value & Self::SEGMENT_BITS) as u8 | Self::CONTINUE_BIT])
                .expect(WRITE_ERROR);
            bytes_count += 1;

            if bytes_count > (Self::MAX_BYTES - 1) {
                panic!("Length value is too large for encoding!")
            }

            value = value.wrapping_shr(7);
        }
    }
}

impl PacketReadable for Length {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        let mut value = 0;
        let mut position = 0;
        let mut bytes_count = 0;

        loop {
            let bytes: [u8; 1] = read_bytes(stream);
            bytes_count += 1;

            let byte = bytes[0];

            value |= (byte as u32 & Self::SEGMENT_BITS) << position;

            if (byte & Self::CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                panic!("VarInt is too big");
            };

            if bytes_count > 3 {
                panic!("VarInt data didn't end!");
            }
        }

        let value = i32::from_be_bytes(value.to_be_bytes());
        Self::assert_range(value);

        Self::new(value)
    }
}

// implements https://wiki.vg/Protocol#Position
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i16,
    z: i32,
}

impl Position {
    const N_BYTES: usize = 8;

    #[inline]
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn from_xyz(x: i32, y: i16, z: i32) -> Self {
        Self::new(x, y, z)
    }

    #[inline]
    pub fn from_u64(value: u64) -> Self {
        let x = value.wrapping_shr(38) & 0x3FFFFFF;
        let y = value & 0xFFF;
        let z = value.wrapping_shr(12) & 0x3FFFFFF;

        Self::new(
            if x > 0x1FFFFFF {
                -(x as i32 - 0x1FFFFFF)
            } else {
                x as i32
            }, // handle two's complement
            if y > 0x7FF {
                -(y as i16 - 0x7FF)
            } else {
                y as i16
            }, // handle two's complement
            if z > 0x1FFFFFF {
                -(z as i32 - 0x1FFFFFF)
            } else {
                z as i32
            }, // handle two's complement
        )
    }

    pub fn to_u64(&self) -> u64 {
        let x: u32 = if self.x >= 0 {
            self.x.unsigned_abs()
        } else {
            self.x.unsigned_abs() + 0x1FFFFFF
        };
        let y: u16 = if self.y >= 0 {
            self.y.unsigned_abs()
        } else {
            self.y.unsigned_abs() + 0x7FF
        };
        let z: u32 = if self.z >= 0 {
            self.z.unsigned_abs()
        } else {
            self.z.unsigned_abs() + 0x1FFFFFF
        };
        (((x as u64) & 0x3FFFFFF) << 38) | ((y as u64) & 0xFFF) | (((z as u64) & 0x3FFFFFF) << 12)
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i16 {
        self.y
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }
}

impl PacketWritable for Position {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.to_u64().to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for Position {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        Self::from_u64(u64::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Angle {
    value: u8,
}

impl Angle {
    const N_BYTES: usize = 1;

    #[inline]
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_i8(value: i8) -> Self {
        Self::new(u8::from_be_bytes(i8::to_be_bytes(value)))
    }

    #[inline]
    pub fn from_u8(value: u8) -> Self {
        Self::new(value)
    }

    pub fn get_degrees(&self) -> f32 {
        self.value as f32 * (360.0 / 256.0)
    }

    pub fn get_rad(&self) -> f32 {
        self.value as f32 * (PI / 128.0)
    }
}

impl PacketWritable for Angle {
    fn write(&self, stream: &mut impl Write) {
        stream.write_all(&mut [self.value]).expect(WRITE_ERROR)
    }
}

impl PacketReadable for Angle {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        Self::from_u8(u8::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Debug)]
pub struct ParseUUIDError;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UUID {
    value: u128,
}

impl UUID {
    const N_BYTES: usize = 16;

    #[inline]
    pub fn new(value: u128) -> Self {
        Self { value }
    }

    #[inline]
    pub fn from_u128(value: u128) -> Self {
        Self::new(value)
    }

    pub fn to_hex(&self) -> std::string::String {
        format!("{:x}", self.value)
    }

    // formats the UUID just like in minecraft
    pub fn to_string(&self) -> std::string::String {
        let format: [usize; 5] = [8, 4, 4, 4, 12];

        let hex = format!("{:x}", self.value);

        let mut chunks: Vec<&str> = Vec::new();
        let mut s: (&str, &str) = ("", hex.as_str());
        for v in format {
            s = s.1.split_at(v);
            chunks.push(s.0);
        }
        chunks.join("-")
    }

    // reads the UUID from a string
    #[inline]
    pub fn from_string(str: &str) -> Result<Self, ParseUUIDError> {
        let hex = str.replace("-", "");
        if hex.len() != 32 {
            return Err(ParseUUIDError);
        }

        match u128::from_str_radix(hex.as_str(), 16) {
            Ok(v) => Ok(Self::from_u128(v)),
            Err(_) => Err(ParseUUIDError),
        }
    }

    pub fn get_value(&self) -> u128 {
        self.value
    }
}

impl PacketWritable for UUID {
    fn write(&self, stream: &mut impl Write) {
        stream
            .write_all(&mut self.value.to_be_bytes())
            .expect(WRITE_ERROR);
    }
}

impl PacketReadable for UUID {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        Self::from_u128(u128::from_be_bytes(read_bytes(stream)))
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct String {
    codes: Vec<u16>,
}

impl String {
    #[inline]
    pub fn new(codes: Vec<u16>) -> Self {
        Self { codes }
    }

    #[inline]
    pub fn from_string(str: std::string::String) -> Self {
        Self::new(str.encode_utf16().collect())
    }

    #[inline]
    pub fn from_str(str: &str) -> Self {
        Self::new(str.encode_utf16().collect())
    }

    pub fn to_string(&self) -> std::string::String {
        std::string::String::from_utf16_lossy(&self.codes)
    }

    pub fn get_value(&self) -> std::string::String {
        self.to_string()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn len(&self) -> usize {
        self.codes.len()
    }
}

impl PacketWritable for String {
    fn write(&self, stream: &mut impl Write) {
        if self.codes.len() > 0x7FFF {
            println!("WARNING: writing a string that is too large!");
        }
        // get data as bytes
        let mut utf8_bytes = self.as_bytes();
        // write length as varint
        VarInt::from_i32(utf8_bytes.len() as i32).write(stream);
        // write the data
        stream.write_all(&mut utf8_bytes).expect(WRITE_ERROR);
    }
}

impl PacketReadable for String {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // read the first VarInt field that contains the size of the string.
        let size = VarInt::read(stream).get_value();

        if size < 0 {
            panic!(
                "invalid string length, unable to read data!\nsize = {}",
                size
            )
        }

        if size == 0 {
            return Self::from_str("");
        }

        // read data bytes
        let mut utf8_bytes: Vec<u8> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            let bytes: [u8; 1] = read_bytes(stream);
            utf8_bytes.push(bytes[0]);
        }

        let value =
            std::string::String::from_utf8(utf8_bytes).expect("error while decoding utf8 data!");

        Self::from_str(value.as_str())
    }
}

// it's a string, but verified
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Identifier {
    value: String,
}

impl Identifier {
    #[inline]
    pub fn new(value: std::string::String) -> Self {
        let v = Self::validate(&value);
        Self {
            value: String::from_string(v),
        }
    }

    #[inline]
    pub fn validate(value: &std::string::String) -> std::string::String {
        if value.len() > 0x7FFF {
            panic!("Identifier string is too large!");
        }

        // namespaces default to minecraft
        let v = if !value.contains(":") {
            "minecraft:".to_string() + value.to_lowercase().as_str()
        } else {
            value.to_lowercase()
        };

        let re = Regex::new(r"([a-z0-9.\-_])+:[a-z0-9.\-_/]+").unwrap();
        if !re.is_match(&v) {
            panic!("Identifier string is invalid: {}", v);
        }

        v
    }

    #[inline]
    pub fn from_str(str: &str) -> Self {
        Self::new(str.to_string())
    }

    #[inline]
    pub fn from_string(str: std::string::String) -> Self {
        Self::new(str)
    }

    pub fn to_string(&self) -> std::string::String {
        self.value.to_string()
    }

    pub fn get_value(&self) -> std::string::String {
        self.to_string()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.value.as_bytes()
    }
}

impl PacketWritable for Identifier {
    fn write(&self, stream: &mut impl Write) {
        self.value.write(stream)
    }
}

impl PacketReadable for Identifier {
    #[inline]
    fn read(stream: &mut impl Read) -> Self {
        // read the string form the stream
        let string = String::read(stream).to_string();
        // initialize a new Identifier object containing the string (validates the string automatically)
        Self::new(string)
    }
}