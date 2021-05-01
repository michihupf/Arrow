use crate::serde::packets::play::clientbound::SpawnEntity;
use crate::serde::types::Varint;
use crate::server::Server;

use uuid::Uuid;
use serde::Serializer;

pub mod clientbound;
pub mod serverbound;

pub async fn spawn_entity(server: Server, entity_id: Varint, object_uuid: Uuid, entity_type: Varint, position: (f32, f32, f32), pitch: f32, yaw: f32, data: i32, velocity: (i16, i16, i16)) {
    let spawn_entity = SpawnEntity {
        entity_id: entity_id,
        object_uuid: object_uuid,
        entity_type: entity_type,
        x: position.0,
        y: position.1,
        z: position.2,
        pitch: pitch,
        yaw: yaw,
        data: data,
        vel_x: velocity.0,
        vel_y: velocity.1,
        vel_z: velocity.2,
    };

    // send the packet to every player
    server.lock().await.broadcast_packet(0x00, spawn_entity);
}
