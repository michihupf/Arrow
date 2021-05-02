use crate::serde::types::Varint;
use crate::server::Server;
use crate::net::client::Client;
use crate::net::error::NetError;
use crate::serde::packets::play::clientbound::{
    SpawnEntity, PluginMessage, ServerDifficulty, PlayerAbilities
};

use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub mod clientbound;
pub mod serverbound;


pub async fn send_plugin_message_to_client(
    mut client: Client,
    channel: String,
    data: Vec<i8>
) -> Result<(), NetError> {
    let plugin_message = PluginMessage {
        channel,
        data
    };

    client
        .send_packet(0x17, plugin_message)
        .await
}

pub async fn send_plugin_message_to_all_clients(
    server: Arc<Mutex<Server>>,
    channel: String,
    data: Vec<i8>
) -> Result<(), NetError> {
    let plugin_message = PluginMessage {
        channel,
        data
    };

    server
        .lock()
        .await
        .broadcast_packet(0x17, plugin_message)
        .await
}

pub async fn set_server_difficulty(
    server: Arc<Mutex<Server>>,
    difficulty: u8,
    difficulty_locked: bool
) -> Result<(), NetError> {
    let difficulty_packet = ServerDifficulty {
        difficulty,
        difficulty_locked
    };

    server
        .lock()
        .await
        .broadcast_packet(0x0D, difficulty_packet)
        .await
}

/// method for spawning an Entity
pub async fn spawn_entity(
    server: Arc<Mutex<Server>>,
    entity_id: Varint,
    object_uuid: Uuid,
    entity_type: Varint,
    position: (f32, f32, f32),
    pitch: f32,
    yaw: f32,
    data: i32,
    velocity: (i16, i16, i16),
) -> Result<(), NetError> {
    let spawn_entity = SpawnEntity {
        entity_id,
        object_uuid,
        entity_type,
        x: position.0,
        y: position.1,
        z: position.2,
        pitch,
        yaw,
        data,
        vel_x: velocity.0,
        vel_y: velocity.1,
        vel_z: velocity.2,
    };

    server
        .lock()
        .await
        .broadcast_packet(0x00, spawn_entity)
        .await
}

pub async fn spawn_experience_orb(
    server: Arc<Mutex<Server>>,
    entity_id: Varint,
    position: (f32, f32, f32),
    count: i16,
) {
    todo!("Implement Experience Orb Spawning");
}
