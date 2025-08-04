use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};

use super::Location;

#[derive(Debug)]
pub struct SyncPlayerPositionPacket {
    location: Location, // contains the location of a player
    flags: i8,          // When the value of the this byte masked is zero the field is absolute, otherwise relative.
    teleport_id: i32    // VarInt: the client should respond with the same id  
}

impl SyncPlayerPositionPacket {
    #[inline]
    pub fn new(location: Location, flags: i8, teleport_id: i32) -> Self {
        Self { location: location, flags: flags, teleport_id: teleport_id }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn get_flags(&self) -> i8 {
        self.flags
    }

    pub fn get_teleport_id(&self) -> i32 {
        self.teleport_id
    }

    pub fn apply_changes(&self, location: Location) -> Location {
        let mut new_location = location.clone();
        new_location.set_x(if (self.flags & 0x01) == 0 {self.location.get_x()} else {new_location.get_x()+self.location.get_x()});
        new_location.set_y(if (self.flags & 0x02) == 0 {self.location.get_y()} else {new_location.get_y()+self.location.get_y()});
        new_location.set_z(if (self.flags & 0x04) == 0 {self.location.get_z()} else {new_location.get_z()+self.location.get_z()});
        new_location.set_yaw(if (self.flags & 0x08) == 0 {self.location.get_yaw()} else {new_location.get_yaw()+self.location.get_yaw()});
        new_location.set_pitch(if (self.flags & 0x10) == 0 {self.location.get_pitch()} else {new_location.get_pitch()+self.location.get_pitch()});
        new_location
    }
        
}

impl Packet for SyncPlayerPositionPacket {
    const ID: i32 = 0x40;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for SyncPlayerPositionPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let x = reader.read_double();
        let y = reader.read_double();
        let z = reader.read_double();
        let yaw = reader.read_float();
        let pitch = reader.read_float();
        let flags = reader.read_byte();
        let teleport_id = reader.read_varint();
        Self {
            location: Location::new(x, y, z, yaw, pitch),
            flags: flags,
            teleport_id: teleport_id
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for SyncPlayerPositionPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        // send xyz position
        let (x, y, z) = self.location.get_xyz();
        writer.write_double(x);
        writer.write_double(y);
        writer.write_double(z);
        // send yaw and pitch
        let (yaw, pitch) = self.location.get_direction();
        writer.write_float(yaw);
        writer.write_float(pitch);
        // send mask (not all sent values should be considered)
        writer.write_byte(self.flags);
        // send teleport id for confirmation
        writer.write_varint(self.teleport_id);
    }
}

impl PacketRecv for SyncPlayerPositionPacket {}
impl PacketSend for SyncPlayerPositionPacket {}
