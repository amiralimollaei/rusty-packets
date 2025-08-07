use std::io::Read;
use std::io::Seek;
use std::io::Write;

use crate::minecraft::connection::Client;
use crate::minecraft::packets::PacketRecv;
use crate::minecraft::packets::PacketSend;
use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketWriter,
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

#[derive(Clone, Debug)]
pub struct ClientInformationPacket {
    locale: String,              // String: max 16 characters
    view_distance: i8,           // Byte: for some reason this HAD TO BE SIGNED
    chat_mode: i32,              // VarInt Enum: 0: enabled, 1: commands only, 2: hidden
    chat_colors: bool,           // Boolean: can the chat be colored?
    skin_parts: u8,              // Unsigned Byte: parts of skin that are visible (7 bit bitflag)
    main_hand: i32,              // VarInt Enum: 0: left, 1: right
    text_filtering: bool, // Boolean: Enables filtering of text on signs and written book titles
    allow_server_listings: bool, // Boolean: Servers usually list online players, this option should let you not show up in that list
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
            locale: locale.to_string(),
            view_distance: view_distance,
            chat_mode: chat_mode as i32,
            chat_colors: chat_colors,
            skin_parts: skin_parts,
            main_hand: main_hand as i32,
            text_filtering: text_filtering,
            allow_server_listings: allow_server_listings,
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

impl<T: Read + Seek> PacketIn<T> for ClientInformationPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            locale: reader.read_string(),
            view_distance: reader.read_byte(),
            chat_mode: reader.read_varint(),
            chat_colors: reader.read_boolean(),
            skin_parts: reader.read_ubyte(),
            main_hand: reader.read_varint(),
            text_filtering: reader.read_boolean(),
            allow_server_listings: reader.read_boolean(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for ClientInformationPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_str(self.locale.as_str());
        writer.write_byte(self.view_distance);
        writer.write_varint(self.chat_mode);
        writer.write_boolean(self.chat_colors);
        writer.write_ubyte(self.skin_parts);
        writer.write_varint(self.main_hand);
        writer.write_boolean(self.text_filtering);
        writer.write_boolean(self.allow_server_listings);
    }
}

impl PacketRecv for ClientInformationPacket {}
impl PacketSend for ClientInformationPacket {}
