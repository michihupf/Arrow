pub mod error;
pub mod player;

use uuid::Uuid;

use crate::net::error::NetError;
use self::player::Player;

pub struct Server {
    // world_data: Vec<Vec<chunk::Chunk>>,
    players: Vec<Player>,
}

impl Server {
    pub async fn new() -> Self {
        Self { players: vec![] }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub async fn broadcast_packet<P>(&mut self, id: i32, packet: P) -> Result<(), NetError> where P: serde::Serialize {
        let packet = &packet;

        for player in self.players.as_mut_slice() {
            player.client_mut().send_packet(id, packet).await?;
        }

        Ok(())
    }


    pub async fn broadcast_packet_exclude<P>(&mut self, id: i32, packet: P, exclude: Vec<&Uuid>) -> Result<(), NetError> where P: serde::Serialize {
        let packet = &packet;

        for player in self.players.as_mut_slice() {
            if !exclude.contains(&player.uuid()) {
                player.client_mut().send_packet(id, packet).await?;
            }
        }

        Ok(())
    }
}
