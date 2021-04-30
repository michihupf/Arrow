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
    config: Arc<Config>,
}

impl NetHandler {
    pub async fn new(config: Arc<Config>, server: Arc<Mutex<Server>>) -> Self {
        Self {
            listener: TcpListener::bind(format!("0.0.0.0:{}", config.port()))
                .await
                .unwrap(),
            server,
            config,
        }
    }

    pub async fn recv_loop(&mut self) -> Result<(), NetError> {
        debug!("starting loop");

        loop {
            let (socket, _) = self.listener.accept().await.unwrap();
            debug!("Client is connecting");
            let server = self.server.clone();
            let config = self.config.clone();

            tokio::spawn(async move {
                let client = Client::new(socket);

                let handshake = client.handshake(config, server.clone()).await;

                if let Ok(Some(mut player)) = handshake {
                    if let Err(e) = player.client_mut().play_recv_loop(server.clone()).await {
                        log::error!("Player loop stopped: {}", e);
                    }
                    server.lock().await.add_player(player);
                }
            });
        }
    }
}
