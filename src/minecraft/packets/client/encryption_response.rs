use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

const UNWRAP_ERROR: &str = "EncryptionResponsePacket: Unexpected error while reading value.";

#[derive(Debug)]
pub struct EncryptionResponsePacket {
    shared_secret: Vec<u8>,        // Shared Secret value, encrypted with the server's public key.
    verify_token: Vec<u8>,         // Verify Token value, encrypted with the same public key as the shared secret.
}

impl EncryptionResponsePacket {
    #[inline]
    pub fn new(shared_secret: Vec<u8>, verify_token: Vec<u8>) -> Self {
        Self { shared_secret, verify_token, }
    }

    pub fn get_shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }

    pub fn get_verify_token(&self) -> Vec<u8> {
        self.verify_token.clone()
    }
}

impl Packet for EncryptionResponsePacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Login;
}

impl<T: Read + Seek> PacketIn<T> for EncryptionResponsePacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            shared_secret: reader.read_ubyte_array(),
            verify_token: reader.read_ubyte_array(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for EncryptionResponsePacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_ubyte_array(self.shared_secret.clone());
        writer.write_ubyte_array(self.verify_token.clone());
    }
}

impl PacketRecv for EncryptionResponsePacket {}
impl PacketSend for EncryptionResponsePacket {}
