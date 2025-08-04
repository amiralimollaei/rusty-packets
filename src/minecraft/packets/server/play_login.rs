use std::io::{Read, Seek, Write};

use crate::minecraft::types::Position;
use crate::minecraft::packets::{ConnectionState, Packet, PacketIn, PacketOut, PacketReader, PacketRecv, PacketSend, PacketWriter};


#[derive(Debug)]
pub struct PlayLoginPacket {
    entity_id: i32,                           // The player's Entity ID (EID).
    is_harcore: bool,
    dimensions: Vec<String>,                  // Identifiers for all dimensions on the server.
    max_players: i32,                         // Was once used by the client to draw the player list, but now is ignored.
    view_distance: i32,                       // Render distance (2-32).
    simulation_distance: i32,                 // The distance that the client will process specific things, such as entities.
    reduced_debug_info: bool,                 // If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
    enable_respawn_screen: bool,              // Set to false when the doImmediateRespawn gamerule is true.
    do_limited_crafting: bool,                // Whether players can only craft recipes they have already unlocked. Currently unused by the client.
    dimension_type: i32,                      // The ID of the type of dimension in the `minecraft:dimension_type` registry, defined by the Registry Data packet.
	dimension_name: String,                   // Name of the dimension being spawned into.
    hashed_seed: i64,                         // First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
    game_mode: u8,                            // 0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
	previous_game_mode: i8,                   // -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3: Spectator. The previous game mode. Vanilla client uses this for the debug (F3 + N & F3 + F4) game mode switch. (More information needed)
    is_debug: bool,                           // True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks.
    is_flat: bool,                            // True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63.
    death_dimension_name: Option<String>,     // Name of the dimension the player died in.
    death_location: Option<Position>,         // The location that the player died at.
    portal_cooldown: i32,                     // The number of ticks until the player can use the portal again.
    enforces_secure_chat: bool
}

impl PlayLoginPacket {
    #[inline]
    pub fn new(
        entity_id: i32,
        is_harcore: bool,
        dimensions: Vec<String>,
        max_players: i32,
        view_distance: i32,
        simulation_distance: i32,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        do_limited_crafting: bool,
        dimension_type: i32,
        dimension_name: String,
        hashed_seed: i64,
        game_mode: u8,
        previous_game_mode: i8,
        is_debug: bool,
        is_flat: bool,
        death_dimension_name: Option<String>,
        death_location: Option<Position>,
        portal_cooldown: i32,
        enforces_secure_chat: bool
    ) -> Self {
        Self {
            entity_id: entity_id,
            is_harcore: is_harcore,
            dimensions: dimensions,
            max_players: max_players,
            view_distance: view_distance,
            simulation_distance: simulation_distance,
            reduced_debug_info: reduced_debug_info,
            enable_respawn_screen: enable_respawn_screen,
            do_limited_crafting: do_limited_crafting,
            dimension_type: dimension_type,
            dimension_name: dimension_name,
            hashed_seed: hashed_seed,
            game_mode: game_mode,
            previous_game_mode: previous_game_mode,
            is_debug: is_debug,
            is_flat: is_flat,
            death_dimension_name: death_dimension_name,
            death_location: death_location,
            portal_cooldown: portal_cooldown,
            enforces_secure_chat: enforces_secure_chat
        }
    }
}

impl Packet for PlayLoginPacket {
    const ID: i32 = 0x2B;
    const PHASE: ConnectionState = ConnectionState::Play;
}

impl<T: Read + Seek> PacketIn<T> for PlayLoginPacket {
    fn read(reader: &mut PacketReader<T>) -> Self {
        let entity_id = reader.read_int();
        let is_harcore = reader.read_boolean();
        let dimensions_len = reader.read_varint();
        let mut dimensions = Vec::with_capacity(dimensions_len as usize);
        for _ in 0..dimensions_len {
            dimensions.push(reader.read_identifier());
        }
        let max_players = reader.read_varint();
        let view_distance = reader.read_varint();
        let simulation_distance = reader.read_varint();
        let reduced_debug_info = reader.read_boolean();
        let enable_respawn_screen = reader.read_boolean();
        let do_limited_crafting = reader.read_boolean();
        let dimension_type = reader.read_varint();
        let dimension_name = reader.read_string();
        let hashed_seed = reader.read_long();
        let game_mode = reader.read_ubyte();
        let previous_game_mode = reader.read_byte();
        let is_debug = reader.read_boolean();
        let is_flat = reader.read_boolean();
        let has_death_location = reader.read_boolean();
        let death_dimension_name = if has_death_location {
            Some(reader.read_identifier())
        } else {
            None
        };
        let death_location = if has_death_location {
            Some(reader.read_position())
        } else {
            None
        };
        let portal_cooldown = reader.read_varint();
        let enforces_secure_chat = reader.read_boolean();

        Self {
            entity_id,
            is_harcore,
            dimensions,
            max_players,
            view_distance,
            simulation_distance,
            reduced_debug_info,
            enable_respawn_screen,
            do_limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_game_mode,
            is_debug,
            is_flat,
            death_dimension_name,
            death_location,
            portal_cooldown,
            enforces_secure_chat,
        }
    }
}

impl<T: Write + Seek> PacketOut<T> for PlayLoginPacket {
    fn write(&self, writer: &mut PacketWriter<T>) {
        writer.write_int(self.entity_id);
        writer.write_boolean(self.is_harcore);
        writer.write_varint(self.dimensions.len() as i32);
        for dim in &self.dimensions {
            writer.write_identifier_str(dim);
        }
        writer.write_varint(self.max_players);
        writer.write_varint(self.view_distance);
        writer.write_varint(self.simulation_distance);
        writer.write_boolean(self.reduced_debug_info);
        writer.write_boolean(self.enable_respawn_screen);
        writer.write_boolean(self.do_limited_crafting);
        writer.write_varint(self.dimension_type);
        writer.write_identifier_str(&self.dimension_name);
        writer.write_long(self.hashed_seed);
        writer.write_ubyte(self.game_mode);
        writer.write_byte(self.previous_game_mode);
        writer.write_boolean(self.is_debug);
        writer.write_boolean(self.is_flat);
        writer.write_boolean(self.death_dimension_name.is_some());
        if let Some(ref name) = self.death_dimension_name {
            writer.write_str(name);
        }
        if let Some(ref pos) = self.death_location {
            writer.write_position(pos.get_x(), pos.get_y(), pos.get_z());
        }
        writer.write_varint(self.portal_cooldown);
        writer.write_boolean(self.enforces_secure_chat);
    }
}

impl PacketRecv for PlayLoginPacket {}
impl PacketSend for PlayLoginPacket {}
