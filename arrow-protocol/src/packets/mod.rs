/// All packets that have the same data over all versions.
pub mod common;
/// The error module for packets.
pub mod error;
/// All common types used in packets
pub mod types;
/// All version specific packets and types.
pub mod version_specific;

use std::fmt::Display;

use serde::Deserialize;
use uuid::Uuid;

use self::{common::*, error::PacketError, types::{Difficulty, Gamemode, LevelType}, version_specific::{play::v754::clientbound::JoinGame, types::v47::Dimension}};
use crate::serde::{de::Deserializer, varint::VarInt};

/// A trait giving functions to get the packet id and serialize it.
pub trait Packet {
    /// Returns the id for the given protocol version.
    fn id(protocol_version: i32) -> i32
    where
        Self: Sized;

    /// Returns the id for the given protocol version.
    fn self_id(&self, protocol_version: i32) -> i32;

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
    /// The [Login Success](https://wiki.vg/Protocol#Login_Success) packet.
    LoginSuccess(Uuid, String),
    /// The [Request](https://wiki.vg/Protocol#Status) packet
    StatusRequest,
    /// The [Response](https://wiki.vg/Protocol#Status) packet
    StatusResponse(common::status::ResponseData),
    /// The [Ping](https://wiki.vg/Protocol#Status) packet
    StatusPing(i64),
    /// The [Pong](https://wiki.vg/Protocol#Status) packet
    StatusPong(i64),
    /// The [JoinGame](https://wiki.vg/Protocol#Join_Game) packet
    JoinGame {
        /// This is the player's Entity ID (EID).
        entity_id: i32,
        /// True if the servers difficulty is hardcore
        is_hardcore: bool,
        /// 0: survival, 1: creative, 2: adventure, 3: spectator.
        gamemode: Gamemode,
        /// 0: survival, 1: creative, 2: adventure, 3: spectator. The hardcore flag is not included. The previous gamemode.
        previous_gamemode: Gamemode,
        /// Specifies how many worlds are present on the server
        world_count: VarInt,
        /// Identifiers for all worlds on the server
        world_names: Vec<String>,
        /// The full extent of these is still unknown, but the tag represents a dimension and biome registry. See below for the vanilla default.
        dimension_codec: Vec<u8>,
        /// Valid dimensions are defined per dimension registry sent before this
        dimension: Vec<u8>,
        /// Dimension is defined here
        dimension_47: Dimension,
        /// The difficulty of the server
        difficulty: Difficulty,
        /// Name of the world being spawned into
        world_name: String,
        /// First 8 bytes of the SHA-256 hash of the world's seed. Used client side for biome noise
        hashed_seed: i64,
        /// Name of the world being spawned into.
        max_players: i32,
        /// Level type specified here: default, flat, largeBiomes, amplified, customized, buffet, default_1_1
        level_type: LevelType,
        /// Render distance (2-32).
        view_distance: VarInt,
        /// If true, a Notchian client shows reduced information on the debug screen. For servers in development, this should almost always be false.
        reduced_debug_info: bool,
        /// Set to false when the doImmediateRespawn gamerule is true.
        enable_respawn_screen: bool,
        /// True if the world is a debug mode world; debug mode worlds cannot be modified and have predefined blocks
        is_debug: bool,
        /// True if the world is a superflat world; flat worlds have different void fog and a horizon at y=0 instead of y=63
        is_flat: bool,
    },
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
    pub fn into_packet(self, protocol_version: i32) -> Result<Box<dyn Packet>, PacketError> {
        use PacketKind::*;

        match self {
            Handshake {
                protocol_version,
                host,
                port,
                next_state,
            } => Ok(Box::new(handshake::serverbound::Handshake::new(
                protocol_version,
                host,
                port,
                next_state,
            ))),
            LoginStart(name) => Ok(Box::new(common::login::serverbound::LoginStart::new(name))),
            LoginSuccess(uuid, name) => {
                if protocol_version >= 707 {
                    Ok(Box::new(
                        version_specific::login::v707::clientbound::LoginSuccess::new(uuid, name),
                    ))
                } else {
                    Ok(Box::new(
                        version_specific::login::v47::clientbound::LoginSuccess::new(
                            uuid.to_hyphenated().to_string(),
                            name,
                        ),
                    ))
                }
            }
            StatusRequest => Ok(Box::new(common::status::serverbound::Request::new())),
            StatusResponse(json_response) => Ok(Box::new(
                match common::status::clientbound::Response::new(json_response) {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                },
            )),
            StatusPing(payload) => Ok(Box::new(common::status::serverbound::Ping::new(payload))),
            StatusPong(payload) => Ok(Box::new(common::status::clientbound::Pong::new(payload))),
            JoinGame {
                entity_id,
                is_hardcore,
                gamemode,
                previous_gamemode,
                world_count,
                world_names,
                dimension_codec,
                dimension,
                dimension_47,
                difficulty,
                world_name,
                hashed_seed,
                max_players,
                level_type,
                view_distance,
                reduced_debug_info,
                enable_respawn_screen,
                is_debug,
                is_flat,
            } => {
                if protocol_version >= 754 {
                    Ok(Box::new(
                        version_specific::play::v754::clientbound::JoinGame::new(
                            entity_id,
                            is_hardcore,
                            gamemode as u8,
                            previous_gamemode as i8,
                            world_count,
                            world_names,
                            dimension_codec,
                            dimension,
                            world_name,
                            hashed_seed,
                            VarInt(max_players),
                            view_distance,
                            reduced_debug_info,
                            enable_respawn_screen,
                            is_debug,
                            is_flat,
                        ),
                    ))
                } else if protocol_version >= 522 {
                    Ok(Box::new(
                        version_specific::play::v552::clientbound::JoinGame::new(
                            entity_id,
                            gamemode as u8 | ((is_hardcore as u8) << 3),
                            dimension_47 as i32,
                            hashed_seed,
                            max_players as u8,
                            level_type,
                            view_distance,
                            reduced_debug_info,
                            enable_respawn_screen,
                        )
                    ))
                } else if protocol_version >= 468 {
                    Ok(Box::new(
                        version_specific::play::v468::clientbound::JoinGame::new(
                            entity_id,
                            gamemode as u8 | ((is_hardcore as u8) << 3),
                            dimension_47 as i32,
                            max_players as u8,
                            level_type,
                            view_distance,
                            reduced_debug_info,
                        )
                    ))
                } else if protocol_version >= 464 {
                    Ok(Box::new(
                        version_specific::play::v464::clientbound::JoinGame::new(
                            entity_id,
                            gamemode as u8 | ((is_hardcore as u8) << 3),
                            dimension_47 as i32,
                            max_players as u8,
                            level_type,
                            reduced_debug_info,
                        )
                    ))
                } else if protocol_version >= 108 {
                    Ok(Box::new(
                        version_specific::play::v108::clientbound::JoinGame::new(
                            entity_id,
                            gamemode as u8 | ((is_hardcore as u8) << 3),
                            dimension_47 as i32,
                            difficulty as u8,
                            max_players as u8,
                            level_type,
                            reduced_debug_info,
                        )
                    ))
                } else {
                    Ok(Box::new(
                        version_specific::play::v47::clientbound::JoinGame::new(
                            entity_id,
                            gamemode as u8 | ((is_hardcore as u8) << 3),
                            dimension_47 as i8,
                            difficulty as u8,
                            max_players as u8,
                            level_type,
                            reduced_debug_info,
                        )
                    ))
                }
            }
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
                        let packet = handshake::serverbound::Handshake::deserialize(&mut de)?;

                        Ok(PacketKind::Handshake {
                            protocol_version: packet.protocol_version.0,
                            host: packet.host,
                            port: packet.port,
                            next_state: packet.next_state.0,
                        })
                    }
                    i => return Err(PacketError::InvalidPacketId(i, state)),
                },
                State::Login => match id {
                    i if i == login::serverbound::LoginStart::id(protocol_version) => {
                        let packet = login::serverbound::LoginStart::deserialize(&mut de)?;

                        Ok(PacketKind::LoginStart(packet.name))
                    }
                    i => return Err(PacketError::InvalidPacketId(i, state)),
                },
                State::Play => todo!(),
                State::Status => match id {
                    i if i == status::serverbound::Request::id(protocol_version) => {
                        Ok(PacketKind::StatusRequest)
                    }
                    i if i == status::serverbound::Ping::id(protocol_version) => {
                        let packet = status::serverbound::Ping::deserialize(&mut de)?;

                        Ok(PacketKind::StatusPing(packet.payload))
                    }
                    i => return Err(PacketError::InvalidPacketId(i, state)),
                },
            }
        } else {
            todo!("add clientbound support");
        }
    }
}

impl Display for PacketKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PacketKind::*;

        match self {
            Handshake {
                protocol_version: _,
                host: _,
                port: _,
                next_state: _,
            } => write!(f, "Handshake"),
            LoginStart(_) => write!(f, "LoginStart"),
            LoginSuccess(_, _) => write!(f, "LoginSuccess"),
            StatusRequest => write!(f, "StatusRequest"),
            StatusResponse(_) => write!(f, "StatusResponse"),
            StatusPing(_) => write!(f, "StatusPing"),
            StatusPong(_) => write!(f, "StatusPong"),
            JoinGame {
                entity_id: _,
                is_hardcore: _,
                gamemode: _,
                previous_gamemode: _,
                world_count: _,
                world_names: _,
                dimension_codec: _,
                dimension: _,
                dimension_47: _,
                difficulty: _,
                world_name: _,
                hashed_seed: _,
                max_players: _,
                level_type: _,
                view_distance: _,
                reduced_debug_info: _,
                enable_respawn_screen: _,
                is_debug: _,
                is_flat: _,
            } => write!(f, "JoinGame"),
        }
    }
}
