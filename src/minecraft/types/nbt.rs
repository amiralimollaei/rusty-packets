use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Error, Read},
    path::Path,
};

use cesu8;
use flate2::bufread::GzDecoder;

use crate::{
    minecraft::packet::{PacketReadable, PacketWritable},
    minecraft::types::MinecraftType,
    utils::{read_bytes, read_n_bytes},
};

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
    String(String),
    List(Vec<NBTValue>),
    Compound(String, HashMap<String, NBTValue>),
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
    fn read_string(stream: &mut impl Read) -> String {
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
                    String::new()
                };
                //println!("root_name={}", root_name);
                let mut values: HashMap<String, NBTValue> = HashMap::new();
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
