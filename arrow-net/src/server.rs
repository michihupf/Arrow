use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::player::Player;

pub static SERVER: RwLock<Server> = RwLock::const_new(Server::new());

pub struct Server {
    players: Vec<Arc<RwLock<Player>>>,
}

impl Server {
    pub const fn new() -> Self {
        Self { players: vec![] }
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
