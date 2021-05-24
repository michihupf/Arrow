use std::sync::Arc;

use arrow_codec::codec::McCodec;
use arrow_protocol::{packets::{PacketKind, common::status, types::{self, Gamemode}, version_specific::{self, types::v754::{
            BiomeEffects, BiomeProperties, BiomeRegistry, BiomeRegistryEntry, DimensionCodec,
            DimensionRegistry, DimensionRegistryEntry, DimensionType,
        }}}, serde::varint::VarInt};
use futures::{SinkExt, StreamExt, TryStreamExt};
use log::{error, info};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_util::codec::Framed;
use uuid::Uuid;

use crate::error::NetError;
use crate::player::Player;
use crate::server::SERVER;

macro_rules! next_packet {
    ($self:ident) => {
        match $self.next_packet().await {
            Ok(p) => p,
            Err(e) => {
                error!("Failed reading next packet: {}", e);
                return;
            }
        }
    };
}

macro_rules! send_packet {
    ($self:ident $packet:expr) => {
        match $self.framed.send($packet).await {
            Ok(p) => p,
            Err(e) => {
                error!("Failed sending packet: {}", e.0);
                return;
            }
        }
    };
}

/// A client that connected to the server.
pub struct Client {
    framed: Framed<TcpStream, McCodec>,
}

impl Client {
    /// Creates a new client using a [`tokio::net::TcpStream`].
    pub fn new(stream: TcpStream) -> Self {
        Self {
            framed: Framed::new(stream, McCodec::new(true)),
        }
    }

    /// Handles the handshake packet and select the right state to continue in.
    pub async fn connect(mut self) {
        match next_packet!(self) {
            PacketKind::Handshake {
                protocol_version: _,
                host: _,
                port: _,
                next_state,
            } => match next_state {
                1 => return self.status().await,
                2 => return self.login().await,
                i => error!("Invalid next state {}.", i),
            },
            p => error!("Unexpected packet {}, expected Handshake.", p),
        };
    }

    /// Returns the protocol version for this client
    pub fn get_protocol_version(&self) -> i32 {
        self.framed.codec().get_protocol_version()
    }

    pub(crate) async fn recv(&mut self) {
        loop {
            match self.framed.try_next().await {
                Ok(Some(p)) => match p {
                    PacketKind::Handshake {
                        protocol_version: _,
                        host: _,
                        port: _,
                        next_state: _,
                    }
                    | PacketKind::LoginStart(_)
                    | PacketKind::LoginSuccess(_, _)
                    | PacketKind::StatusRequest
                    | PacketKind::StatusResponse(_)
                    | PacketKind::StatusPing(_)
                    | PacketKind::StatusPong(_)
                    | PacketKind::JoinGame {
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
                    } => {
                        error!("Received packet from other protocol state: {}.", p);
                        return;
                    }
                },
                Ok(None) => return,
                Err(e) => {
                    error!("Failed reading next packet: {}", e.0);
                    return;
                }
            }
        }
    }

    async fn status(mut self) {
        match next_packet!(self) {
            PacketKind::StatusRequest => {}
            p => {
                error!("Unexpected packet {}, expected StatusRequest.", p);
                return;
            }
        };
        let test_img = "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";

        let response_data = status::ResponseData {
            version: status::VersionData {
                name: String::from("1.8 - 1.16"),
                protocol: self.get_protocol_version(),
            },
            players: status::PlayerData {
                max: SERVER.read().await.get_max_online_player_count(),
                online: SERVER.read().await.get_online_player_count(),
                sample: Vec::new(),
            },
            description: status::DescriptionData {
                text: String::from("Hello world"),
            },
            favicon: String::from(format!("data:image/png;base64,{}", test_img)),
        };

        send_packet!(self PacketKind::StatusResponse(response_data));

        let ping_data = match next_packet!(self) {
            PacketKind::StatusPing(data) => data,
            p => {
                error!("Unexpected packet {}, expected StatusPing.", p);
                return;
            }
        };

        send_packet!(self PacketKind::StatusPong(ping_data));
    }

    async fn login(mut self) {
        let name = match next_packet!(self) {
            PacketKind::LoginStart(n) => n,
            p => {
                error!("Unexpected packet {}, expected LoginStart.", p);
                return;
            }
        };

        let uuid = Uuid::new_v3(&Uuid::NAMESPACE_OID, name.as_bytes());

        if SERVER.read().await.has_uuid(&uuid).await {
            error!("Player already connected.");
        } else {
            info!("Player {} with uuid {} logged in successfully.", name, uuid);
            send_packet!(self PacketKind::LoginSuccess(uuid.clone(), name.clone()));
            SERVER
                .write()
                .await
                .add_player(Arc::new(RwLock::new(Player::new(uuid, name, self))));
        }
    }

