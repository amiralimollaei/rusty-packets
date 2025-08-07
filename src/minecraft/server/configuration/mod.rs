mod disconnect;
pub use disconnect::DisconnectPacket;

mod cookie_request;
pub use cookie_request::CookieRequestPacket;

mod keepalive;
pub use keepalive::KeepAlivePacket;

mod configuration_finish;
pub use configuration_finish::ConfigurationFinishPacket;

mod registry_data;
pub use registry_data::{RegistryDataPacket, RegistryEntry};

mod server_packs;
pub use server_packs::{KnownServerPacksPacket, KnownServerPack};

mod feature_flags;
pub use feature_flags::FeatureFlagsPacket;

mod plugin_message;
pub use plugin_message::PluginMessagesPacket;

mod pong;
pub use pong::PongPacket;

mod reset_chat;
pub use reset_chat::ResetChatPacket;

mod remove_resource_pack;
pub use remove_resource_pack::RemoveResourcePackPacket;

mod add_resource_pack;
pub use add_resource_pack::AddResourcePackPacket;

use crate::minecraft::types;

#[derive(Clone, Copy)]
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

impl From<types::Identifier> for RegistryMap {
    fn from(value: types::Identifier) -> Self {
        match value.to_string().as_str() {
            "minecraft:trim_material" => RegistryMap::ArmorTrimMaterial,
            "minecraft:trim_pattern" => RegistryMap::ArmorTrimPattern,
            "minecraft:banner_pattern" => RegistryMap::BannerPattern,
            "minecraft:worldgen/biome" => RegistryMap::Biome,
            "minecraft:chat_type" => RegistryMap::ChatType,
            "minecraft:damage_type" => RegistryMap::DamageType,
            "minecraft:dimension_type" => RegistryMap::DimensionType,
            "minecraft:wolf_variant" => RegistryMap::WolfVariant,
            "minecraft:painting_variant" => RegistryMap::PaintingVariant,
            _ => panic!("Could not convert to RegistryMap: {:?}", value)
        }
    }
}

impl Into<types::Identifier> for RegistryMap {
    fn into(self) -> types::Identifier {
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
        }.into()
    }
}

impl Into<String> for RegistryMap {
    fn into(self) -> String {
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
        }.to_string()
    }
}