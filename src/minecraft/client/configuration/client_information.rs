use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketReadable, PacketWritable},
    types,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientChatMode {
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClientMainHand {
    Left = 0,
    Right = 1,
}

#[derive(MinecraftType, Clone, Debug)]
pub struct ClientInformationPacket {
    pub locale: types::String,                 // String: max 16 characters
    pub view_distance: types::Byte,            // Byte: for some reason this HAD TO BE SIGNED
    pub chat_mode: types::VarInt, // VarInt Enum: 0: enabled, 1: commands only, 2: hidden
    pub chat_colors: types::Boolean, // Boolean: can the chat be colored?
    pub skin_parts: types::UnsignedByte, // Unsigned Byte: parts of skin that are visible (7 bit bitflag)
    pub main_hand: types::VarInt,        // VarInt Enum: 0: left, 1: right
    pub text_filtering: types::Boolean, // Boolean: Enables filtering of text on signs and written book titles
    pub allow_server_listings: types::Boolean, // Boolean: Servers usually list online players, this option should let you not show up in that list
}

impl ClientInformationPacket {
    #[inline]
    pub fn default() -> Self {
        Self {
            locale: "en_GB".into(),
            view_distance: 8.into(),
            chat_mode: (ClientChatMode::Enabled as i32).into(),
            chat_colors: true.into(),
            skin_parts: 0x7F.into(),
            main_hand: (ClientMainHand::Right as i32).into(),
            text_filtering: false.into(),
            allow_server_listings: true.into(),
        }
    }
}

impl Packet for ClientInformationPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}
