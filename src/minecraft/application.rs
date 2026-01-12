// mod events;

// implements a connection loop
use std::net::{TcpStream, ToSocketAddrs};

use std::time::Duration;

use crate::minecraft::clientbound::status::deseralize_status_response;
use crate::minecraft::packet::GenericPacket;
use crate::minecraft::serverbound::configuration::ServerboundKnownPack;
use crate::minecraft::types::Optional;

use super::clientbound::ClientboundConfigurationPacket;
use super::clientbound::ClientboundHandshakePacket;  // placeholder, not actually usable
use super::clientbound::ClientboundLoginPacket;
use super::clientbound::ClientboundPlayPacket;
use super::clientbound::ClientboundStatusPacket;

use super::serverbound::ServerboundConfigurationPacket;
use super::serverbound::ServerboundHandshakePacket;
use super::serverbound::ServerboundLoginPacket;
use super::serverbound::ServerboundPlayPacket;
use super::serverbound::ServerboundStatusPacket;

use super::packet::{ConnectionState, set_compression_threshold};
use super::serverbound::configuration::ClientMainHand;
use super::{PROTOCOL_VERSION, clientbound, serverbound};

use super::super::utils::logging::get_logger;

fn connect(hostname: &str, port: u16) -> TcpStream {
    let addrs = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .expect("Connection Error!");
    let mut stream: Option<TcpStream> = None;
    for addr in addrs {
        let connection = TcpStream::connect_timeout(&addr, Duration::from_secs(10));
        match connection {
            Ok(v) => stream = Some(v),
            Err(e) => {
                get_logger().error(e.to_string());
            }
        }
    }

    let stream: TcpStream = stream.expect("Connection Error!");
    stream.set_nodelay(true).expect("set_nodelay call failed");
    stream
        .set_read_timeout(Some(Duration::from_secs(15)))
        .expect("set_read_timeout call failed");
    stream
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct Client {
    hostname: String,
    port: u16,
    username: String,
    state: ConnectionState,

    locale: String,              // String: max 16 characters
    view_distance: i8,           // Byte: for some reason this HAS TO BE SIGNED
    main_hand: ClientMainHand,   // VarInt Enum: 0: left, 1: right
    allow_server_listings: bool, // Boolean: Servers usually list online players, this option should let you not show up in that list

    // location and movement states
    location: Location,
    velocity: Velocity,
}

static mut NOT_IMPLEMENTED_PACKET_IDS: Vec<i32> = Vec::new();

// handles connecting to a server and all packets in any state other than play
impl Client {
    const INITIAL_STATE: ConnectionState = ConnectionState::Handshaking;

    pub fn new(hostname: &str, port: u16, username: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            port: port,
            username: username.to_string(),
            state: Self::INITIAL_STATE,
            locale: "en_GB".to_string(),
            view_distance: 8,
            main_hand: ClientMainHand::Right,
            allow_server_listings: true,
            location: Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                yaw: 180.0,
                pitch: 0.0,
            },
            velocity: Velocity {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn status_request(hostname: &str, port: u16) -> clientbound::status::StatusResponse {
        let mut stream = connect(hostname, port);

        // send handshake start packet
        ServerboundHandshakePacket::HandshakeStart {
            protocol: PROTOCOL_VERSION.into(),
            hostname: hostname.into(),
            port: port.into(),
            next_state: serverbound::handshake::HandshakeRequest::STATUS,
        }
        .send(&mut stream);

        // send status request packet to get the server's motd
        ServerboundStatusPacket::StatusRequest.send(&mut stream);

        // the next packet the server sends us must be a status reponse packet
        if let ClientboundStatusPacket::StatusResponse { field_status } =
            ClientboundStatusPacket::recv(&mut stream)
        {
            deseralize_status_response(field_status.get_value())
        } else {
            panic!()
        }
    }

    pub fn get_state(&self) -> ConnectionState {
        self.state
    }

    pub fn get_locale(&self) -> &str {
        &self.locale
    }

    pub fn get_view_distance(&self) -> i8 {
        self.view_distance
    }

    pub fn get_main_hand(&self) -> ClientMainHand {
        self.main_hand
    }

    pub fn allows_server_listings(&self) -> bool {
        self.allow_server_listings
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    fn handshake(&mut self, stream: &mut TcpStream) {
        get_logger().info(format!(
            "Connecting to {}:{} as {}",
            self.hostname, self.port, self.username
        ));

        assert_eq!(self.state, ConnectionState::Handshaking);

        // send handshake start packet
        ServerboundHandshakePacket::HandshakeStart {
            protocol: PROTOCOL_VERSION.into(),
            hostname: self.hostname.as_str().into(),
            port: self.port.into(),
            next_state: serverbound::handshake::HandshakeRequest::LOGIN,
        }
        .send(stream);

        self.state = ConnectionState::Login;
    }

    fn login(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Login);

        // send login start packet
        ServerboundLoginPacket::LoginStart {
            username: self.username.clone().into(),
            uuid: 0.into(),
        }
        .send(stream);

        // login phase loop
        loop {
            // read one packet from the stream and process
            let packet = ClientboundLoginPacket::recv(stream);
            match &packet {
                ClientboundLoginPacket::Disconnect { reason } => {
                    get_logger().error(format!("Login Failed!: {:?}", packet));
                    panic!()
                }
                ClientboundLoginPacket::EncryptionRequest { server_id, public_key, verify_token, should_authenticate } => {
                    if should_authenticate.get_value() {
                        get_logger().error(format!("Online mode is not supported!"));
                    }
                    get_logger().error(format!("Encryption is not supported: {:?}", packet));
                    panic!()
                }
                ClientboundLoginPacket::LoginSuccess { uuid, username, properties} => {
                    get_logger().info(format!("Login Success: {:?}", packet));
                    // send login acknowledged packet and move on to configuration state
                    ServerboundLoginPacket::LoginAcknowledged.send(stream);
                    self.state = ConnectionState::Configuration;
                    break;
                }
                ClientboundLoginPacket::SetCompression { threshold } => {
                    // set global compression threshold
                    set_compression_threshold(threshold.get_value());
                    get_logger().info(format!("Set Compression: threshold={}", threshold.get_value()));
                }
                ClientboundLoginPacket::PluginRequest { message_id, channel: _, data: _ } => {
                    // Unlike plugin messages in "play" mode, these messages follow a lock-step request/response scheme,
                    // where the client is expected to respond to a request indicating whether it understood. The
                    // notchian client always responds that it hasn't understood, and sends an empty payload.

                    get_logger().info(format!("LoginPluginRequestPacket: {:?}", packet));

                    ServerboundLoginPacket::LoginPluginResponse {
                        message_id: message_id.clone(),
                        successful: false.into(),
                        data: Vec::new().into(),
                    }
                    .send(stream);
                }
                ClientboundLoginPacket::CookieRequest { key } => {
                    get_logger().warn(format!("LoginCookieRequest: {:?}", packet));

                    ServerboundLoginPacket::CookieResponse { 
                        key: key.clone(), 
                        payload: Optional::None
                    }
                    .send(stream);
                }
            }
        }
    }

    fn configure(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Configuration);

        // TODO: maybe act as a fabric client
        // client::configuration::PluginMessagePakcet {
        //     channel: "minecraft:brand",
        //     data: "fabric".as_bytes().to_vec()
        // }.send(stream);

        // send a default client information packet, otherwise we might not be able to join
        ServerboundConfigurationPacket::ClientInformation {
            locale: self.get_locale().into(),
            view_distance: self.get_view_distance().into(),
            chat_mode: serverbound::configuration::ClientChatMode::Enabled,
            chat_colors: true.into(),
            skin_parts: 0x7F.into(),
            main_hand: self.get_main_hand(),
            text_filtering: false.into(),
            allow_server_listings: self.allows_server_listings().into(),
        }
        .send(stream);

        // configuration phase loop
        loop {
            // read one packet from the stream and process
            let packet = ClientboundConfigurationPacket::recv(stream);
            match &packet {
                ClientboundConfigurationPacket::CookieRequest { key } => {
                    get_logger().warn(format!("CookieRequestPacket: {:?}", packet));
                    ServerboundConfigurationPacket::CookieResponse {
                        key: key.clone(),
                        payload: Optional::None,
                    }
                    .send(stream);
                }
                ClientboundConfigurationPacket::PluginMessage { channel, data } => {
                    get_logger().warn(format!("Ignored PluginMessagePacket: {:?}", packet))
                }
                ClientboundConfigurationPacket::Disconnect { reason } => {
                    get_logger().error(format!("Configuration Failed: {:?}", reason));
                    panic!();
                }
                ClientboundConfigurationPacket::ConfigurationFinish => {
                    get_logger().info(format!("Configuration Finished!: {:?}", packet));
                    // send finish configuration acknowledged packet
                    ServerboundConfigurationPacket::AcknowledgeFinishConfiguration.send(stream);
                    // set state to play
                    self.state = ConnectionState::Play;
                    break;
                }
                ClientboundConfigurationPacket::KeepAlive { keepalive_id } => {
                    // respond to keepalive packet
                    ServerboundConfigurationPacket::ServerboundKeepAlive{
                        keepalive_id: *keepalive_id,
                    }
                    .send(stream);
                }
                ClientboundConfigurationPacket::Ping { timestamp } => {
                    // respond to keepalive packet
                    ServerboundConfigurationPacket::Pong{
                        timestamp: *timestamp,
                    }
                    .send(stream);
                }
                ClientboundConfigurationPacket::ResetChat => {
                    get_logger().info(format!("ResetChatPacket: {:?}", packet));
                }
                ClientboundConfigurationPacket::RegistryData { registry_id, entries} => {
                    get_logger().warn(format!(
                        "Ignored registry data packet: {:?}",
                        registry_id
                    ));
                }
                ClientboundConfigurationPacket::RemoveResourcePack { uuid } => {
                    get_logger().warn(format!("Ignored remove resource pack packet: {:?}", packet));
                }
                ClientboundConfigurationPacket::AddResourcePack { uuid, url, hash, forced, prompt_message } => {
                    get_logger().warn(format!("Ignored add resource pack packet: {:?}", packet));
                }
                ClientboundConfigurationPacket::StoreCookie { key, payload } => {
                    get_logger().warn(format!("Ignored StoreCookiePacket: {:?}", packet));
                }
                ClientboundConfigurationPacket::Transfer { host, port} => {
                    todo!("transfer packet is not supported, but got: {:?}", packet)
                }
                ClientboundConfigurationPacket::FeatureFlags { flags } => {
                    get_logger().info(format!("Feature Flags: {:?}", flags));
                }
                ClientboundConfigurationPacket::UpdateTags { tags } => {
                    get_logger().warn(format!("Ignored UpdateTagsPacket: {:?}", packet));
                }
                ClientboundConfigurationPacket::KnownServerPacks { packs } => {
                    get_logger().info(format!("Known Server Packs: {:?}", packs));

                    // respond with the default notchain response
                    ServerboundConfigurationPacket::KnownClientPacks {
                        packs: vec![ServerboundKnownPack {
                            namespace: "minecraft".into(),
                            id: "core".into(),
                            version: "1.21.1".into(),
                        }]
                        .into(),
                    }
                    .send(stream);
                }
                ClientboundConfigurationPacket::CustomReportDetails { details } => {
                    get_logger().info(format!("Custom Report Details: {:?}", details));
                }
                ClientboundConfigurationPacket::ServerLinks { links } => {
                    get_logger().info(format!("Server Links: {:?}", links));
                }
            }
        }
    }

    fn process_play_bundle_packets(&mut self, bundle_packets: Vec<ClientboundPlayPacket>) {
        for packet in bundle_packets {
            match &packet {
                ClientboundPlayPacket::ChangeDifficulty { difficulty, is_locked} => {
                    get_logger().info(format!("Difficulty Changed: {:?}", packet));
                }
                ClientboundPlayPacket::SetHeldItem { slot } => {
                    get_logger().info(format!("Held Slot Changed: {:?}", packet));
                }
                ClientboundPlayPacket::SpawnEntity { entity_id, entity_uuid, entity_type, position, pitch, yaw, head_yaw, data, velocity} => {
                    get_logger().info(format!("SpawnEntityPacket: {:?}", packet));
                }
                packet => {
                    let id = packet.get_id();
                    // avoid spamming the logger with the same message
                    unsafe {
                        let ids = &raw mut NOT_IMPLEMENTED_PACKET_IDS as *mut Vec<i32>;
                        if !(*ids).contains(&id) {
                            get_logger().warn(format!(
                                "Clientbound Play packet with ID={:#04x} is not implemented, skipping.",
                                id
                            ));
                            (*ids).push(id);
                        }
                    }
                }
            }
        }
    }

    fn play(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Play);

        let mut bundle_packets: Vec<ClientboundPlayPacket> = Vec::new();

        // play phase loop
        loop {
            // read one packet from the stream
            let packet = ClientboundPlayPacket::recv(stream);

            match &packet {
                // packets that are bundled when processing
                ClientboundPlayPacket::BundleDelimiter => {
                    get_logger().debug(format!("BundleDelimiterPacket"));
                    self.process_play_bundle_packets(bundle_packets);
                    bundle_packets = Vec::new();
                }

                // packets that are not bundled when processing
                ClientboundPlayPacket::SynchronizePlayerPosition { location, flags, teleport_id} => {
                    let flags_byte: i8 = flags.get_value();
                    self.location.x = if (flags_byte & 0x01) == 0 {
                        location.x.get_value()
                    } else {
                        self.location.x + location.x.get_value()
                    };
                    self.location.y = if (flags_byte & 0x02) == 0 {
                        location.y.get_value()
                    } else {
                        self.location.y + location.y.get_value()
                    };
                    self.location.z = if (flags_byte & 0x04) == 0 {
                        location.z.get_value()
                    } else {
                        self.location.z + location.z.get_value()
                    };
                    self.location.yaw = if (flags_byte & 0x08) == 0 {
                        location.yaw.get_value()
                    } else {
                        self.location.yaw + location.yaw.get_value()
                    };
                    self.location.pitch = if (flags_byte & 0x10) == 0 {
                        location.pitch.get_value()
                    } else {
                        self.location.pitch + location.pitch.get_value()
                    };

                    get_logger().info(format!("Teleported by server: {:?}", self.location));
                    // send teleport confirmation packet
                    ServerboundPlayPacket::ConfirmTeleportation {
                        teleport_id: teleport_id.clone(),
                    }
                    .send(stream);
                }

                // excluded from bundle delimiter because the server closes the connection after this packet
                ClientboundPlayPacket::Disconnect { reason } => {
                    get_logger().error(format!("Disconnected: {:?}", reason));
                    self.state = ConnectionState::Handshaking;
                    break;
                }

                ClientboundPlayPacket::KeepAlive { keepalive_id} => {
                    // respond to keepalive packet
                    ServerboundPlayPacket::KeepAlive {
                        keepalive_id: keepalive_id.clone(),
                    }
                    .send(stream);
                }

                ClientboundPlayPacket::Login { data } => {
                    get_logger().info(format!("Successfully Logged In!: {:?}", packet));
                }

                ClientboundPlayPacket::PlayerAbilities { flags, flying_speed, field_of_view_modifier} => {
                    get_logger().info(format!("Player Abilities: {:?}", packet));
                }

                _ => {
                    bundle_packets.push(packet);
                }
            }
        }
    }

    pub fn connect(&mut self) {
        let mut stream = connect(&self.hostname, self.port);
        // TODO: Dynamic handling of Phases (they're not always in the following order)
        self.handshake(&mut stream);
        self.login(&mut stream);
        self.configure(&mut stream);
        self.play(&mut stream);
    }
}
