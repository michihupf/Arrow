use serde::Serialize;

use crate::server::Server;
use crate::{net::client::Client, serde::types::Varint};
use crate::net::error::NetError;
use crate::serde::packets::play::clientbound::{
    SpawnEntity, PluginMessage, ServerDifficulty, Difficulty, DeclareRecipes
};
use crate::minecraft::recipe::Recipe;

use log::debug;
use nbt::{Blob, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use self::clientbound::{Gamemode, JoinGame};

pub mod clientbound;
pub mod serverbound;

/// * send plugin message to a specific `client` via a specific `plugin_channel`
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

/// * broadcast plugin message to `all clients` on the server via a specific `plugin channel`
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

/// * set server difficulty
pub async fn set_server_difficulty(
    server: Arc<Mutex<Server>>,
    difficulty: Difficulty,
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

/// * method for spawning an Entity
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

pub async fn declare_recipes(
    mut client: Client,
    num_recipes: Varint,
    recipes: Vec<Recipe>
) -> Result<(), NetError> {
    let declare_recipes = DeclareRecipes {
        num_recipes,
        recipes
    };

    client
        .send_packet(0x5A, declare_recipes)
        .await
}

pub async fn spawn_experience_orb(
    server: Arc<Mutex<Server>>,
    entity_id: Varint,
    position: (f32, f32, f32),
    count: i16,
) {
    // TODO: IMPLEMENT EXPERIENCE ORB SPAWING
    todo!("Implement Experience Orb Spawning");
}

pub async fn join_game(client: &mut Client, entity_id: i32) -> Result<(), NetError> {
    let mut dimension_codec = Blob::new();

    let mut dimension_type = HashMap::new();
    let mut dimension_type_registry = HashMap::new();
    let mut element = HashMap::new();

    element.insert(String::from("piglin_safe"), Value::Byte(0));
    element.insert(String::from("natural"), Value::Byte(1));
    element.insert(String::from("ambient_light"), Value::Float(0f32));
    element.insert(String::from("infiniburn"), Value::String(String::from("minecraft:infiniburn_overworld")));
    element.insert(String::from("respawn_anchor_works"), Value::Byte(0));
    element.insert(String::from("has_skylight"), Value::Byte(1));
    element.insert(String::from("bed_works"), Value::Byte(1));
    element.insert(String::from("effects"), Value::String(String::from("minecraft:overworld")));
    element.insert(String::from("has_raids"), Value::Byte(1));
    element.insert(String::from("logical_height"), Value::Int(256));
    element.insert(String::from("coordinate_scale"), Value::Float(1f32));
    element.insert(String::from("ultrawarm"), Value::Byte(0));
    element.insert(String::from("has_ceiling"), Value::Byte(0));

    dimension_type_registry.insert(String::from("name"), Value::String(String::from("minecraft:overworld")));
    dimension_type_registry.insert(String::from("id"), Value::Int(0));
    dimension_type_registry.insert(String::from("element"), Value::Compound(element.clone()));

    let value = vec![Value::Compound(dimension_type_registry)];

    dimension_type.insert(String::from("type"), Value::String(String::from("minecraft:dimension_type")));
    dimension_type.insert(String::from("value"), Value::List(value));

    dimension_codec.insert("minecraft:dimension_type", Value::Compound(dimension_type)).unwrap();

    let mut biome_registry = HashMap::new();
    let mut plains_properties = HashMap::new();

    plains_properties.insert(String::from("precipitation"), Value::String(String::from("rain")));
    plains_properties.insert(String::from("depth"), Value::Float(0.125f32));
    plains_properties.insert(String::from("temperature"), Value::Float(0.8f32));
    plains_properties.insert(String::from("scale"), Value::Float(0.05f32));
    plains_properties.insert(String::from("downfall"), Value::Float(0.4f32));
    plains_properties.insert(String::from("category"), Value::String(String::from("plains")));

    let biome_value = vec![Value::Compound(plains_properties)];

    biome_registry.insert(String::from("type"), Value::String(String::from("minecraft:worldgen/biome")));
    biome_registry.insert(String::from("value"), Value::List(biome_value));

    dimension_codec.insert(String::from("worldgen/biome"), Value::Compound(biome_registry)).unwrap();

    let dimension = Value::Compound(element);

    let mut dcba: Vec<u8> = Vec::new();
    dimension_codec.to_writer(&mut dcba);

    let mut dba = Vec::new();
    dimension.to_writer(&mut dba);

    let join_game = JoinGame {
        entity_id,
        hardcore: false,
        gamemode: Gamemode::Creative as u8,
        prev_gamemode: Gamemode::Survival as i8,
        world_names: vec!["world".to_string()],
        world_name: "world".to_string(),
        dimension_codec: dcba,
        dimension: dba,
        hashed_seed: 0,
        max_players: Varint(42),
        view_distance: Varint(10),
        reduced_debug_info: false,
        enable_respawn_screen: true,
        debug: true,
        flat: false,
    };

    let response = client.send_packet(0x24, join_game).await;
    debug!("Sent a packet with id {}", 0x24);
    response
}
