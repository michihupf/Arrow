use uuid::Uuid;
use serde::Serialize;
use crate::serde::types::Varint;

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