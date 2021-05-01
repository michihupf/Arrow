pub mod error;
pub mod player;

use std::sync::Arc;

use uuid::Uuid;
use tokio::sync::Mutex;

use self::player::Player;
use crate::net::error::NetError;

pub struct Server {
    // world_data: Vec<Vec<chunk::Chunk>>,
    players: Vec<Arc<Mutex<Player>>>,
}

impl Server {
    pub async fn new() -> Self {
        Self { players: vec![] }
    }

    pub fn add_player(&mut self, player: Arc<Mutex<Player>>) {
        self.players.push(player);
    }

    pub async fn broadcast_packet<P>(&mut self, id: i32, packet: P) -> Result<(), NetError>
    where
        P: serde::Serialize,
    {
        let packet = &packet;

        for player in self.players.as_mut_slice() {
            player.lock().await.client_mut().send_packet(id, packet).await?;
        }

        Ok(())
    }

    pub async fn broadcast_packet_exclude<P>(
        &mut self,
        id: i32,
        packet: P,
        exclude: Vec<&Uuid>,
    ) -> Result<(), NetError>
    where
        P: serde::Serialize,
    {
        let packet = &packet;

        for player in self.players.as_mut_slice() {
            if !exclude.contains(&player.lock().await.uuid()) {
                player.lock().await.client_mut().send_packet(id, packet).await?;
            }
        }

        Ok(())
    }

    pub async fn has_uuid(&self, uuid: &Uuid) -> bool {
        for player in self.players.as_slice() {
            if player.lock().await.uuid() == uuid {
                return true;
            }
        }

        false
    }
}
