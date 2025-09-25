// mod events;

// implements a connection loop
use std::net::{TcpStream, ToSocketAddrs};

use std::time::Duration;

use super::serverbound::configuration::ClientMainHand;
use super::packet::{
    ConnectionState, Packet, PacketContainer, PacketRecv, PacketSend, set_threshold,
};
use super::{PROTOCOL_VERSION, serverbound, clientbound, types};

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

    pub fn status_request(&mut self) -> clientbound::status::StatusResponse {
        let mut stream = connect(&self.hostname, self.port);

        // send handshake start packet
        serverbound::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            &self.hostname,
            self.port,
            serverbound::handshake::HandshakeRequest::STATUS,
        )
        .send(&mut stream);

        self.state = ConnectionState::Status;

        // send status request packet to get the server's motd
        serverbound::status::RequestPacket {}.send(&mut stream);

        // the next packet the server sends us must be a status reponse packet
        clientbound::status::ResponsePacket::recv(&mut stream).deseralize()
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
        serverbound::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            self.hostname.as_str(),
            self.port,
            serverbound::handshake::HandshakeRequest::LOGIN,
        )
        .send(stream);

        self.state = ConnectionState::Login;
    }

    fn login(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Login);

        // send login start packet
        serverbound::login::LoginStartPacket {
            username: self.username.clone().into(),
            uuid: 0.into(),
        }
        .send(stream);

        // login phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                clientbound::login::DisconnectPacket::ID => {
                    let packet = clientbound::login::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Login Failed!: {:?}", packet));
                    panic!()
                }
                clientbound::login::EncryptionRequestPacket::ID => {
                    let packet = clientbound::login::EncryptionRequestPacket::from_packet(raw_packet);
                    if packet.should_authenticate.into() {
                        get_logger().error(format!("Online mode is not suported!"));
                    }
                    get_logger().error(format!("Encryption is not suported: {:?}", packet));
                    panic!()
                }
                clientbound::login::LoginSuccessPacket::ID => {
                    let packet = clientbound::login::LoginSuccessPacket::from_packet(raw_packet);
                    get_logger().info(format!(
                        "Login Finished! properties:{:?}",
                        packet.properties
                    ));
                    // send login acknowledged packet to move to configuration phase
                    serverbound::login::LoginAcknowledgedPacket.send(stream);
                    // set state to configuration
                    self.state = ConnectionState::Configuration;
                    break;
                }
                clientbound::login::SetCompressionPacket::ID => {
                    let packet = clientbound::login::SetCompressionPacket::from_packet(raw_packet);
                    // set global compression threshold
                    set_threshold(packet.threshold.into());
                    get_logger().info(format!(
                        "Set Compression: threshold={}",
                        packet.threshold.get_value()
                    ));
                }
                clientbound::login::PluginRequestPacket::ID => {
                    // Unlike plugin messages in "play" mode, these messages follow a lock-step request/response scheme,
                    // where the client is expected to respond to a request indicating whether it understood. The
                    // notchian client always responds that it hasn't understood, and sends an empty payload.

                    let packet = clientbound::login::PluginRequestPacket::from_packet(raw_packet);
                    get_logger().info(format!("LoginPluginRequestPacket: {:?}", packet));

                    serverbound::login::LoginPluginResponsePacket {
                        message_id: packet.message_id.clone(),
                        successful: false.into(),
                        data: Vec::new().into(),
                    }
                    .send(stream);
                }
                clientbound::login::CookieRequest::ID => {
                    let packet = clientbound::login::CookieRequest::from_packet(raw_packet);
                    get_logger().warn(format!("LoginCookieRequest: {:?}", packet));

                    serverbound::login::CookieResponsePacket::new(packet.key.into(), None).send(stream);
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
        serverbound::configuration::ClientInformationPacket {
            locale: self.get_locale().into(),
            view_distance: self.get_view_distance().into(),
            chat_mode: (serverbound::configuration::ClientChatMode::Enabled as i32).into(),
            chat_colors: true.into(),
            skin_parts: 0x7F.into(),
            main_hand: (self.get_main_hand() as i32).into(),
            text_filtering: false.into(),
            allow_server_listings: self.allows_server_listings().into(),
        }
        .send(stream);

        // configuration phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                clientbound::configuration::CookieRequestPacket::ID => {
                    let packet =
                        clientbound::configuration::CookieRequestPacket::from_packet(raw_packet);
                    get_logger().warn(format!("CookieRequestPacket: {:?}", packet));
                    serverbound::configuration::CookieResponsePacket {
                        key: packet.key,
                        payload: types::Optional::None,
                    }
                    .send(stream);
                }
                clientbound::configuration::ClientboundPluginMessagePacket::ID => {
                    let packet =
                        clientbound::configuration::ClientboundPluginMessagePacket::from_packet(raw_packet);
                    get_logger().warn(format!("Ignoring login plugin message: {:?}", packet))
                }
                clientbound::configuration::DisconnectPacket::ID => {
                    let packet = clientbound::configuration::DisconnectPacket::from_packet(raw_packet);
                    get_logger()
                        .error(format!("Configuration Failed! reason: {:?}", packet.reason));
                    panic!();
                }

                clientbound::configuration::ConfigurationFinishPacket::ID => {
                    let packet =
                        clientbound::configuration::ConfigurationFinishPacket::from_packet(raw_packet);
                    get_logger().info(format!("Configuration Finished!: {:?}", packet));
                    // send finish configuration acknowledged packet
                    serverbound::configuration::AcknowledgeFinishConfigurationPacket.send(stream);
                    // set state to play
                    self.state = ConnectionState::Play;
                    break;
                }

                clientbound::configuration::KeepAlivePacket::ID => {
                    let keepalive = clientbound::configuration::KeepAlivePacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    serverbound::configuration::ServerboundKeepAlivePacket {
                        keepalive_id: keepalive.keepalive_id,
                    }
                    .send(stream);
                }

                clientbound::configuration::PingPacket::ID => {
                    let packet = clientbound::configuration::PingPacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    serverbound::configuration::PongPacket {
                        timestamp: packet.timestamp,
                    }
                    .send(stream);
                }

                clientbound::configuration::ResetChatPacket::ID => {
                    let packet = clientbound::configuration::ResetChatPacket::from_packet(raw_packet);
                    get_logger().info(format!("ResetChatPacket: {:?}", packet));
                }

                clientbound::configuration::RegistryDataPacket::ID => {
                    let packet = clientbound::configuration::RegistryDataPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored registry data packet: {:?}",
                        packet.registry_id
                    ));
                }

                clientbound::configuration::AddResourcePackPacket::ID => {
                    let packet =
                        clientbound::configuration::AddResourcePackPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored add resource pack packet: {:?}",
                        packet
                    ));
                }

                clientbound::configuration::RemoveResourcePackPacket::ID => {
                    let packet =
                        clientbound::configuration::RemoveResourcePackPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored remove resource pack packet: {:?}",
                        packet
                    ));
                }

                0x0D => {
                    get_logger().warn(format!("WARNING: Ignored update tags packet"));
                }

                // TODO: implement packets
                clientbound::configuration::KnownServerPacksPacket::ID => {
                    let packet =
                        clientbound::configuration::KnownServerPacksPacket::from_packet(raw_packet);
                    get_logger().info(format!("Known Packs: {:?}", packet));
                    serverbound::configuration::KnownClientPacksPacket {
                        packs: vec![serverbound::configuration::ServerboundKnownPacksPacket {
                            namespace: "minecraft".into(),
                            id: "core".into(),
                            version: "1.21.1".into(),
                        }]
                        .into(),
                    }
                    .send(stream);
                }

                clientbound::configuration::FeatureFlagsPacket::ID => {
                    let packet = clientbound::configuration::FeatureFlagsPacket::from_packet(raw_packet);
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

    pub fn execute_synchronize_player_position_packet(
        &mut self,
        packet: &clientbound::play::SyncPlayerPositionPacket,
    ) {
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
            clientbound::play::SyncPlayerPositionPacket::ID => {
                let packet = clientbound::play::SyncPlayerPositionPacket::from_packet(raw_packet);
                self.execute_synchronize_player_position_packet(&packet);
                get_logger().info(format!("Teleported by server: {:?}", self.location));
                // send teleport confirmation packet
                serverbound::play::ConfirmTeleportationPacket {
                    teleport_id: packet.teleport_id.clone(),
                }
                .send(stream);
            }

            clientbound::play::ChangeDifficultyPacket::ID => {
                let packet = clientbound::play::ChangeDifficultyPacket::from_packet(raw_packet);
                get_logger().info(format!("Difficulty Changed: {:?}", packet));
            }

            clientbound::play::SetHeldItemPacket::ID => {
                let packet = clientbound::play::SetHeldItemPacket::from_packet(raw_packet);
                get_logger().info(format!("Held Slot Changed: {:?}", packet));
            }

            clientbound::play::SpawnEntityPacket::ID => {
                let packet = clientbound::play::SpawnEntityPacket::from_packet(raw_packet);
                get_logger().info(format!("SpawnEntityPacket: {:?}", packet));
            }

            id => {
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

    fn play(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Play);

        let mut bundle_packets: Vec<PacketContainer> = Vec::new();
        let mut do_bundle: bool = false;

        // play phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);

            match raw_packet.get_id() {
                clientbound::play::BundleDelimiterPacket::ID => {
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
                clientbound::play::DisconnectPacket::ID => {
                    let packet = clientbound::play::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Disconnected: {:?}", packet.reason));
                    self.state = ConnectionState::Handshaking;
                    break;
                }

                clientbound::play::KeepAlivePacket::ID => {
                    let packet = clientbound::play::KeepAlivePacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    serverbound::play::KeepAlivePacket {
                        keepalive_id: packet.keepalive_id.clone(),
                    }
                    .send(stream);
                }

                clientbound::play::LoginPacket::ID => {
                    let packet = clientbound::play::LoginPacket::from_packet(raw_packet);
                    get_logger().info(format!("Successfully Logged In!: {:?}", packet));
                }

                clientbound::play::PlayerAbilitiesPacket::ID => {
                    let packet = clientbound::play::PlayerAbilitiesPacket::from_packet(raw_packet);
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
