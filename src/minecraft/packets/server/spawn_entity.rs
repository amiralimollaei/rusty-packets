use std::io::{Read, Seek, Write};

use crate::minecraft::packets::{
    ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter
};

#[derive(Debug)]
pub struct SpawnEntityPacket {
    entity_id: i32,    // VarInt: A unique integer ID mostly used in the protocol to identify the entity.
    entity_uuid: u128, // A unique identifier that is mostly used in persistence and places where the uniqueness matters more.
    entity_type: i32,  // VarInt: ID in the minecraft:entity_type registry.
    x: f64,            // Double: entity x position
    y: f64,            // Double: entity y position
    z: f64,            // Double: entity z position
    pitch: f32,        // To get the real pitch, you must divide this by (256.0F / 360.0F)
    yaw: f32,          // To get the real yaw, you must divide this by (256.0F / 360.0F)
    head_yaw: f32,     // Only used by living entities, where the head of the entity may differ from the general body rotation.
    data: i32,         // Meaning dependent on the value of the Type field, see Object Data for details.
    velocity_x: i16,   // Same units as Set Entity Velocity.
    velocity_y: i16,   // Same units as Set Entity Velocity.
    velocity_z: i16,   // Same units as Set Entity Velocity.
}

impl SpawnEntityPacket {
    #[inline]
    pub fn new(
        entity_id: i32,
        entity_uuid: u128,
        entity_type: i32,
        x: f64,
        y: f64,
        z: f64,
        pitch: f32,
        yaw: f32,
        head_yaw: f32,
        data: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    ) -> Self {
        Self {
            entity_id: entity_id,
            entity_uuid: entity_uuid,
            entity_type: entity_type,
            x: x,
            y: y,
            z: z,
            pitch: pitch,
            yaw: yaw,
            head_yaw: head_yaw,
            data: data,
            velocity_x: velocity_x,
            velocity_y: velocity_y,
            velocity_z: velocity_z,
        }
    }
}

impl Packet for SpawnEntityPacket {
    const ID: i32 = 0x01;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for SpawnEntityPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        Self {
            entity_id: reader.read_varint(),
            entity_uuid: reader.read_uuid(),
            entity_type: reader.read_varint(),
            x: reader.read_double(),
            y: reader.read_double(),
            z: reader.read_double(),
            pitch: reader.read_float(),
            yaw: reader.read_float(),
            head_yaw: reader.read_float(),
            data: reader.read_varint(),
            velocity_x: reader.read_short(),
            velocity_y: reader.read_short(),
            velocity_z: reader.read_short(),
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for SpawnEntityPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_varint(self.entity_id);
        writer.write_uuid(self.entity_uuid);
        writer.write_varint(self.entity_type);
        writer.write_double(self.x);
        writer.write_double(self.y);
        writer.write_double(self.z);
        writer.write_float(self.pitch);
        writer.write_float(self.yaw);
        writer.write_float(self.head_yaw);
        writer.write_varint(self.data);
        writer.write_short(self.velocity_x);
        writer.write_short(self.velocity_y);
        writer.write_short(self.velocity_z);
    }
}

impl PacketRecv for SpawnEntityPacket {}
impl PacketSend for SpawnEntityPacket {}
