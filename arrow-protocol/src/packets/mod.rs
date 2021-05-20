use serde::Deserialize;

use self::{common::*, error::PacketError};
use crate::serde::de::Deserializer;

/// Here are all packets that have the same data.
pub mod common;
pub mod error;

pub trait Packet {
    fn id(protocol_version: i32) -> i32
    where
        Self: Sized;

    fn data_bytes(&self) -> Result<Vec<u8>, PacketError>;
}

pub enum PacketKind {
    Handshake {
        protocol_version: i32,
        host: String,
        port: u16,
        next_state: i32,
    },
    LoginStart(String),
}

#[derive(Debug, Clone)]
pub enum State {
    Handshake,
    Login,
    Play,
    Status,
}

impl PacketKind {
    pub fn get_packet(self) -> Box<dyn Packet> {
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
