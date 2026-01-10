// mod events;

// implements a connection loop
use std::net::{TcpStream, ToSocketAddrs};

use std::time::Duration;

use crate::minecraft::packet::GenericPacket;

use super::clientbound::ClientboundHandshakePacket;
use super::clientbound::ClientboundStatusPacket;
use super::clientbound::ClientboundLoginPacket;
use super::clientbound::ClientboundConfigurationPacket;
use super::clientbound::ClientboundPlayPacket;

use super::serverbound::configuration::ClientMainHand;
use super::packet::{
    ConnectionState, Packet, RawPacket, PacketSendRecv, set_threshold,
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

    pub fn status_request(hostname: &str, port: u16) -> clientbound::status::StatusResponse {
        let mut stream = connect(hostname, port);

        // send handshake start packet
        serverbound::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            hostname,
            port,
            serverbound::handshake::HandshakeRequest::STATUS,
        ).send(&mut stream);

        // send status request packet to get the server's motd
        serverbound::status::StatusRequestPacket.send(&mut stream);

        // the next packet the server sends us must be a status reponse packet
        clientbound::status::StatusResponsePacket::recv(&mut stream).deseralize()
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
            // read one packet from the stream and process
            match ClientboundLoginPacket::recv(stream) {
                ClientboundLoginPacket::Disconnect(packet) => {
                    get_logger().error(format!("Login Failed!: {:?}", packet));
                    panic!()
                },
                ClientboundLoginPacket::EncryptionRequest(packet) => {
                    if packet.should_authenticate.into() {
                        get_logger().error(format!("Online mode is not suported!"));
                    }
                    get_logger().error(format!("Encryption is not suported: {:?}", packet));
                    panic!()
                },
                ClientboundLoginPacket::LoginSuccess(packet) => {
                    get_logger().info(format!(
                        "Login Finished! properties:{:?}",
                        packet.properties
                    ));
                    // send login acknowledged packet and move on to configuration state
                    serverbound::login::LoginAcknowledgedPacket.send(stream);
                    self.state = ConnectionState::Configuration;
                    break;
                },
                ClientboundLoginPacket::SetCompression(packet) => {
                    // set global compression threshold
                    set_threshold(packet.threshold.into());
                    get_logger().info(format!(
                        "Set Compression: threshold={}",
                        packet.threshold.get_value()
                    ));
                },
                ClientboundLoginPacket::PluginRequest(packet) => {
                    // Unlike plugin messages in "play" mode, these messages follow a lock-step request/response scheme,
                    // where the client is expected to respond to a request indicating whether it understood. The
                    // notchian client always responds that it hasn't understood, and sends an empty payload.

                    get_logger().info(format!("LoginPluginRequestPacket: {:?}", packet));

                    serverbound::login::LoginPluginResponsePacket {
                        message_id: packet.message_id.clone(),
                        successful: false.into(),
                        data: Vec::new().into(),
                    }
                    .send(stream);
                },
                ClientboundLoginPacket::CookieRequest(packet) => {
                    get_logger().warn(format!("LoginCookieRequest: {:?}", packet));

                    serverbound::login::CookieResponsePacket::new(packet.key.into(), None).send(stream);
                },
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
            // read one packet from the stream and process
            match ClientboundConfigurationPacket::recv(stream) {
                ClientboundConfigurationPacket::CookieRequest(packet) => {
                    get_logger().warn(format!("CookieRequestPacket: {:?}", packet));
                    serverbound::configuration::CookieResponsePacket {
                        key: packet.key,
                        payload: types::Optional::None,
                    }.send(stream);
                },
                ClientboundConfigurationPacket::PluginMessage(packet) => {
                    get_logger().warn(format!("Ignored PluginMessagePacket: {:?}", packet))
                },
                ClientboundConfigurationPacket::Disconnect(packet) => {
                    get_logger().error(format!("Configuration Failed: {:?}", packet.reason));
                    panic!();
                },
                ClientboundConfigurationPacket::ConfigurationFinish(packet) => {
                    get_logger().info(format!("Configuration Finished!: {:?}", packet));
                    // send finish configuration acknowledged packet
                    serverbound::configuration::AcknowledgeFinishConfigurationPacket.send(stream);
                    // set state to play
                    self.state = ConnectionState::Play;
                    break;
                },
                ClientboundConfigurationPacket::KeepAlive(packet) => {
                    // respond to keepalive packet
                    serverbound::configuration::ServerboundKeepAlivePacket {
                        keepalive_id: packet.keepalive_id,
                    }.send(stream);
                },
                ClientboundConfigurationPacket::Ping(packet) => {
                    // respond to keepalive packet
                    serverbound::configuration::PongPacket {
                        timestamp: packet.timestamp,
                    }.send(stream);
                },
                ClientboundConfigurationPacket::ResetChat(packet) => {
                    get_logger().info(format!("ResetChatPacket: {:?}", packet));
                },
                ClientboundConfigurationPacket::RegistryData(packet) => {
                    get_logger().warn(format!(
                        "Ignored registry data packet: {:?}",
                        packet.registry_id
                    ));
                },
                ClientboundConfigurationPacket::RemoveResourcePack(packet) => {
                    get_logger().warn(format!(
                        "Ignored remove resource pack packet: {:?}",
                        packet
                    ));
                },
                ClientboundConfigurationPacket::AddResourcePack(packet) => {
                    get_logger().warn(format!(
                        "Ignored add resource pack packet: {:?}",
                        packet
                    ));
                },
                ClientboundConfigurationPacket::StoreCookie(packet) => {
                    get_logger().warn(format!("Ignored StoreCookiePacket: {:?}", packet));
                },
                ClientboundConfigurationPacket::Transfer(packet) => todo!("transfer packet is not supported, but got: {:?}", packet),
                ClientboundConfigurationPacket::FeatureFlags(packet) => {
                    get_logger().info(format!("Feature Flags: {:?}", packet.feature_flags));
                },
                ClientboundConfigurationPacket::UpdateTags(packet) => {
                    get_logger().warn(format!("Ignored UpdateTagsPacket: {:?}", packet));
                },
                ClientboundConfigurationPacket::KnownServerPacks(packet) => {
                    get_logger().info(format!("Known Server Packs: {:?}", packet.packs));

                    // respond with the default notchain response
                    serverbound::configuration::KnownClientPacksPacket {
                        packs: vec![serverbound::configuration::ServerboundKnownPacksPacket {
                            namespace: "minecraft".into(),
                            id: "core".into(),
                            version: "1.21.1".into(),
                        }]
                        .into(),
                    }
                    .send(stream);
                },
                ClientboundConfigurationPacket::CustomReportDetails(packet) => {
                    get_logger().info(format!("Custom Report Details: {:?}", packet.details));
                },
                ClientboundConfigurationPacket::ServerLinks(packet) => {
                    get_logger().info(format!("Server Links: {:?}", packet.links));
                },
            }
        }
    }

    pub fn execute_synchronize_player_position_packet(
        &mut self,
        packet: &clientbound::play::SynchronizePlayerPositionPacket,
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

    fn process_play_bundle_packets(&mut self, bundle_packets: Vec<ClientboundPlayPacket>) {
        for packet in bundle_packets {
            match packet { 
                ClientboundPlayPacket::ChangeDifficulty(packet) => {
                    get_logger().info(format!("Difficulty Changed: {:?}", packet));
                },
                ClientboundPlayPacket::SetHeldItem(packet) => {
                    get_logger().info(format!("Held Slot Changed: {:?}", packet));
                },
                ClientboundPlayPacket::SpawnEntity(packet) => {
                    get_logger().info(format!("SpawnEntityPacket: {:?}", packet));
                },
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

            match packet {
                // packets that are bundled when processing
                ClientboundPlayPacket::BundleDelimiter(packet) => {
                    get_logger().debug(format!("BundleDelimiterPacket"));
                    self.process_play_bundle_packets(bundle_packets);
                    bundle_packets = Vec::new();
                }
                
                // packets that are not bundled when processing
                ClientboundPlayPacket::SynchronizePlayerPosition(packet) => {
                    self.execute_synchronize_player_position_packet(&packet);
                    get_logger().info(format!("Teleported by server: {:?}", self.location));
                    // send teleport confirmation packet
                    serverbound::play::ConfirmTeleportationPacket {
                        teleport_id: packet.teleport_id.clone(),
                    }
                    .send(stream);
                },

                // excluded from bundle delimiter because the server closes the connection after this packet
                ClientboundPlayPacket::Disconnect(packet) => {
                    get_logger().error(format!("Disconnected: {:?}", packet.reason));
                    self.state = ConnectionState::Handshaking;
                    break;
                }

                ClientboundPlayPacket::KeepAlive(packet) => {
                    // respond to keepalive packet
                    serverbound::play::KeepAlivePacket {
                        keepalive_id: packet.keepalive_id.clone(),
                    }
                    .send(stream);
                }

                ClientboundPlayPacket::Login(packet) => {
                    get_logger().info(format!("Successfully Logged In!: {:?}", packet));
                }

                ClientboundPlayPacket::PlayerAbilities(packet) => {
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
