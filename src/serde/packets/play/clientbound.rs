use crate::serde::types::Varint;
use serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(Serialize)]
pub struct SpawnEntity {
    pub entity_id: Varint,
    pub object_uuid: Uuid,
    pub entity_type: Varint,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub data: i32,
    pub vel_x: i16,
    pub vel_y: i16,
    pub vel_z: i16,
}

#[derive(Serialize)]
pub struct SpawnExperienceOrb {
    pub entity_id: Varint,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub count: i16,
}

#[derive(Serialize)]
pub struct SpawnLivingEntity {
    pub entity_id: Varint,
    pub entity_uuid: Uuid,
    pub entity_type: Varint,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub head_pitch: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,
}

#[derive(Serialize)]
pub struct JoinGame<'a> {
    pub entity_id: i32,
    pub hardcore: bool,
    pub gamemode: u8,
    pub prev_gamemode: i8,
    pub world_names: Vec<String>,
    #[serde(serialize_with = "serialize_bytes")]
    pub dimension_codec: &'a [u8],
    #[serde(serialize_with = "serialize_bytes")]
    pub dimension: &'a [u8],
    pub world_name: String,
    pub hashed_seed: u64,
    pub max_players: Varint,
    pub view_distance: Varint,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub debug: bool,
    pub flat: bool,
}

pub fn serialize_bytes<S>(bytes: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_bytes(bytes)
}

pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}
