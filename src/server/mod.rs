pub mod error;
pub mod player;

use self::player::Player;

pub struct Server {
    // world_data: Vec<Vec<chunk::Chunk>>,
    players: Vec<Player>,
}

impl Server {
    pub async fn new() -> Self {
        Self {
            players: vec![]
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}
