mod events;

// implements a connection loop
use std::net::{TcpStream, ToSocketAddrs};

use std::time::Duration;

use crate::minecraft::packets::client::configuration::ClientMainHand;
use crate::minecraft::packets::server::status::StatusResponse;
use crate::minecraft::packets::set_threshold;
use crate::minecraft::{PROTOCOL_VERSION, types};

use super::packets as mcp;
use super::packets::server::Location;
use super::packets::{
    ConnectionState, Packet, PacketContainer, PacketIn, PacketOut, PacketRecv, PacketSend,
};

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

    location: Location,
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
            location: Location::new(0.0, 0.0, 0.0, 180.0, 0.0),
        }
    }

    pub fn status_request(&mut self) -> StatusResponse {
        let mut stream = connect(&self.hostname, self.port);

        // send handshake start packet
        mcp::client::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            &self.hostname,
            self.port,
            mcp::client::handshake::HandshakeRequest::STATUS,
        )
        .send(&mut stream);

        self.state = ConnectionState::Status;

        // send status request packet to get the server's motd
        mcp::client::status::RequestPacket::new().send(&mut stream);

        // the next packet the server sends us must be a status reponse packet
        mcp::server::status::ResponsePacket::recv(&mut stream).get_status()
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
        mcp::client::handshake::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            self.hostname.as_str(),
            self.port,
            mcp::client::handshake::HandshakeRequest::LOGIN,
        )
        .send(stream);

        self.state = ConnectionState::Login;
    }

    fn login(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Login);

        // send login start packet
        mcp::client::login::LoginStartPacket::new(self.username.to_string(), 0).send(stream);

        // login phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                mcp::server::login::DisconnectPacket::ID => {
                    let packet = mcp::server::login::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Login Failed!: {:?}", packet));
                    panic!()
                }
                mcp::server::login::EncryptionRequestPacket::ID => {
                    let packet =
                        mcp::server::login::EncryptionRequestPacket::from_packet(raw_packet);
                    if packet.should_authenticate() {
                        get_logger().error(format!("Online mode is not suported!"));
                    }
                    get_logger().error(format!("Encryption is not suported: {:?}", packet));
                    panic!()
                }
                mcp::server::login::LoginSuccessPacket::ID => {
                    let packet = mcp::server::login::LoginSuccessPacket::from_packet(raw_packet);
                    // send login acknowledged packet to move to configuration phase
                    mcp::client::login::LoginAcknowledgedPacket::new().send(stream);
                    get_logger().info(format!(
                        "Login Finished! properties:{:?}",
                        packet.get_properties()
                    ));
                    // set state to configuration
                    self.state = ConnectionState::Configuration;
                    break;
                }
                mcp::server::login::SetCompressionPacket::ID => {
                    let packet = mcp::server::login::SetCompressionPacket::from_packet(raw_packet);
                    // set global compression threshold
                    set_threshold(packet.get_threshold());
                    get_logger().info(format!(
                        "Set Compression: threshold={}",
                        packet.get_threshold()
                    ));
                }
                mcp::server::login::PluginRequestPacket::ID => {
                    // Unlike plugin messages in "play" mode, these messages follow a lock-step request/response scheme,
                    // where the client is expected to respond to a request indicating whether it understood. The
                    // notchian client always responds that it hasn't understood, and sends an empty payload.

                    let packet = mcp::server::login::PluginRequestPacket::from_packet(raw_packet);
                    get_logger().info(format!("LoginPluginRequestPacket: {:?}", packet));

                    mcp::client::login::LoginPluginResponsePacket::new(
                        packet.get_message_id(),
                        false,
                        Vec::new(),
                    )
                    .send(stream);
                }
                mcp::server::login::CookieRequest::ID => {
                    let packet = mcp::server::login::CookieRequest::from_packet(raw_packet);
                    get_logger().warn(format!("LoginCookieRequest: {:?}", packet));

                    mcp::client::login::CookieResponsePacket::new(packet.get_key(), None)
                        .send(stream);
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

        // send a default client information packet, otherwise we might not be able to join
        mcp::client::configuration::ClientInformationPacket::default().send(stream);

        // configuration phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                mcp::server::configuration::CookieRequestPacket::ID => {
                    let packet =
                        mcp::server::configuration::CookieRequestPacket::from_packet(raw_packet);
                    get_logger().warn(format!("CookieRequestPacket: {:?}", packet));
                    mcp::client::configuration::CookieResponsePacket {
                        key: packet.key,
                        payload: types::Optional::None,
                    }
                    .send(stream);
                }
                mcp::server::configuration::PluginMessagesPacket::ID => {
                    let packet =
                        mcp::server::configuration::PluginMessagesPacket::from_packet(raw_packet);
                    get_logger().warn(format!("Ignoring login plugin message: {:?}", packet))
                }
                mcp::server::configuration::DisconnectPacket::ID => {
                    let packet =
                        mcp::server::configuration::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Configuration Failed!: {:?}", packet));
                    panic!();
                }

                mcp::server::configuration::ConfigurationFinishPacket::ID => {
                    let packet = mcp::server::configuration::ConfigurationFinishPacket::from_packet(
                        raw_packet,
                    );
                    get_logger().info(format!("Configuration Finished!: {:?}", packet));
                    // send finish configuration acknowledged packet
                    mcp::client::configuration::ConfigurationAcknowledgedPacket.send(stream);
                    // set state to play
                    self.state = ConnectionState::Play;
                    break;
                }

                mcp::server::configuration::KeepAlivePacket::ID => {
                    let keepalive =
                        mcp::server::configuration::KeepAlivePacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    mcp::client::configuration::KeepAlivePacket {
                        keepalive_id: keepalive.keepalive_id,
                    }
                    .send(stream);
                }

                mcp::server::configuration::ResetChatPacket::ID => {
                    let packet =
                        mcp::server::configuration::ResetChatPacket::from_packet(raw_packet);
                    get_logger().info(format!("ResetChatPacket: {:?}", packet));
                }

                mcp::server::configuration::RegistryDataPacket::ID => {
                    let packet =
                        mcp::server::configuration::RegistryDataPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored registry data packet: {:?}",
                        packet.registry_id
                    ));
                }

                mcp::server::configuration::AddResourcePackPacket::ID => {
                    let packet =
                        mcp::server::configuration::AddResourcePackPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored add resource pack packet: {:?}",
                        packet
                    ));
                }

                mcp::server::configuration::RemoveResourcePackPacket::ID => {
                    let packet = mcp::server::configuration::RemoveResourcePackPacket::from_packet(
                        raw_packet,
                    );
                    get_logger().warn(format!(
                        "WARNING: Ignored remove resource pack packet: {:?}",
                        packet
                    ));
                }

                0x0D => {
                    get_logger().warn(format!("WARNING: Ignored update tags packet"));
                }

                // TODO: implement packets
                mcp::server::configuration::KnownServerPacksPacket::ID => {
                    let packet =
                        mcp::server::configuration::KnownServerPacksPacket::from_packet(raw_packet);
                    get_logger().info(format!("Known Packs: {:?}", packet));
                    mcp::client::configuration::KnownClientPacksPacket {
                        packs: vec![mcp::client::configuration::KnownClientPack {
                            namespace: "minecraft".into(),
                            id: "core".into(),
                            version: "1.21.1".into(),
                        }]
                        .into(),
                    }
                    .send(stream);
                }

                mcp::server::configuration::FeatureFlagsPacket::ID => {
                    let packet =
                        mcp::server::configuration::FeatureFlagsPacket::from_packet(raw_packet);
                    get_logger().info(format!("Feature Flags: {:?}", packet));
                }

                mcp::server::configuration::PongPacket::ID => {
                    let packet = mcp::server::configuration::PongPacket::from_packet(raw_packet);
                    get_logger().info(format!("PongPacket: {:?}", packet));
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

    fn process_play_packet(&mut self, raw_packet: PacketContainer, stream: &mut TcpStream) {
        match raw_packet.get_id() {
            mcp::server::play::SyncPlayerPositionPacket::ID => {
                let packet = mcp::server::play::SyncPlayerPositionPacket::from_packet(raw_packet);
                let new_location = packet.apply_changes(self.location);
                get_logger().info(format!("Teleported by server: {:?}", new_location));
                self.location = new_location;
                // send teleport confirmation packet
                mcp::server::play::ConfirmTeleportationPacket::new(packet.get_teleport_id())
                    .send(stream);
            }

            mcp::server::play::ChangeDifficultyPacket::ID => {
                let packet = mcp::server::play::ChangeDifficultyPacket::from_packet(raw_packet);
                get_logger().info(format!("Difficulty Changed: {:?}", packet));
            }

            mcp::server::play::SetHeldItemPacket::ID => {
                let packet = mcp::server::play::SetHeldItemPacket::from_packet(raw_packet);
                get_logger().info(format!("Held Slot Changed: {:?}", packet));
            }

            mcp::server::play::SpawnEntityPacket::ID => {
                let packet = mcp::server::play::SpawnEntityPacket::from_packet(raw_packet);
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
                mcp::server::play::BundleDelimiterPacket::ID => {
                    // let packet: mcp::server::BundleDelimiterPacket = mcp::server::BundleDelimiterPacket::from_packet(raw_packet);
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
                mcp::server::play::DisconnectPacket::ID => {
                    let packet = mcp::server::play::DisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Disconnected: {:?}", packet.get_reason()));
                    self.state = ConnectionState::Handshaking;
                    break;
                }

                mcp::server::play::KeepAlivePacket::ID => {
                    let packet = mcp::server::play::KeepAlivePacket::from_packet(raw_packet);
                    get_logger().debug(format!("KeepAlive: {:?}", packet));
                    // respond to keepalive packet
                    mcp::client::play::KeepAlivePacket::new(*packet.get_id()).send(stream);
                }

                mcp::server::play::LoginPacket::ID => {
                    let packet = mcp::server::play::LoginPacket::from_packet(raw_packet);
                    get_logger().info(format!("Successfully Logged In!: {:?}", packet));
                }

                mcp::server::play::PlayerAbilitiesPacket::ID => {
                    let packet = mcp::server::play::PlayerAbilitiesPacket::from_packet(raw_packet);
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
