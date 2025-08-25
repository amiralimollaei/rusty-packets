use cesu8;
use flate2::bufread::GzDecoder;
use regex::Regex;

use std::f32::consts::PI;
use std::fmt::Debug;
use std::ops::Deref;

use crate::minecraft::packet::{PacketReadable, PacketWritable};
use crate::utils::{read_bytes, read_n_bytes};

use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Error, Read, Write},
    path::Path,
};

const WRITE_ERROR: &str = "Error while writing to connection";
const READ_ERROR: &str = "Error while reading connection";

pub trait MinecraftType: PacketReadable + PacketWritable {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Boolean {
    value: bool,
}

impl Debug for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<bool> for Boolean {
    fn from(item: bool) -> Self {
        Self { value: item }
    }
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.value
    }
}

impl Boolean {
    const N_BYTES: usize = 1;

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

impl MinecraftType for Boolean {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Byte {
    value: i8,
}

impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}


impl From<i8> for Byte {
    fn from(item: i8) -> Self {
        Self { value: item }
    }
}

impl Into<i8> for Byte {
    fn into(self) -> i8 {
        self.value
    }
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

impl MinecraftType for Byte {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnsignedByte {
    value: u8,
}

impl Debug for UnsignedByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<u8> for UnsignedByte {
    fn from(item: u8) -> Self {
        Self { value: item }
    }
}

impl Into<u8> for UnsignedByte {
    fn into(self) -> u8 {
        self.value
    }
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

impl MinecraftType for UnsignedByte {}

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

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Short {
    value: i16,
}

impl Debug for Short {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<i16> for Short {
    fn from(item: i16) -> Self {
        Self { value: item }
    }
}

impl Into<i16> for Short {
    fn into(self) -> i16 {
        self.value
    }
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

impl MinecraftType for Short {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnsignedShort {
    value: u16,
}

impl Debug for UnsignedShort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<u16> for UnsignedShort {
    fn from(item: u16) -> Self {
        Self { value: item }
    }
}

impl Into<u16> for UnsignedShort {
    fn into(self) -> u16 {
        self.value
    }
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

impl MinecraftType for UnsignedShort {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Int {
    value: i32,
}

impl Debug for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<i32> for Int {
    fn from(item: i32) -> Self {
        Self { value: item }
    }
}

impl Into<i32> for Int {
    fn into(self) -> i32 {
        self.value
    }
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

impl MinecraftType for Int {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Long {
    value: i64,
}

impl Debug for Long {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<i64> for Long {
    fn from(item: i64) -> Self {
        Self { value: item }
    }
}

impl Into<i64> for Long {
    fn into(self) -> i64 {
        self.value
    }
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

impl MinecraftType for Long {}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Float {
    value: f32,
}

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<f32> for Float {
    fn from(item: f32) -> Self {
        Self { value: item }
    }
}

impl Into<f32> for Float {
    fn into(self) -> f32 {
        self.value
    }
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

impl MinecraftType for Float {}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Double {
    value: f64,
}

impl Debug for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<f64> for Double {
    fn from(item: f64) -> Self {
        Self { value: item }
    }
}

impl Into<f64> for Double {
    fn into(self) -> f64 {
        self.value
    }
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

impl MinecraftType for Double {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VarInt {
    value: i32,
}

impl Debug for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<i32> for VarInt {
    fn from(item: i32) -> Self {
        Self { value: item }
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.value
    }
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

impl MinecraftType for VarInt {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VarLong {
    value: i64,
}

impl Debug for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl Into<i64> for VarLong {
    fn into(self) -> i64 {
        self.value
    }
}

impl From<i64> for VarLong {
    fn from(item: i64) -> Self {
        Self { value: item }
    }
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

impl MinecraftType for VarLong {}

// addresses the limitations of https://wiki.vg/Protocol#Packet_format
// it's VarInt but limited to 2^21 -1 max value and 3 bytes max size
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Length {
    value: i32,
}

impl Debug for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.value))
    }
}

impl From<i32> for Length {
    fn from(item: i32) -> Self {
        Self { value: item }
    }
}

impl Into<i32> for Length {
    fn into(self) -> i32 {
        self.value
    }
}

impl From<usize> for Length {
    fn from(item: usize) -> Self {
        Self { value: item as i32 }
    }
}

impl Into<usize> for Length {
    fn into(self) -> usize {
        self.value as usize
    }
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

impl MinecraftType for Length {}

// implements https://wiki.vg/Protocol#Position
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Position {
    x: i32,
    y: i16,
    z: i32,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{ x: {:?}, y: {:?}, z: {:?} }}", &self.x, &self.y, &self.z))
    }
}

impl From<(i32, i16, i32)> for Position {
    fn from(item: (i32, i16, i32)) -> Self {
        Self {
            x: item.0,
            y: item.1,
            z: item.2,
        }
    }
}

impl Into<(i32, i16, i32)> for Position {
    fn into(self) -> (i32, i16, i32) {
        (self.x, self.y, self.z)
    }
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

impl MinecraftType for Position {}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Angle {
    value: u8,
}

impl Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:.2}/360", &self.get_degrees()))
    }
}

impl From<u8> for Angle {
    fn from(item: u8) -> Self {
        Self { value: item }
    }
}

impl Into<u8> for Angle {
    fn into(self) -> u8 {
        self.value
    }
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

impl MinecraftType for Angle {}

#[derive(Debug)]
pub struct ParseUUIDError;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UUID {
    value: u128,
}

impl Debug for UUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", &self.to_hex()))
    }
}

impl From<u128> for UUID {
    fn from(item: u128) -> Self {
        Self { value: item }
    }
}

impl Into<u128> for UUID {
    fn into(self) -> u128 {
        self.value
    }
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
    /*pub fn to_string(&self) -> std::string::String {
        let format: [usize; 5] = [8, 4, 4, 4, 12];

        let hex = format!("{:x}", self.value);

        let mut chunks: Vec<&str> = Vec::new();
        let mut s: (&str, &str) = ("", hex.as_str());
        for v in format {
            s = s.1.split_at(v);
            chunks.push(s.0);
        }
        chunks.join("-")
    }*/

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

impl MinecraftType for UUID {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct String {
    codes: Vec<u16>,
}

impl Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", &self.to_string()))
    }
}

impl From<std::string::String> for String {
    fn from(item: std::string::String) -> Self {
        Self::from_string(item)
    }
}

impl Into<std::string::String> for String {
    fn into(self) -> std::string::String {
        self.to_string()
    }
}

impl From<&str> for String {
    fn from(item: &str) -> Self {
        Self::from_str(item)
    }
}

/*impl Into<&str> for String {
    fn into(self) -> &str {
        &self.to_string()
    }
}*/

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

impl MinecraftType for String {}

// it's just a string, but verified
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier {
    codes: Vec<u16>,
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", &self.to_string()))
    }
}

impl From<std::string::String> for Identifier {
    fn from(item: std::string::String) -> Self {
        Self::from_string(item)
    }
}

impl Into<std::string::String> for Identifier {
    fn into(self) -> std::string::String {
        self.to_string()
    }
}

impl From<&str> for Identifier {
    fn from(item: &str) -> Self {
        Self::from_str(item)
    }
}

impl Identifier {
    #[inline]
    pub fn new(codes: Vec<u16>) -> Self {
        let v = Self::validate(&std::string::String::from_utf16_lossy(&codes));
        Self {
            codes: v.encode_utf16().collect(),
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

impl PacketWritable for Identifier {
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

impl PacketReadable for Identifier {
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

impl MinecraftType for Identifier {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Optional<T: MinecraftType> {
    Some(T),
    None,
}

impl<T: Debug + MinecraftType> Debug for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(v) => {
                f.write_str(&format!("Some({:?})", v))
            }
            Self::None => {
                f.write_str("None")
            }
        }
    }
}

impl<T: MinecraftType, U: Into<T>> From<Option<U>> for Optional<T> {
    fn from(value: Option<U>) -> Self {
        match value {
            Some(v) => Self::Some(v.into()),
            None => Self::None,
        }
    }
}

impl<T: MinecraftType> Into<Option<T>> for Optional<T> {
    fn into(self) -> Option<T> {
        match self {
            Self::Some(v) => Some(v),
            Self::None => None,
        }
    }
}

impl<T: MinecraftType> Optional<T> {
    /// Converts this `Optional<T>` into a standard `Option<U>`,
    /// consuming the original value and converting the inner type.
    pub fn into_option<U>(self) -> Option<U>
    where
        T: Into<U>,
    {
        match self {
            Optional::Some(v) => Some(v.into()),
            Optional::None => None,
        }
    }
}

impl<T: MinecraftType> Optional<T> {
    pub fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => false,
            Self::None => true,
        }
    }
}

impl<T: MinecraftType> PacketReadable for Optional<T> {
    fn read(stream: &mut impl Read) -> Self {
        let is_some = Boolean::read(stream).get_value();
        if is_some {
            Self::Some(T::read(stream))
        } else {
            Self::None
        }
    }
}

impl<T: MinecraftType> PacketWritable for Optional<T> {
    fn write(&self, stream: &mut impl Write) {
        match self {
            Optional::Some(v) => {
                Boolean::new(true).write(stream);
                v.write(stream);
            }
            Optional::None => {
                Boolean::new(false).write(stream);
            }
        }
    }
}

impl<T: MinecraftType> MinecraftType for Optional<T> {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct Array<T: MinecraftType> {
    values: Vec<T>,
}

impl<T: Debug + MinecraftType> Debug for Array<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.values))
    }
}

impl<T: MinecraftType> Into<Vec<T>> for Array<T> {
    fn into(self) -> Vec<T> {
        self.values
    }
}

impl<T: MinecraftType> From<Vec<T>> for Array<T> {
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}

impl<T: MinecraftType> From<&[T]> for Array<T>
where
    T: Clone,
{
    fn from(slice: &[T]) -> Self {
        Self {
            values: slice.to_vec(),
        }
    }
}

impl<T: MinecraftType> Array<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self { values: values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T: MinecraftType> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T: MinecraftType> PacketReadable for Array<T> {
    fn read(stream: &mut impl Read) -> Self {
        let values_count = VarInt::read(stream).get_value() as usize;
        let mut values = Vec::with_capacity(values_count);
        for _ in 0..values_count {
            values.push(T::read(stream));
        }
        Self { values: values }
    }
}

impl<T: MinecraftType> PacketWritable for Array<T> {
    fn write(&self, stream: &mut impl Write) {
        VarInt::new(self.values.len() as i32).write(stream);
        for value in &self.values {
            value.write(stream);
        }
    }
}

impl<T: MinecraftType> MinecraftType for Array<T> {}

// a very common type of arrays
// it is implemented in a way that is more optimized and more convenient
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct ByteArray {
    values: Vec<u8>,
}

impl Debug for ByteArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.values))
    }
}

impl Into<Vec<u8>> for ByteArray {
    fn into(self) -> Vec<u8> {
        self.values
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(values: Vec<u8>) -> Self {
        Self { values }
    }
}

impl From<&[u8]> for ByteArray {
    fn from(slice: &[u8]) -> Self {
        Self {
            values: slice.to_vec(),
        }
    }
}

impl ByteArray {
    pub fn new(values: Vec<u8>) -> Self {
        Self { values: values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Deref for ByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl PacketReadable for ByteArray {
    fn read(stream: &mut impl Read) -> Self {
        let values_count = VarInt::read(stream).get_value() as usize;
        let mut values: Vec<u8> = Vec::with_capacity(values_count);
        stream.read_exact(&mut values).expect(READ_ERROR);
        Self { values: values }
    }
}

impl PacketWritable for ByteArray {
    fn write(&self, stream: &mut impl Write) {
        VarInt::new(self.values.len() as i32).write(stream);
        stream.write(&self.values).expect(WRITE_ERROR);
    }
}

impl MinecraftType for ByteArray {}

// a very common type of arrays that their size is inferred by the packet length
// it is implemented in a way that is more optimized and more convenient
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]

pub struct UnsizedByteArray {
    values: Vec<u8>,
}

impl Debug for UnsizedByteArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", &self.values))
    }
}

impl Into<Vec<u8>> for UnsizedByteArray {
    fn into(self) -> Vec<u8> {
        self.values
    }
}

impl From<Vec<u8>> for UnsizedByteArray {
    fn from(values: Vec<u8>) -> Self {
        Self { values }
    }
}

impl From<&[u8]> for UnsizedByteArray {
    fn from(slice: &[u8]) -> Self {
        Self {
            values: slice.to_vec(),
        }
    }
}

impl UnsizedByteArray {
    pub fn new(values: Vec<u8>) -> Self {
        Self { values: values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Deref for UnsizedByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl PacketReadable for UnsizedByteArray {
    fn read(stream: &mut impl Read) -> Self {
        let mut values: Vec<u8> = Vec::new();
        stream.read_to_end(&mut values).expect(READ_ERROR);
        Self { values: values }
    }
}

impl PacketWritable for UnsizedByteArray {
    fn write(&self, stream: &mut impl Write) {
        stream.write(&self.values).expect(WRITE_ERROR);
    }
}

impl MinecraftType for UnsizedByteArray {}

macro_rules! impl_minecraft_type_for_tuple {
    // The macro takes two lists:
    //  - $($T:ident),+ : A list of generic type names (e.g., T0, T1, T2)
    //  - $($idx:tt),+  : A list of the corresponding tuple indices (e.g., 0, 1, 2)
    ( $($T:ident),+ ; $($idx:tt),+ ) => {
        // Implement the marker trait `MinecraftType` for the tuple
        impl<$($T),+> MinecraftType for ( $($T,)+ )
        where
            // This is only valid if every element in the tuple is also a MinecraftType
            $( $T: MinecraftType ),+
        {}

        // Implement `PacketWritable` for the tuple
        impl<$($T),+> PacketWritable for ( $($T,)+ )
        where
            // This is only valid if every element is writable
            $( $T: PacketWritable ),+
        {
            /// Writes each element of the tuple to the stream in order.
            fn write(&self, stream: &mut impl std::io::Write) {
                // The magic of macros: this line expands for each element.
                // For a (T0, T1) tuple, it becomes:
                //   self.0.write(stream);
                //   self.1.write(stream);
                $(
                    self.$idx.write(stream);
                )+
            }
        }

        // Implement `PacketReadable` for the tuple
        impl<$($T),+> PacketReadable for ( $($T,)+ )
        where
            // This is required for any type that implements the trait
            Self: Sized,
            // This is only valid if every element is readable
            $( $T: PacketReadable ),+
        {
            /// Reads each element of the tuple from the stream in order.
            fn read(stream: &mut impl std::io::Read) -> Self {
                // This creates the tuple by calling `read` for each type in order.
                // For a (T0, T1) tuple, it becomes:
                //   (T0::read(stream), T1::read(stream))
                (
                    $(
                        $T::read(stream),
                    )+
                )
            }
        }
    };
}

//                Type Names | Indices
//                -----------|--------
impl_minecraft_type_for_tuple!(T0;        0);
impl_minecraft_type_for_tuple!(T0, T1;    0, 1);
impl_minecraft_type_for_tuple!(T0, T1, T2; 0, 1, 2);
impl_minecraft_type_for_tuple!(T0, T1, T2, T3; 0, 1, 2, 3);
impl_minecraft_type_for_tuple!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
impl_minecraft_type_for_tuple!(T0, T1, T2, T3, T4, T5; 0, 1, 2, 3, 4, 5);

/// A custom trait for element-wise tuple conversion, used when `From`/`Into`
/// cannot be implemented due to Rust's orphan rule.
pub trait TupleInto<D> {
    /// Performs the conversion.
    fn into_tuple(self) -> D;
}

// Now, we create a macro to implement OUR trait. This is allowed!
macro_rules! impl_into_tuple {
    // - $($D:ident),+: A list of Destination generic type names (e.g., D0, D1)
    // - $($S:ident),+: A list of Source generic type names (e.g., S0, S1)
    // - $($idx:tt),+ : A list of the tuple indices (e.g., 0, 1)
    ( $($D:ident),+ ; $($S:ident),+ ; $($idx:tt),+ ) => {
        // Implement `TupleInto<DestinationTuple>` for `SourceTuple`
        // This is allowed because `TupleInto` is OUR trait (a "local" trait).
        impl<$($D,)+ $($S,)+> TupleInto<( $($D,)+ )> for ( $($S,)+ )
        where
            // The conversion is only possible if each source element can
            // be converted into the corresponding destination element.
            $( $S: Into<$D> ),+
        {
            fn into_tuple(self) -> ( $($D,)+ ) {
                (
                    $(
                        self.$idx.into(),
                    )+
                )
            }
        }
    };
}

impl_into_tuple!(D0; S0; 0);
impl_into_tuple!(D0, D1; S0, S1; 0, 1);
impl_into_tuple!(D0, D1, D2; S0, S1, S2; 0, 1, 2);
impl_into_tuple!(D0, D1, D2, D3; S0, S1, S2, S3; 0, 1, 2, 3);
impl_into_tuple!(D0, D1, D2, D3, D4; S0, S1, S2, S3, S4; 0, 1, 2, 3, 4);
impl_into_tuple!(D0, D1, D2, D3, D4, D5; S0, S1, S2, S3, S4, S5; 0, 1, 2, 3, 4, 5);

/// A custom trait for element-wise tuple conversion from a source tuple.
/// This is the counterpart to `TupleInto`.
pub trait TupleFrom<S> {
    /// Creates a value from a source tuple.
    fn from_tuple(source: S) -> Self;
}

macro_rules! impl_from_tuple {
    // - $($D:ident),+: A list of Destination generic type names (e.g., D0, D1)
    // - $($S:ident),+: A list of Source generic type names (e.g., S0, S1)
    // - $($idx:tt),+ : A list of the tuple indices (e.g., 0, 1)
    ( $($D:ident),+ ; $($S:ident),+ ; $($idx:tt),+ ) => {
        // Implement `TupleFrom<SourceTuple>` for `DestinationTuple`
        // This is allowed because `TupleFrom` is our local trait.
        impl<$($D,)+ $($S,)+> TupleFrom<( $($S,)+ )> for ( $($D,)+ )
        where
            // The conversion is only possible if each destination element
            // can be created from the corresponding source element.
            $( $D: From<$S> ),+
        {
            fn from_tuple(source: ( $($S,)+ )) -> Self {
                (
                    // Call .into() on each source element to convert
                    // it to the destination type. This works because
                    // `D: From<S>` implies `S: Into<D>`.
                    $(
                        source.$idx.into(),
                    )+
                )
            }
        }
    };
}

impl_from_tuple!(D0; S0; 0);
impl_from_tuple!(D0, D1; S0, S1; 0, 1);
impl_from_tuple!(D0, D1, D2; S0, S1, S2; 0, 1, 2);
impl_from_tuple!(D0, D1, D2, D3; S0, S1, S2, S3; 0, 1, 2, 3);
impl_from_tuple!(D0, D1, D2, D3, D4; S0, S1, S2, S3, S4; 0, 1, 2, 3, 4);
impl_from_tuple!(D0, D1, D2, D3, D4, D5; S0, S1, S2, S3, S4, S5; 0, 1, 2, 3, 4, 5);


// ------------ NBT Implentation Start ------------

#[derive(Clone, Debug, PartialEq, Eq)]
// represents all possible NBT types
pub enum NBTType {
    End,                // Signifies the end of a Compound.
    Byte,               // A single signed byte
    Short,              // A single signed, big endian 16-bit integer
    Int,                // A single signed, big endian 32-bit integer
    Long,               // A single signed, big endian 64-bit integer
    Float,              // A single, big endian IEEE-754 single-precision floating point number (NaN possible)
    Double,             // A single, big endian IEEE-754 double-precision floating point number (NaN possible)
    ByteArray,          // A length-prefixed array of signed bytes. The prefix is a signed integer (thus 4 bytes)
    String,             // A length-prefixed modified UTF-8 string. The prefix is an unsigned short (thus 2 bytes)
    List(Box<NBTType>), // A list of nameless tags with the same type. prefixed with the Type ID and length as a signed integer (a thus 5 bytes).
    Compound,           // A list of named tags with variable types, Order is not guaranteed.
    IntArray,           // A length-prefixed array of signed integers. The prefix is a signed integer (thus 4 bytes) and indicates the number of 4 byte integers.
    LongArray,          // A length-prefixed array of signed longs. The prefix is a signed integer (thus 4 bytes) and indicates the number of 8 byte longs.
}

impl NBTType {
    pub fn is_simple(&self) -> bool {
        match self {
            Self::End => true,
            Self::Byte => true,
            Self::Short => true,
            Self::Int => true,
            Self::Long => true,
            Self::Float => true,
            Self::Double => true,
            Self::ByteArray => true,
            Self::String => true,
            Self::List(_) => false,
            Self::Compound => false,
            Self::IntArray => true,
            Self::LongArray => true,
        }
    }

    pub fn get_id(&self) -> u8 {
        match self {
            Self::End => 0,
            Self::Byte => 1,
            Self::Short => 2,
            Self::Int => 3,
            Self::Long => 4,
            Self::Float => 5,
            Self::Double => 6,
            Self::ByteArray => 7,
            Self::String => 8,
            Self::List(_) => 9,
            Self::Compound => 10,
            Self::IntArray => 11,
            Self::LongArray => 12,
        }
    }

    pub fn from_value(value: &NBTValue) -> Self {
        match value {
            NBTValue::End => Self::End,
            NBTValue::Byte(_) => Self::Byte,
            NBTValue::Short(_) => Self::Short,
            NBTValue::Int(_) => Self::Int,
            NBTValue::Long(_) => Self::Long,
            NBTValue::Float(_) => Self::Float,
            NBTValue::Double(_) => Self::Double,
            NBTValue::ByteArray(_) => Self::ByteArray,
            NBTValue::String(_) => Self::String,
            NBTValue::Compound(_, _) => Self::Compound,
            NBTValue::IntArray(_) => Self::IntArray,
            NBTValue::LongArray(_) => Self::LongArray,
            NBTValue::List(values) => {
                let mut inner_types: Vec<NBTType> = Vec::with_capacity(values.len());
                for v in values {
                    let ity = Self::from_value(v);
                    match inner_types.last() {
                        None => {}
                        Some(ty) => {
                            if *ty != ity {
                                panic!("NBT: List items must have the same type")
                            }
                        }
                    }
                    inner_types.push(ity);
                }
                Self::List(Box::new(inner_types.last().unwrap().clone()))
            }
        }
    }

    pub fn get_inner_type(&self) -> Option<&NBTType> {
        match self {
            Self::List(ty) => Some(ty.as_ref()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum NBTValue {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(std::string::String),
    List(Vec<NBTValue>),
    Compound(std::string::String, HashMap<std::string::String, NBTValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NBTValue {
    fn is_simple(&self) -> bool {
        match self {
            NBTValue::End => true,
            NBTValue::Byte(_) => true,
            NBTValue::Short(_) => true,
            NBTValue::Int(_) => true,
            NBTValue::Long(_) => true,
            NBTValue::Float(_) => true,
            NBTValue::Double(_) => true,
            NBTValue::ByteArray(_) => true,
            NBTValue::String(_) => true,
            NBTValue::List(_) => false,
            NBTValue::Compound(_, _) => false,
            NBTValue::IntArray(_) => true,
            NBTValue::LongArray(_) => true,
        }
    }

    fn to_simple_bytes(&self) -> Vec<u8> {
        assert!(self.is_simple());
        match self {
            NBTValue::End => Vec::new(),
            NBTValue::Byte(v) => v.to_be_bytes().to_vec(),
            NBTValue::Short(v) => v.to_be_bytes().to_vec(),
            NBTValue::Int(v) => v.to_be_bytes().to_vec(),
            NBTValue::Long(v) => v.to_be_bytes().to_vec(),
            NBTValue::Float(v) => v.to_be_bytes().to_vec(),
            NBTValue::Double(v) => v.to_be_bytes().to_vec(),
            NBTValue::ByteArray(vs) => {
                let mut bytes: Vec<[u8; 1]> = Vec::with_capacity(vs.len());
                for v in vs {
                    bytes.push(v.to_be_bytes());
                }
                [&(vs.len() as i32).to_be_bytes(), bytes.concat().as_slice()]
                    .concat()
                    .to_vec()
            }
            NBTValue::String(v) => {
                let bytes = cesu8::to_java_cesu8(v.as_str()).into_owned();
                [&(v.len() as u16).to_be_bytes(), bytes.as_slice()]
                    .concat()
                    .to_vec()
            }
            NBTValue::IntArray(vs) => {
                let mut bytes: Vec<[u8; 4]> = Vec::with_capacity(vs.len());
                for v in vs {
                    bytes.push(v.to_be_bytes());
                }
                [&(vs.len() as i32).to_be_bytes(), bytes.concat().as_slice()]
                    .concat()
                    .to_vec()
            }
            NBTValue::LongArray(vs) => {
                let mut bytes: Vec<[u8; 8]> = Vec::with_capacity(vs.len());
                for v in vs {
                    bytes.push(v.to_be_bytes());
                }
                [&(vs.len() as i32).to_be_bytes(), bytes.concat().as_slice()]
                    .concat()
                    .to_vec()
            }
            _ => {
                panic!("NotImplemented")
            }
        }
    }

    #[inline]
    fn write_simple_value(
        value: &NBTValue,
        stream: &mut impl std::io::Write,
    ) -> Result<usize, Error> {
        assert!(value.is_simple());
        stream.write(value.to_simple_bytes().as_slice())
    }

    #[inline]
    fn write_complex_value(
        value: &NBTValue,
        stream: &mut impl std::io::Write,
        compound_has_name: bool,
    ) {
        assert!(!value.is_simple());
        let ty = NBTType::from_value(value);
        match value {
            Self::List(vs) => {
                let ity = ty.get_inner_type().expect("NBT: TypeError");
                // write the inner type ID
                stream
                    .write(&ity.get_id().to_be_bytes())
                    .expect("NBT: WriteError");
                // write length
                stream
                    .write(&(vs.len() as i32).to_be_bytes())
                    .expect("NBT: WriteError");
                // write values
                for v in vs {
                    v.write_value(stream, true);
                }
            }
            Self::Compound(key, vs) => {
                // only in packets, the root compound tag does not have a name in 1.20.2+
                if compound_has_name {
                    NBTValue::String(key.clone()).write_value(stream, true);
                }
                for (k, v) in vs {
                    v.write_type(stream).expect("NBT: TypeError");
                    NBTValue::String(k.clone()).write_value(stream, true);
                    v.write_value(stream, true);
                }
                NBTValue::End.write(stream);
            }
            _ => {
                panic!("NotImplemented")
            }
        }
    }

    fn write_type(&self, stream: &mut impl std::io::Write) -> Result<usize, Error> {
        stream.write(&NBTType::from_value(self).get_id().to_be_bytes())
    }

    fn write_value(&self, stream: &mut impl std::io::Write, root_compound_has_name: bool) {
        if self.is_simple() {
            // write the value
            Self::write_simple_value(self, stream).expect("NBT: WriteError");
        } else {
            // write the value
            Self::write_complex_value(self, stream, root_compound_has_name)
        }
    }

    #[inline]
    fn read_type_id(stream: &mut impl Read) -> u8 {
        let mut buf: [u8; 1] = [0];
        stream.read_exact(&mut buf).expect("NBT: ReadError");
        u8::from_be_bytes(buf)
    }

    #[inline]
    fn read_string(stream: &mut impl Read) -> std::string::String {
        let length = u16::from_be_bytes(read_bytes(stream));
        let bytes = read_n_bytes(length as usize, stream);
        let str = cesu8::from_cesu8(bytes.as_slice()).expect("NBT: Error decoding string value!");
        str.into_owned()
    }

    // recursive value read
    fn read_value(type_id: u8, stream: &mut impl Read, root_compound_has_name: bool) -> NBTValue {
        //println!("{} {}", type_id, root_compound_has_name);
        match type_id {
            0 => Self::End,
            1 => Self::Byte(i8::from_be_bytes(read_bytes(stream))),
            2 => Self::Short(i16::from_be_bytes(read_bytes(stream))),
            3 => Self::Int(i32::from_be_bytes(read_bytes(stream))),
            4 => Self::Long(i64::from_be_bytes(read_bytes(stream))),
            5 => Self::Float(f32::from_be_bytes(read_bytes(stream))),
            6 => Self::Double(f64::from_be_bytes(read_bytes(stream))),
            7 => {
                let length = i32::from_be_bytes(read_bytes(stream));
                let mut values: Vec<i8> = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    values.push(i8::from_be_bytes(read_bytes(stream)))
                }
                Self::ByteArray(values)
            }
            8 => Self::String(Self::read_string(stream)),
            9 => {
                let type_id = u8::from_be_bytes(read_bytes(stream));
                let length = i32::from_be_bytes(read_bytes(stream));
                let mut values: Vec<NBTValue> = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    values.push(Self::read_value(type_id, stream, false));
                }
                Self::List(values)
            }
            10 => {
                // only in packets, the root compound tag does not have a name in 1.20.2+
                let root_name = if root_compound_has_name {
                    Self::read_string(stream)
                } else {
                    std::string::String::new()
                };
                //println!("root_name={}", root_name);
                let mut values: HashMap<std::string::String, NBTValue> = HashMap::new();
                loop {
                    let type_id = u8::from_be_bytes(read_bytes(stream));
                    if type_id == NBTType::End.get_id() {
                        break;
                    }
                    // read compound name
                    let key = Self::read_string(stream);
                    //println!("key={}", key);
                    values.insert(key, Self::read_value(type_id, stream, false));
                }
                Self::Compound(root_name, values)
            }
            11 => {
                let length = i32::from_be_bytes(read_bytes(stream));
                let mut values: Vec<i32> = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    values.push(i32::from_be_bytes(read_bytes(stream)));
                }
                Self::IntArray(values)
            }
            12 => {
                let length = i32::from_be_bytes(read_bytes(stream));
                let mut values: Vec<i64> = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    values.push(i64::from_be_bytes(read_bytes(stream)));
                }
                Self::LongArray(values)
            }
            _ => panic!("NotImplemented"),
        }
    }

    pub fn from_stream(stream: &mut impl Read, read_root_name: bool) -> Self {
        let type_id = Self::read_type_id(stream);
        Self::read_value(type_id, stream, read_root_name)
    }

    pub fn write_to_stream(&self, stream: &mut impl std::io::Write, write_root_name: bool) {
        self.write_type(stream).expect("NBT: WriteError");
        self.write_value(stream, write_root_name);
    }

    pub fn from_nbt(filepath: &str) -> Result<Self, Error> {
        let path = Path::new(filepath);
        if !path.exists() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?} does not exist.", filepath),
            ));
        }
        if !path.is_file() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?} is a directory.", filepath),
            ));
        }
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(e);
            }
        };
        let mut data: Vec<u8> = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        let mut gzip_decoder = GzDecoder::new(Cursor::new(data));
        Ok(Self::from_stream(&mut gzip_decoder, true))
    }
}

impl PacketWritable for NBTValue {
    fn write(&self, stream: &mut impl std::io::Write) {
        // write the type ID
        self.write_type(stream).expect("NBT: WriteError");
        // write the value
        self.write_value(stream, false);
    }
}

impl PacketReadable for NBTValue {
    fn read(stream: &mut impl Read) -> Self {
        let type_id = Self::read_type_id(stream);
        Self::read_value(type_id, stream, false)
    }
}

impl MinecraftType for NBTValue {}

// ------------- NBT Implentation end -------------