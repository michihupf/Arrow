use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::player::Player;

pub static SERVER: RwLock<Server> = RwLock::const_new(Server::new(100));

pub struct Server {
    players: Vec<Arc<RwLock<Player>>>,
    max_player_count: i32,
}

impl Server {
    pub const fn new(max_player_count: i32) -> Self {
        Self { players: vec![], max_player_count }
    }

    pub fn add_player(&mut self, player: Arc<RwLock<Player>>) {
        self.players.push(player);
    }

    pub async fn remove_player(&mut self, uuid: &Uuid) {
        let mut idx = usize::MAX;

        for (i, player) in self.players.iter().enumerate() {
            if player.read().await.uuid() == uuid {
                idx = i;
            }
        }

        if idx != usize::MAX {
            self.players.remove(idx);
        }
    }

    pub async fn get_max_online_player_count(&self) -> i32 {
        self.max_player_count
    }

    pub async fn get_online_player_count(&self) -> usize {
        self.players.len()
    }

    pub async fn has_uuid(&self, uuid: &Uuid) -> bool {
        for player in self.players.iter() {
            if player.read().await.uuid() == uuid {
                return true;
            }
        }

        return false;
    }

    pub async fn recv(&self) {
        for player in self.players.iter() {
            let player = player.clone();
            tokio::spawn(async move {
                let mut player = player.write().await;

                player.recv().await;
            });
        }
    }
}
