mod events;

// implements a connection loop
use std::net::{TcpStream, ToSocketAddrs};

use std::thread;
use std::time::Duration;

use crate::minecraft::PROTOCOL_VERSION;
use crate::minecraft::packets::set_threshold;

use super::packets as mcp;
use super::packets::client::ClientMainHand;
use super::packets::server::{Base64Image, Location, StatusResponse};
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

    pub fn ping(&mut self) -> StatusResponse {
        let mut stream = connect(&self.hostname, self.port);

        // send handshake start packet
        mcp::client::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            &self.hostname,
            self.port,
            mcp::client::HandshakeRequest::STATUS,
        )
        .send(&mut stream);

        // send handshake status packet
        mcp::client::HandshakeStatusPacket::new().send(&mut stream);

        // read the response packet
        mcp::server::StatusResponsePacket::recv(&mut stream).get_status()
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
        mcp::client::HandshakeStartPacket::new(
            PROTOCOL_VERSION,
            self.hostname.as_str(),
            self.port,
            mcp::client::HandshakeRequest::LOGIN,
        )
        .send(stream);

        // send handshake login packet
        mcp::client::HandshakeLoginPacket::new(self.username.to_string(), 0).send(stream);

        self.state = ConnectionState::Login;
    }

    fn login(&mut self, stream: &mut TcpStream) {
        assert_eq!(self.state, ConnectionState::Login);

        // login phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                mcp::server::LoginDisconnectPacket::ID => {
                    let packet = mcp::server::LoginDisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Login Failed!: {:?}", packet));
                    panic!()
                }
                mcp::server::EncryptionRequestPacket::ID => {
                    let packet = mcp::server::EncryptionRequestPacket::from_packet(raw_packet);
                    get_logger().error(format!(
                        "Encryption Requested (not implemented): {:?}",
                        packet
                    ));
                    panic!()
                }
                mcp::server::LoginSuccessPacket::ID => {
                    let packet = mcp::server::LoginSuccessPacket::from_packet(raw_packet);
                    // send login acknowledged packet to move to configuration phase
                    mcp::client::LoginAcknowledgedPacket::new().send(stream);
                    get_logger().info(format!(
                        "Login Finished! properties:{:?}",
                        packet.get_properties()
                    ));
                    // set state to configuration
                    self.state = ConnectionState::Configuration;
                    break;
                }
                mcp::server::SetCompressionPacket::ID => {
                    let packet = mcp::server::SetCompressionPacket::from_packet(raw_packet);
                    // set global compression threshold
                    set_threshold(packet.get_threshold());
                    get_logger().info(format!(
                        "Set Compression: threshold={}",
                        packet.get_threshold()
                    ));
                }
                0x04 => {
                    get_logger().warn(format!("Ignoring login plugin messages (not implemented)."))
                }
                mcp::server::LoginCookieRequest::ID => {
                    let packet = mcp::server::LoginCookieRequest::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "Ignoring cookie request (not implemented): {:?}",
                        packet
                    ))
                }
                // TODO: don't completely die just because one packet is not supported
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
        mcp::client::ClientInformationPacket::default().send(stream);

        // configuration phase loop
        loop {
            // read one packet from the stream
            let raw_packet = PacketContainer::recv(stream);
            match raw_packet.get_id() {
                mcp::server::ConfigCookieRequest::ID => {
                    let packet = mcp::server::ConfigCookieRequest::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "Ignoring cookie request (not implemented): {:?}",
                        packet
                    ))
                }
                0x01 => {
                    get_logger().warn(format!("Ignoring login plugin messages (not implemented)."))
                }
                mcp::server::ConfigDisconnectPacket::ID => {
                    let packet = mcp::server::ConfigDisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Configuration Failed!: {:?}", packet));
                    panic!();
                }

                mcp::server::FinishConfigurationPacket::ID => {
                    let packet = mcp::server::FinishConfigurationPacket::from_packet(raw_packet);
                    get_logger().info(format!("Configuration Finished!: {:?}", packet));
                    // send finish configuration acknowledged packet
                    mcp::client::AcknowledgeFinishConfigPacket::new().send(stream);
                    // set state to play
                    self.state = ConnectionState::Play;
                    break;
                }

                mcp::server::ClientBoundKeepAlivePacket::ID => {
                    let keepalive =
                        mcp::server::ClientBoundKeepAlivePacket::from_packet(raw_packet);
                    // respond to keepalive packet
                    mcp::client::ServerBoundKeepAlivePacket::new(*keepalive.get_id()).send(stream);
                }

                0x06 => {
                    get_logger().warn(format!("Ignoring reset chat packet (not implemented)."));
                }

                mcp::server::RegistryDataPacket::ID => {
                    let packet = mcp::server::RegistryDataPacket::from_packet(raw_packet);
                    get_logger().warn(format!(
                        "WARNING: Ignored registry data packet: {:?}",
                        packet.get_registry_id()
                    ));
                }

                0x0D => {
                    get_logger().warn(format!("WARNING: Ignored update tags packet"));
                }

                // TODO: implement packets
                mcp::server::KnownServerPacksPacket::ID => {
                    let packet = mcp::server::KnownServerPacksPacket::from_packet(raw_packet);
                    get_logger().info(format!("Known Packs: {:?}", packet));
                    mcp::client::KnownClientPacksPacket::new(vec![mcp::client::KnownClientPack {
                        namespace: "minecraft".to_string(),
                        id: "core".to_string(),
                        version: "1.21.1".to_string(),
                    }])
                    .send(stream);
                }

                mcp::server::ConfigFeatureFlagsPacket::ID => {
                    let packet = mcp::server::ConfigFeatureFlagsPacket::from_packet(raw_packet);
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

    fn process_play_packet(&mut self, raw_packet: PacketContainer, stream: &mut TcpStream) {
        match raw_packet.get_id() {
            mcp::server::BundleDelimiterPacket::ID => {
                get_logger().warn(format!("ignoring BundleDelimiterPacket"));
            }
            mcp::server::PlayDisconnectPacket::ID => {
                get_logger().warn(format!(" ignoring PlayDisconnectPacket"));
            }

            mcp::server::SyncPlayerPositionPacket::ID => {
                let packet = mcp::server::SyncPlayerPositionPacket::from_packet(raw_packet);
                let new_location = packet.apply_changes(self.location);
                get_logger().info(format!("Teleported by server: {:?}", new_location));
                self.location = new_location;
                // send teleport confirmation packet
                mcp::server::ConfirmTeleportationPacket::new(packet.get_teleport_id()).send(stream);
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
                mcp::server::BundleDelimiterPacket::ID => {
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
                mcp::server::PlayDisconnectPacket::ID => {
                    let packet = mcp::server::PlayDisconnectPacket::from_packet(raw_packet);
                    get_logger().error(format!("Disconnected: {:?}", packet.get_reason()));
                }

                mcp::server::PlayLoginPacket::ID => {
                    let packet = mcp::server::PlayLoginPacket::from_packet(raw_packet);
                    get_logger().info(format!("Successfully Logged In!: {:?}", packet));
                }

                mcp::server::PlayChangeDifficultyPacket::ID => {
                    let packet = mcp::server::PlayChangeDifficultyPacket::from_packet(raw_packet);
                    get_logger().info(format!("Difficulty Changed: {:?}", packet));
                }

                mcp::server::PlayPlayerAbilitiesPacket::ID => {
                    let packet = mcp::server::PlayPlayerAbilitiesPacket::from_packet(raw_packet);
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
            thread::sleep(Duration::from_millis(20));
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
