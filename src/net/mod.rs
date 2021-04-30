pub mod client;
pub mod error;

use std::sync::Arc;

use log::{debug, info};
use tokio::{net::TcpListener, sync::Mutex};

use crate::{config::Config, server::Server};
use client::Client;
use error::NetError;

pub struct NetHandler {
    listener: TcpListener,
    server: Arc<Mutex<Server>>,
}

impl NetHandler {
    pub async fn new(config: &Config, server: Arc<Mutex<Server>>) -> Self {
        Self {
            listener: TcpListener::bind(format!("0.0.0.0:{}", config.port()))
                .await
                .unwrap(),
            server,
        }
    }

    pub async fn recv_loop(&mut self) -> Result<(), NetError> {
        debug!("starting loop");

        loop {
            let (socket, _) = self.listener.accept().await.unwrap();

            let server = self.server.clone();

            tokio::spawn(async move {
                let client = Client::new(socket);

                let handshake = client.handshake().await;

                if let Ok(Some(mut player)) = handshake {
                    player.client_mut().play_recv_loop(server.clone()).await;
                    server.lock().await.add_player(player);
                }
            });
        }
    }
}
