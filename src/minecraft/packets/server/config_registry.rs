use std::io::{Read, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketReader, PacketRecv};
use crate::minecraft::types::NBTValue;

const UNWRAP_ERROR: &str = "RegistryDataPacket: Unexpected error while reading value.";

enum RegistryMap {
    ArmorTrimMaterial,
    ArmorTrimPattern,
    BannerPattern,
    Biome,
    ChatType,
    DamageType,
    DimensionType,
    WolfVariant,
    PaintingVariant,
}

impl RegistryMap {
    fn get_key(&self) -> String {
        match self {
            RegistryMap::ArmorTrimMaterial => "minecraft:trim_material",
            RegistryMap::ArmorTrimPattern => "minecraft:trim_pattern",
            RegistryMap::BannerPattern => "minecraft:banner_pattern",
            RegistryMap::Biome => "minecraft:worldgen/biome",
            RegistryMap::ChatType => "minecraft:chat_type",
            RegistryMap::DamageType => "minecraft:damage_type",
            RegistryMap::DimensionType => "minecraft:dimension_type",
            RegistryMap::WolfVariant => "minecraft:wolf_variant",
            RegistryMap::PaintingVariant => "minecraft:painting_variant",
        }
        .to_string()
    }
    /*
    fn deserialize<S: Read>(&self, stream: &mut S) -> RegistryData {

    }
    */
}

#[derive(Debug, Clone)]
pub struct RegistryEntry {
    entry_id: String,
    data: Option<NBTValue>,
}

impl RegistryEntry {
    pub fn get_id(&self) -> String {
        self.entry_id.clone()
    }

    pub fn get_data(&self) -> Option<NBTValue> {
        self.data.clone()
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

#[derive(Debug)]
pub struct RegistryDataPacket {
    registry_id: String,         // Identifier
    entries: Vec<RegistryEntry>, // an array of entries
}

impl RegistryDataPacket {
    #[inline]
    pub fn new(registry_id: &str, entries: Vec<RegistryEntry>) -> Self {
        Self {
            registry_id: registry_id.to_string(),
            entries: entries,
        }
    }

    pub fn get_registry_id(&self) -> String {
        self.registry_id.clone()
    }

    pub fn get_entries(&self) -> Vec<RegistryEntry> {
        self.entries.clone()
    }
}

impl Packet for RegistryDataPacket {
    const ID: i32 = 0x07;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}

impl<T: Read + Seek> PacketIn<T> for RegistryDataPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let registry_id = reader.read_identifier();
        let entry_count = reader.read_varint() as usize;
        let mut entries: Vec<RegistryEntry> = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            entries.push(RegistryEntry {
                entry_id: reader.read_identifier(),
                data: if reader.read_boolean() {
                    Some(reader.read_nbt())
                } else {
                    None
                },
            });
        }

        Self {
            registry_id: registry_id,
            entries: entries,
        }
    }
}
/*
impl PacketOut for RegistryDataPacket {
    fn send<T: Write>(&self, stream: &mut T) {
        let mut builder = PacketBuilder::new(Self::ID);
        builder.add_str(self.registry_id.as_str());
        let entries: Vec<FieldValue> = Vec::with_capacity(self.entries.len());
        for e in self.entries.clone() {
            entries.push(
                FieldValue::Bundle(vec![
                    FieldValue::new_string(e.get_id().as_str()),
                    FieldValue::new_boolean(e.has_data()),
                    FieldValue::Conditional(FieldValue::NBT(e.get_data()), Self::has_data)
                ])
            )
        }
        let entries_field = FieldValue::Array(entries);

        builder.finish().send(stream);
    }
}*/

impl PacketRecv for RegistryDataPacket {}