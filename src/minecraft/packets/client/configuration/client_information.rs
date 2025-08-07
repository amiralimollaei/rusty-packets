use crate::minecraft::types::MinecraftType;
use minecraft_type_derive::MinecraftType;

use crate::utils::{PacketReadable, PacketWritable};

use crate::minecraft::{
    packets::{ConnectionState, Packet},
    types,
};

use crate::minecraft::connection::Client;

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
    pub fn new(
        locale: &str,
        view_distance: i8,
        chat_mode: ClientChatMode,
        chat_colors: bool,
        skin_parts: u8,
        main_hand: ClientMainHand,
        text_filtering: bool,
        allow_server_listings: bool,
    ) -> Self {
        Self {
            locale: locale.into(),
            view_distance: view_distance.into(),
            chat_mode: (chat_mode as i32).into(),
            chat_colors: chat_colors.into(),
            skin_parts: skin_parts.into(),
            main_hand: (main_hand as i32).into(),
            text_filtering: text_filtering.into(),
            allow_server_listings: allow_server_listings.into(),
        }
    }

    #[inline]
    pub fn default() -> Self {
        Self::new(
            "en_GB",
            8,
            ClientChatMode::Enabled,
            true,
            0x7F,
            ClientMainHand::Right,
            false,
            true,
        )
    }

    pub fn from_client(client: Client) -> Self {
        Self::new(
            client.get_locale(),
            client.get_view_distance(),
            ClientChatMode::Enabled,
            true,
            0x7F,
            client.get_main_hand(),
            false,
            client.allows_server_listings(),
        )
    }
}

impl Packet for ClientInformationPacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Configuration;
}
