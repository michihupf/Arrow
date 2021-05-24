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
        Self {
            players: vec![],
            max_player_count,
        }
    }

    pub fn add_player(&mut self, player: Arc<RwLock<Player>>) {
        let player_clone = player.clone();
        tokio::spawn(async move {
            player_clone.write().await.client_mut().join().await;
        });
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

    pub fn get_max_online_player_count(&self) -> i32 {
        self.max_player_count
    }

    pub fn get_online_player_count(&self) -> i32 {
        self.players.len() as i32
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