    ///
    pub async fn join(&mut self) {
        let dimension = DimensionType {
            piglin_safe: false,
            natural: true,
            ambient_light: 1.0,
            fixed_time: None,
            infiniburn: String::from("minecraft:infiniburn_overworld"),
            respawn_anchor_works: false,
            has_skylight: true,
            bed_works: true,
            effects: String::from("minecraft:overworld"),
            has_raids: false,
            logical_height: 255,
            coordinate_scale: 1.0,
            ultrawarm: false,
            has_ceiling: false,
        };

        let dimension_codec = DimensionCodec {
            dimension_registry: DimensionRegistry {
                dimension_type: String::from("minecraft:dimension_type"),
                value: vec![
                    DimensionRegistryEntry {
                        name: String::from("minecraft:overworld"),
                        id: 0,
                        element: dimension.clone(),
                    },
                    DimensionRegistryEntry {
                        name: String::from("minecraft:overworld_caves"),
                        id: 1,
                        element: DimensionType {
                            piglin_safe: true,
                            natural: true,
                            ambient_light: 1.0,
                            fixed_time: None,
                            infiniburn: String::from("minecraft:infiniburn_overworld"),
                            respawn_anchor_works: false,
                            has_skylight: true,
                            bed_works: true,
                            effects: String::from("minecraft:overworld"),
                            has_raids: false,
                            logical_height: 255,
                            coordinate_scale: 1.0,
                            ultrawarm: false,
                            has_ceiling: false,
                        },
                    },
                    DimensionRegistryEntry {
                        name: String::from("minecraft:the_nether"),
                        id: 2,
                        element: DimensionType {
                            piglin_safe: true,
                            natural: true,
                            ambient_light: 0.0,
                            fixed_time: None,
                            infiniburn: String::from("minecraft:infiniburn_nether"),
                            respawn_anchor_works: false,
                            has_skylight: true,
                            bed_works: true,
                            effects: String::from("minecraft:the_nether"),
                            has_raids: false,
                            logical_height: 255,
                            coordinate_scale: 1.0,
                            ultrawarm: false,
                            has_ceiling: false,
                        },
                    },
                    DimensionRegistryEntry {
                        name: String::from("minecraft:the_end"),
                        id: 3,
                        element: DimensionType {
                            piglin_safe: true,
                            natural: true,
                            ambient_light: 0.1,
                            fixed_time: None,
                            infiniburn: String::from("minecraft:infiniburn_end"),
                            respawn_anchor_works: false,
                            has_skylight: true,
                            bed_works: true,
                            effects: String::from("minecraft:the_end"),
                            has_raids: false,
                            logical_height: 255,
                            coordinate_scale: 1.0,
                            ultrawarm: false,
                            has_ceiling: false,
                        },
                    },
                ],
            },
            biome_registry: BiomeRegistry {
                biome_type: String::from("minecraft:worldgen/biome"),
                value: vec![BiomeRegistryEntry {
                    name: String::from("minecraft:plains"),
                    id: 1,
                    element: BiomeProperties {
                        precipitation: String::from("rain"),
                        depth: -1.0,
                        temperature: 0.5,
                        scale: 0.1,
                        downfall: 0.5,
                        category: String::from("plains"),
                        temperature_modifier: None,
                        effects: BiomeEffects {
                            sky_color: 8103167,
                            water_fog_color: 329011,
                            fog_color: 12638463,
                            water_color: 4159204,
                            foilage_color: None,
                            grass_color: None,
                            grass_color_modifier: None,
                            music: None,
                            ambient_sound: None,
                            additions_sound: None,
                            mood_sound: None,
                        },
                        particle: None,
                    },
                }],
            },
        };

        let packet = PacketKind::JoinGame {
            entity_id: 0,
            is_hardcore: false,
            gamemode: Gamemode::Survival,
            previous_gamemode: Gamemode::NoPreviousMode,
            world_count: VarInt(1),
            world_names: vec![String::from("world")],
            dimension_codec: dimension_codec.get_bytes(),
            dimension: dimension.get_bytes(),
            dimension_47: version_specific::types::v47::Dimension::Overworld,
            difficulty: types::Difficulty::Peaceful,
            world_name: String::from("world"),
            // SERVER.read().await.get_max_online_player_count()
            max_players: SERVER.read().await.get_max_online_player_count(),
            level_type: types::LevelType::Default,
            view_distance: VarInt(8),
            hashed_seed: 0x6B51D431DF5D7F14,
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: true,
            is_flat: false,
        };

        send_packet!(self packet);
    }

    async fn next_packet(&mut self) -> Result<PacketKind, NetError> {
        Ok(self.framed.next().await.ok_or(NetError::UnexpectedEof)??)
    }
}
