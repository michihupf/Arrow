use std::sync::Arc;

use arrow_codec::codec::McCodec;
use arrow_protocol::packets::PacketKind;
use futures::{SinkExt, StreamExt, TryStreamExt};
use log::{error, info};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_util::codec::Framed;
use uuid::Uuid;

use crate::error::NetError;
use crate::player::Player;
use crate::server::SERVER;

macro_rules! next_packet {
    ($self:ident) => {
        match $self.next_packet().await {
            Ok(p) => p,
            Err(e) => {
                error!("Failed reading next packet: {}", e);
                return;
            }
        }
    };
}

macro_rules! send_packet {
    ($self:ident $packet:expr) => {
        match $self.framed.send($packet).await {
            Ok(p) => p,
            Err(e) => {
                error!("Failed sending packet: {}", e.0);
                return;
            }
        }
    };
}

/// A client that connected to the server.
pub struct Client {
    framed: Framed<TcpStream, McCodec>,
}

impl Client {
    /// Creates a new client using a [`tokio::net::TcpStream`].
    pub fn new(stream: TcpStream) -> Self {
        Self {
            framed: Framed::new(stream, McCodec::new(true)),
        }
    }

    /// Handles the handshake packet and select the right state to continue in.
    pub async fn connect(mut self) {
        match next_packet!(self) {
            PacketKind::Handshake {
                protocol_version: _,
                host: _,
                port: _,
                next_state,
            } => match next_state {
                1 => todo!("status"),
                2 => return self.login().await,
                i => error!("Invalid next state {}.", i),
            },
            p => error!("Unexpected packet {}, expected Handshake.", p),
        };
    }

    pub(crate) async fn recv(&mut self) {
        loop {
            match self.framed.try_next().await {
                Ok(Some(p)) => match p {
                    PacketKind::Handshake {
                        protocol_version: _,
                        host: _,
                        port: _,
                        next_state: _,
                    }
                    | PacketKind::LoginStart(_)
                    | PacketKind::LoginSuccess(_, _) => {
                        error!("Received packet from other protocol state: {}.", p);
                        return;
                    }
                },
                Ok(None) => return,
                Err(e) => {
                    error!("Failed reading next packet: {}", e.0);
                    return;
                }
            }
        }
    }

    async fn login(mut self) {
        let name = match next_packet!(self) {
            PacketKind::LoginStart(n) => n,
            p => {
                error!("Unexpected packet {}, expected LoginStart.", p);
                return;
            }
        };

        let uuid = Uuid::new_v3(&Uuid::NAMESPACE_OID, name.as_bytes());

        send_packet!(self PacketKind::LoginSuccess(uuid.clone(), name.clone()));

        if SERVER.read().await.has_uuid(&uuid).await {
            error!("Player already connected.");
        } else {
            info!("Player {} with uuid {} logged in successfully.", name, uuid);

            SERVER
                .write()
                .await
                .add_player(Arc::new(RwLock::new(Player::new(uuid, name, self))));
        }
    }

    async fn next_packet(&mut self) -> Result<PacketKind, NetError> {
        Ok(self.framed.next().await.ok_or(NetError::UnexpectedEof)??)
    }
}
