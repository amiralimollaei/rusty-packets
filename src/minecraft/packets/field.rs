use std::{collections::HashMap, io::{Read, Seek}};

use crate::{minecraft::{packets::{field, PacketReader}, types::{self, Boolean, MinecraftType}}, utils::PacketReadable};


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FieldType<'a> {
    Boolean,
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    Long,
    Float,
    Double,
    VarInt,
    VarLong,
    Length,
    Position,
    Angle,
    UUID,
    String,
    Identifier,
    Optional(&'a FieldType<'a>),
    Tuple(Vec<&'a FieldType<'a>>),
    Array(&'a FieldType<'a>),
}

enum FieldValue<'a> {
    Boolean(types::Boolean),
    Byte(types::Byte),
    UnsignedByte(types::UnsignedByte),
    Short(types::Short),
    UnsignedShort(types::UnsignedShort),
    Int(types::Int),
    Long(types::Long),
    VarInt(types::VarInt),
    VarLong(types::VarLong),
    Float(types::Float),
    Double(types::Double),
    UUID(types::UUID),
    Length(types::Length),
    Position(types::Position),
    Angle(types::Angle),
    String(types::String),
    Identifier(types::Identifier),
    Optional(types::Optional),
    Tuple(Vec<&'a FieldValue<'a>>),
    Array(Vec<&'a FieldValue<'a>>),
}

pub fn read_fields<'a, T: Read + Seek>(field_type: &'a FieldType<'a>, reader: &'a mut PacketReader<T>) -> FieldValue<'a> {
    match field_type {
        FieldType::Boolean => FieldValue::Boolean(reader.read_boolean_raw()),
        FieldType::Byte => FieldValue::Byte(reader.read_byte_raw()),
        FieldType::UnsignedByte => FieldValue::UnsignedByte(reader.read_ubyte_raw()),
        FieldType::Short => FieldValue::Short(reader.read_short_raw()),
        FieldType::UnsignedShort => FieldValue::UnsignedShort(reader.read_ushort_raw()),
        FieldType::Int => FieldValue::Int(reader.read_int_raw()),
        FieldType::Long => FieldValue::Long(reader.read_long_raw()),
        FieldType::Float => FieldValue::Float(reader.read_float_raw()),
        FieldType::Double => FieldValue::Double(reader.read_double_raw()),
        FieldType::VarInt => FieldValue::VarInt(reader.read_varint_raw()),
        FieldType::VarLong => FieldValue::VarLong(reader.read_varlong_raw()),
        FieldType::Length => FieldValue::Length(reader.read_length_raw()),
        FieldType::Position => FieldValue::Position(reader.read_position_raw()),
        FieldType::Angle => FieldValue::Angle(reader.read_angle_raw()),
        FieldType::UUID => FieldValue::UUID(reader.read_uuid_raw()),
        FieldType::String => FieldValue::String(reader.read_string_raw()),
        FieldType::Identifier => FieldValue::Identifier(reader.read_identifier_raw()),
        FieldType::Optional(inner_field_type) => {
            if reader.read_boolean() {
                FieldValue::Optional(Some(&read_fields(inner_field_type, reader)))
            } else {
                FieldValue::Optional(None)
            }
        },
        FieldType::Tuple(inner_field_types) => {
            let mut field_values = Vec::with_capacity(inner_field_types.len());
            for inner_field_type in inner_field_types {
                field_values.push(&read_fields(inner_field_type, reader));
            }
            FieldValue::Tuple(field_values)
        },
        FieldType::Array(inner_field_type) => {
            let total_count = reader.read_varint() as usize;
            let mut field_values = Vec::with_capacity(total_count);
            for _ in 0..total_count {
                field_values.push(&read_fields(inner_field_type, reader));
            }
            FieldValue::Array(field_values)
        },
    }
}

struct FieldReader<'a> {
    fileds: HashMap<&'a str, FieldType<'a>>
}

impl<'a> FieldReader<'a> {
    fn read<T: Read + Seek>(&self, reader: &mut PacketReader<T>) {
        let mut fileds_map: HashMap<&'a str, FieldValue<'a>> = HashMap::with_capacity(self.fileds.len());
        for (field_name, field_type) in self.fileds {
            fileds_map.insert(field, v)
        }

    }
}