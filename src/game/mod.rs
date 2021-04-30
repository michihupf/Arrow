use std::time::{Duration, SystemTime, UNIX_EPOCH};

use log::debug;
use tokio::time;

const MS_PER_FRAME: u64 = 1000 / 20;

pub async fn start_game_loop() {
    debug!("starting game loop");
    loop {
        let start = SystemTime::now();
        let start = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        time::sleep(
            start + Duration::from_millis(MS_PER_FRAME)
                - SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards"),
        )
        .await;
    }
}
