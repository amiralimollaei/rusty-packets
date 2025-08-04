use std::io::{Read, Seek};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketReader, PacketRecv,};

const UNWRAP_ERROR: &str = "LoginSuccessPacket: Unexpected error while reading value.";

#[derive(Clone, Debug)]
pub struct LoginProperty {
    name: String,
    value: String,
    signature: Option<String>,
}

#[derive(Debug)]
pub struct LoginSuccessPacket {
    uuid: u128,
    username: String,
    properties: Vec<LoginProperty>,
}

impl LoginSuccessPacket {
    #[inline]
    pub fn new(uuid: u128, username: String, properties: Vec<LoginProperty>) -> Self {
        Self {
            uuid: uuid,
            username: username,
            properties: properties,
        }
    }

    pub fn get_properties(&self) -> Vec<LoginProperty> {
        self.properties.clone()
    }

    pub fn get_uuid(&self) -> u128 {
        self.uuid
    }
}

impl Packet for LoginSuccessPacket {
    const ID: i32 = 2;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for LoginSuccessPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let uuid = reader.read_uuid();
        let username = reader.read_string();
        let properties_count = reader.read_varint() as usize;
        let mut properties: Vec<LoginProperty> = Vec::with_capacity(properties_count);
        for _ in 0..properties_count {
            properties.push(LoginProperty {
                name: reader.read_string(),
                value: reader.read_string(),
                signature: if reader.read_boolean() {
                    Some(reader.read_string())
                } else {
                    None
                },
            });
        }
        Self {
            uuid: uuid,
            username: username,
            properties: properties,
        }
    }
}

impl PacketRecv for LoginSuccessPacket {}
