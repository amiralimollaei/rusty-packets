// mod events;

// implements a connection loop
use std::net::{TcpStream, ToSocketAddrs};

use std::time::Duration;

use super::client::configuration::ClientMainHand;
use super::{PROTOCOL_VERSION, client, server, types};
use super::packets::{ConnectionState, Packet, PacketContainer, PacketRecv, PacketSend, set_threshold};

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
            location: Location { x: 0.0, y: 0.0, z: 0.0, yaw: 180.0, pitch: 0.0 },
            velocity: Velocity { x: 0.0, y: 0.0, z: 0.0 }
        }
    }

    pub fn status_request(&mut self) -> server::status::StatusResponse {
        let mut stream = connect(&self.hostname, self.port);

        // send handshake start packet
        client::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            &self.hostname,
            self.port,
            client::handshake::HandshakeRequest::STATUS,
        )
        .send(&mut stream);

        self.state = ConnectionState::Status;

        // send status request packet to get the server's motd
        client::status::RequestPacket::new().send(&mut stream);

        // the next packet the server sends us must be a status reponse packet
        server::status::ResponsePacket::recv(&mut stream).deseralize()
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
        client::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            self.hostname.as_str(),
            self.port,
            client::handshake::HandshakeRequest::LOGIN,
        )
        .send(stream);

        self.state = ConnectionState::Login;
    }

    fn login(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Login);

        // send login start packet
        client::login::LoginStartPacket::new(self.username.to_string(), 0).send(stream);

        // login phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                server::login::DisconnectPacket::ID => {
                    let packet = server::login::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Login Failed!: {:?}", packet));
                    panic!()
                }
                server::login::EncryptionRequestPacket::ID => {
                    let packet = server::login::EncryptionRequestPacket::from_packet(raw_packet);
                    if packet.should_authenticate.into() {
                        get_logger().error(format!("Online mode is not suported!"));
                    }
                    get_logger().error(format!("Encryption is not suported: {:?}", packet));
                    panic!()
                }
                server::login::LoginSuccessPacket::ID => {
                    let packet = server::login::LoginSuccessPacket::from_packet(raw_packet);
                    get_logger().info(format!(
                        "Login Finished! properties:{:?}",
                        packet.properties
                    ));
                    // send login acknowledged packet to move to configuration phase
                    client::login::LoginAcknowledgedPacket.send(stream);
                    // set state to configuration
                    self.state = ConnectionState::Configuration;
                    break;
                }
                server::login::SetCompressionPacket::ID => {
                    let packet = server::login::SetCompressionPacket::from_packet(raw_packet);
                    // set global compression threshold
                    set_threshold(packet.threshold.into());
                    get_logger().info(format!(
                        "Set Compression: threshold={}",
                        packet.threshold.get_value()
                    ));
                }
                server::login::PluginRequestPacket::ID => {
                    // Unlike plugin messages in "play" mode, these messages follow a lock-step request/response scheme,
                    // where the client is expected to respond to a request indicating whether it understood. The
                    // notchian client always responds that it hasn't understood, and sends an empty payload.

                    let packet = server::login::PluginRequestPacket::from_packet(raw_packet);
                    get_logger().info(format!("LoginPluginRequestPacket: {:?}", packet));

                    client::login::LoginPluginResponsePacket::new(
                        packet.get_message_id(),
                        false,
                        Vec::new(),
                    )
                    .send(stream);
                }
                server::login::CookieRequest::ID => {
                    let packet = server::login::CookieRequest::from_packet(raw_packet);
                    get_logger().warn(format!("LoginCookieRequest: {:?}", packet));

                    client::login::CookieResponsePacket::new(packet.key.into(), None).send(stream);
                }
                id => {
                    get_logger().error(format!(
                        "Login stage packet with ID={:#04x} is not implemented.",
                        id
                    ));
                    panic!()
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
        client::configuration::ClientInformationPacket::new(
            self.get_locale(),
            self.get_view_distance(),
            client::configuration::ClientChatMode::Enabled,
            true,
            0x7F,
            self.get_main_hand(),
            false,
            self.allows_server_listings(),
        )
        .send(stream);

        // configuration phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                server::configuration::CookieRequestPacket::ID => {
                    let packet =
                        server::configuration::CookieRequestPacket::from_packet(raw_packet);
                    get_logger().warn(format!("CookieRequestPacket: {:?}", packet));
                    client::configuration::CookieResponsePacket {
                        key: packet.key,
                        payload: types::Optional::None,
                    }
                    .send(stream);
                }
                server::configuration::PluginMessagesPacket::ID => {
                    let packet =
                        server::configuration::PluginMessagesPacket::from_packet(raw_packet);
                    get_logger().warn(format!("Ignoring login plugin message: {:?}", packet))
                }
                server::configuration::DisconnectPacket::ID => {
                    let packet = server::configuration::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Configuration Failed! reason: {:?}", packet.reason));
                    panic!();
                }

                server::configuration::ConfigurationFinishPacket::ID => {
                    let packet =
                        server::configuration::ConfigurationFinishPacket::from_packet(raw_packet);
                    get_logger().info(format!("Configuration Finished!: {:?}", packet));
                    // send finish configuration acknowledged packet
                    client::configuration::ConfigurationAcknowledgedPacket.send(stream);
                    // set state to play
                    self.state = ConnectionState::Play;
                    break;
                }

                server::configuration::KeepAlivePacket::ID => {
                    let keepalive = server::configuration::KeepAlivePacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    client::configuration::KeepAlivePacket {
                        keepalive_id: keepalive.keepalive_id,
                    }
                    .send(stream);
                }

                server::configuration::PingPacket::ID => {
                    let packet = server::configuration::PingPacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    client::configuration::PongPacket {
                        timestamp: packet.timestamp,
                    }
                    .send(stream);
                }

                server::configuration::ResetChatPacket::ID => {
                    let packet = server::configuration::ResetChatPacket::from_packet(raw_packet);
                    get_logger().info(format!("ResetChatPacket: {:?}", packet));
                }

                server::configuration::RegistryDataPacket::ID => {
                    let packet = server::configuration::RegistryDataPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored registry data packet: {:?}",
                        packet.registry_id
                    ));
                }

                server::configuration::AddResourcePackPacket::ID => {
                    let packet =
                        server::configuration::AddResourcePackPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored add resource pack packet: {:?}",
                        packet
                    ));
                }

                server::configuration::RemoveResourcePackPacket::ID => {
                    let packet =
                        server::configuration::RemoveResourcePackPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored remove resource pack packet: {:?}",
                        packet
                    ));
                }

                0x0D => {
                    get_logger().warn(format!("WARNING: Ignored update tags packet"));
                }

                // TODO: implement packets
                server::configuration::KnownServerPacksPacket::ID => {
                    let packet =
                        server::configuration::KnownServerPacksPacket::from_packet(raw_packet);
                    get_logger().info(format!("Known Packs: {:?}", packet));
                    client::configuration::KnownClientPacksPacket {
                        packs: vec![client::configuration::KnownClientPack {
                            namespace: "minecraft".into(),
                            id: "core".into(),
                            version: "1.21.1".into(),
                        }]
                        .into(),
                    }
                    .send(stream);
                }

                server::configuration::FeatureFlagsPacket::ID => {
                    let packet = server::configuration::FeatureFlagsPacket::from_packet(raw_packet);
                    get_logger().info(format!("Feature Flags: {:?}", packet));
                }

                // TODO: don't completely die just because one packet is not supported
                id => {
                    get_logger().error(format!(
                        "Configuration stage packet with ID={:#04x} is not implemented.",
                        id
                    ));
                    panic!()
                }
            }
        }
    }

    pub fn execute_synchronize_player_position_packet(&mut self, packet: &server::play::SyncPlayerPositionPacket) {
        let flags_byte: i8 = packet.flags.into();
        self.location.x = if (flags_byte & 0x01) == 0 {
            packet.location.x.get_value()
        } else {
            self.location.x + packet.location.x.get_value()
        };
        self.location.y = if (flags_byte & 0x02) == 0 {
            packet.location.y.get_value()
        } else {
            self.location.y + packet.location.y.get_value()
        };
        self.location.z = if (flags_byte & 0x04) == 0 {
            packet.location.z.get_value()
        } else {
            self.location.z + packet.location.z.get_value()
        };
        self.location.yaw = if (flags_byte & 0x08) == 0 {
            packet.location.yaw.get_value()
        } else {
            self.location.yaw + packet.location.yaw.get_value()
        };
        self.location.pitch = if (flags_byte & 0x10) == 0 {
            packet.location.pitch.get_value()
        } else {
            self.location.pitch + packet.location.pitch.get_value()
        };
    }

    fn process_play_packet(&mut self, raw_packet: PacketContainer, stream: &mut TcpStream) {
        match raw_packet.get_id() {
            server::play::SyncPlayerPositionPacket::ID => {
                let packet = server::play::SyncPlayerPositionPacket::from_packet(raw_packet);
                self.execute_synchronize_player_position_packet(&packet);
                get_logger().info(format!("Teleported by server: {:?}", self.location));
                // send teleport confirmation packet
                server::play::ConfirmTeleportationPacket {teleport_id: packet.teleport_id.clone()}.send(stream);
            }

            server::play::ChangeDifficultyPacket::ID => {
                let packet = server::play::ChangeDifficultyPacket::from_packet(raw_packet);
                get_logger().info(format!("Difficulty Changed: {:?}", packet));
            }

            server::play::SetHeldItemPacket::ID => {
                let packet = server::play::SetHeldItemPacket::from_packet(raw_packet);
                get_logger().info(format!("Held Slot Changed: {:?}", packet));
            }

            server::play::SpawnEntityPacket::ID => {
                let packet = server::play::SpawnEntityPacket::from_packet(raw_packet);
                get_logger().info(format!("SpawnEntityPacket: {:?}", packet));
            }

            // TODO: don't completely die just because one packet is not supported
            id => {
                unsafe {
                    let ids = &raw mut NOT_IMPLEMENTED_PACKET_IDS as *mut Vec<i32>;
                    if !(*ids).contains(&id) {
                        get_logger().warn(format!(
                            "Play stage packet with ID={:#04x} is not implemented.",
                            id
                        ));
                        (*ids).push(id);
                    }
                }

                //panic!()
            }
        }
    }

    fn play(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Play);

        let mut bundle_packets: Vec<PacketContainer> = Vec::new();
        let mut do_bundle: bool = false;

        // playe phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);

            match raw_packet.get_id() {
                server::play::BundleDelimiterPacket::ID => {
                    // let packet: server::BundleDelimiterPacket = server::BundleDelimiterPacket::from_packet(raw_packet);
                    get_logger().debug(format!("BundleDelimiterPacket"));
                    if !do_bundle {
                        do_bundle = true
                    } else {
                        for pkt in bundle_packets.clone() {
                            self.process_play_packet(pkt, stream);
                        }
                    }
                }

                // excluded from bundle delimiter because the server closes the connection after this packet
                server::play::DisconnectPacket::ID => {
                    let packet = server::play::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Disconnected: {:?}", packet.reason));
                    self.state = ConnectionState::Handshaking;
                    break;
                }

                server::play::KeepAlivePacket::ID => {
                    let packet = server::play::KeepAlivePacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    client::play::KeepAlivePacket {
                        keepalive_id: packet.keepalive_id.clone(),
                    }
                    .send(stream);
                }

                server::play::LoginPacket::ID => {
                    let packet = server::play::LoginPacket::from_packet(raw_packet);
                    get_logger().info(format!("Successfully Logged In!: {:?}", packet));
                }

                server::play::PlayerAbilitiesPacket::ID => {
                    let packet = server::play::PlayerAbilitiesPacket::from_packet(raw_packet);
                    get_logger().info(format!("Player Abilities: {:?}", packet));
                }

                _ => {
                    if do_bundle {
                        bundle_packets.push(raw_packet);
                    } else {
                        self.process_play_packet(raw_packet, stream);
                    }
                }
            }

            // don't spam too much cpu cycles
            // thread::sleep(Duration::from_millis(20)); <--- this line was causing a very strange bug
        }
    }

    pub fn connect(&mut self) {
        let mut stream = connect(&self.hostname, self.port);
        self.handshake(&mut stream);
        self.login(&mut stream);
        self.configure(&mut stream);
        self.play(&mut stream);
    }
}
