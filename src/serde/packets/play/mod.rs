use serde::Serialize;

use crate::server::Server;
use crate::world::dimension::*;
use crate::{net::client::Client, serde::types::Varint};
use crate::{net::error::NetError, serde::packets::play::clientbound::SpawnEntity};

use nbt::{Blob, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use self::clientbound::{Gamemode, JoinGame};

pub mod clientbound;
pub mod serverbound;

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

pub async fn join_game(client: &mut Client, entity_id: i32) -> Result<(), NetError> {
    let mut dimensions = HashMap::new();
    let mut biomes = HashMap::new();

    dimensions.insert(
        "minecraft:overworld".to_string(),
        DimensionType::new(
            Value::Byte(0),
            Value::Byte(1),
            Value::Float(0f32),
            None,
            Value::String("minecraft:infiniburn_overworld".to_string()),
            Value::Byte(0),
            Value::Byte(1),
            Value::Byte(1),
            Value::String("minecraft:overworld".to_string()),
            Value::Byte(1),
            Value::Int(256),
            Value::Double(1.0),
            Value::Byte(0),
            Value::Byte(0),
        ),
    );

    biomes.insert(
        "minecraft:plains".to_string(),
        BiomeProperties::new(
            Value::String("rain".to_string()),
            Value::Float(0.125),
            Value::Float(0.8),
            Value::Float(0.05),
            Value::Float(0.4),
            Value::String("plains".to_string()),
            None,
            BiomeEffects::new(
                Value::Int(7907327),
                Value::Int(329011),
                Value::Int(12638463),
                Value::Int(4159204),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ),
            None,
        ),
    );

    let dimension_codec = DimensionCodec::new(dimensions, biomes);

    let dimension = 
        DimensionType::new(
            Value::Byte(0),
            Value::Byte(1),
            Value::Float(0f32),
            None,
            Value::String("minecraft:infiniburn_overworld".to_string()),
            Value::Byte(0),
            Value::Byte(1),
            Value::Byte(1),
            Value::String("minecraft:overworld".to_string()),
            Value::Byte(1),
            Value::Int(256),
            Value::Double(1.0),
            Value::Byte(0),
            Value::Byte(0),
        );


    let mut dimension_codec_bytes = vec![];
    let mut dimension_bytes = vec![];
   
    let mut blob = Blob::new();
    blob.insert("minecraft:dimension_type", dimension_codec.dimension_type.clone()).unwrap();
    blob.insert("minecraft:worldgen/biome", dimension_codec.worldgen_biome.clone()).unwrap();

    let mut codec_encoder = nbt::ser::Encoder::new(&mut dimension_codec_bytes, None);
    blob.serialize(&mut codec_encoder).map_err(|e| NetError::SerializeError(format!("Failed serializing dimension code: {}", e)))?;

    let mut encoder = nbt::ser::Encoder::new(&mut dimension_bytes, None);
    Value::Compound(dimension.clone().as_hash_map()).serialize(&mut encoder).map_err(|e| NetError::SerializeError(format!("Failed serializing dimension code: {}", e)))?;

    let join_game = JoinGame {
        entity_id,
        hardcore: false,
        gamemode: Gamemode::Creative as u8,
        prev_gamemode: Gamemode::Survival as i8,
        world_names: vec!["world".to_string()],
        world_name: "world".to_string(),
        dimension_codec: dimension_codec_bytes.as_slice(),
        dimension: dimension_bytes.as_slice(),
        hashed_seed: 0,
        max_players: Varint(42),
        view_distance: Varint(10),
        reduced_debug_info: false,
        enable_respawn_screen: true,
        debug: true,
        flat: false,
    };

    client.send_packet(0x24, join_game).await
}
