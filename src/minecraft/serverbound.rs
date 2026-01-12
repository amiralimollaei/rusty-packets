pub mod handshake;
pub mod status;
pub mod login;
pub mod configuration;
pub mod play;

use crate::minecraft::serverbound;
pub use serverbound::handshake::ServerboundHandshakePacket;
pub use serverbound::status::ServerboundStatusPacket;
pub use serverbound::login::ServerboundLoginPacket;
pub use serverbound::configuration::ServerboundConfigurationPacket;
pub use serverbound::play::ServerboundPlayPacket;