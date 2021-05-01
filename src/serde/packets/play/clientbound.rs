use uuid::Uuid;
use crate::serde::types::Varint;

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