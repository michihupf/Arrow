use serde::Deserialize;

use self::{common::*, error::PacketError};
use crate::serde::de::Deserializer;

/// All packets that have the same data over all versions.
pub mod common;
/// The error module for packets.
pub mod error;

/// A trait giving functions to get the packet id and serialize it.
pub trait Packet {
    /// Returns the id for the given protocol version.
    fn id(protocol_version: i32) -> i32
    where
        Self: Sized;

    /// Serialize the packet.
    fn data_bytes(&self) -> Result<Vec<u8>, PacketError>;
}

/// A multi-version representation for packets.
pub enum PacketKind {
    /// The [Handshake](https://wiki.vg/Protocol#Handshake) packet.
    Handshake {
        /// The protocol version of the client.
        protocol_version: i32,
        /// The host the client is connected to.
        host: String,
        /// The port the client is connected to.
        port: u16,
        /// The next state. Can be 1 or 2.
        next_state: i32,
    },
    /// The [Login Start](https://wiki.vg/Protocol#Login_Start) packet.
    LoginStart(String),
}

#[derive(Debug, Clone)]
/// The states of the protocol.
pub enum State {
    /// The first state, reached when connecting to the server.
    Handshake,
    /// The login state. It can be reached by sending a [Handshake](PacketKind::Handshake) packet
    /// with `next_state` set to 2.
    Login,
    /// The play state. It can be reached by successfully logging in in the [Login](State::Login)
    /// state.
    Play,
    /// The status state. It can be reached by sending a [Handshake](PacketKind::Handshake) packet
    /// with `next_state` set to 1.
    Status,
}

impl PacketKind {
    /// Gets a [`Packet`] using `self` and the protocol version.
    pub fn into_packet(self, protocol_version: i32) -> Box<dyn Packet> {
        use PacketKind::*;

        match self {
            Handshake {
                protocol_version,
                host,
                port,
                next_state,
            } => Box::new(handshake::serverbound::Handshake::new(
                protocol_version,
                host,
                port,
                next_state,
            )),
            LoginStart(name) => Box::new(login::LoginStart::new(name)),
        }
    }

    /// Gets `self` using the [`state`](State), the information if its clientbound or serverbound,
    /// the protocol version, the id and the data.
    pub fn from_bytes(
        state: State,
        is_serverbound: bool,
        protocol_version: i32,
        id: i32,
        data: Vec<u8>,
    ) -> Result<Self, PacketError> {
        let mut de = Deserializer::new(data);

        if is_serverbound {
            match state {
                State::Handshake => match id {
                    0 => {
                        let packet = login::LoginStart::deserialize(&mut de)?;

                        Ok(PacketKind::LoginStart(packet.name().clone()))
                    }
                    i => return Err(PacketError::InvalidPacketId(i, state)),
                },
                State::Login => match id {
                    i if i == login::LoginStart::id(protocol_version) => {
                        let packet = login::LoginStart::deserialize(&mut de)?;

                        Ok(PacketKind::LoginStart(packet.name().clone()))
                    }
                    i => return Err(PacketError::InvalidPacketId(i, state)),
                },
                State::Play => todo!(),
                State::Status => todo!(),
            }
        } else {
            todo!("add clientbound support");
        }
    }
}
