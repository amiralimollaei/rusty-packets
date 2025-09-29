use packet_serde_derive::PacketSerde;

use crate::minecraft::{
    packet::{ConnectionState, Packet, PacketSerde, PacketReadable, PacketWritable},
    types,
};

use base64::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::parce_text_component;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Version {
    name: Option<String>,
    protocol: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Player {
    name: String,
    id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlayersStatus {
    max: i32,
    online: i32,
    sample: Option<Vec<Player>>,
}

impl PlayersStatus {
    pub fn get_players_count(&self) -> String {
        format!("{}/{}", self.online, self.max)
    }
    pub fn get_players_list(&self) -> Vec<String> {
        let mut players_list: Vec<String> = Vec::new();
        match &self.sample {
            None => {}
            Some(players) => {
                players_list = Vec::with_capacity(players.len());
                for p in players {
                    players_list.push(p.name.clone());
                }
            }
        };
        players_list
    }
}

#[derive(Debug)]
pub struct Base64DecodeError;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Base64Image(String);
impl Base64Image {
    pub fn get_raw_image(&self) -> Result<Vec<u8>, Base64DecodeError> {
        let re = Regex::new(r"^data:image\/(\w+);base64,([a-zA-Z0-9\+\/=]*)").unwrap();
        match re.captures(&self.0) {
            None => Err(Base64DecodeError),
            Some(cap) => {
                let [_, base64data] = cap.extract().1;
                match BASE64_STANDARD.decode(base64data) {
                    Ok(bytes) => Ok(bytes),
                    Err(e) => {
                        println!("{}", e.to_string());
                        Err(Base64DecodeError)
                    }
                }
            }
        }
    }
}

// stores the information from https://wiki.vg/Server_List_Ping#Status_Response
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct StatusResponse {
    version: Version,
    players: PlayersStatus,
    description: Option<Value>,
    favicon: Option<Base64Image>,
    enforcesSecureChat: Option<bool>,
    previewsChat: Option<bool>,
}

impl StatusResponse {
    #[inline]
    pub fn from_json(s: &str) -> Self {
        let object = serde_json::from_str(s);
        match object {
            Ok(o) => o,
            Err(e) => {
                println!("Error: {}\nJson: {}", e.to_string(), s);
                panic!()
            }
        }
    }

    #[inline]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn get_description_text(&self) -> String {
        match self.description.clone() {
            None => String::new(),
            Some(value) => parce_text_component(&value, None, None)
                .to_string(&crate::utils::ansi::ColorMode::TrueColor),
        }
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }

    pub fn get_players(&self) -> &PlayersStatus {
        &self.players
    }

    pub fn get_favicon(&self) -> &Option<Base64Image> {
        &self.favicon
    }

    pub fn get_securechat(&self) -> &Option<bool> {
        &self.enforcesSecureChat
    }

    pub fn get_previewschat(&self) -> &Option<bool> {
        &self.previewsChat
    }
}

#[derive(PacketSerde, Debug, Clone)]
pub struct ResponsePacket {
    pub field_status: types::String,
}

impl ResponsePacket {
    pub fn deseralize(&self) -> StatusResponse {
        StatusResponse::from_json(self.field_status.to_string().as_str())
    }
}

impl Packet for ResponsePacket {
    const ID: i32 = 0x00;
    const PHASE: ConnectionState = ConnectionState::Status;
}


#[derive(PacketSerde, Debug, Clone)]
pub struct PongPacket {
    pub timestamp: types::Long,
}

impl Packet for PongPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Status;
}
