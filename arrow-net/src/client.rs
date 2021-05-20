use arrow_codec::codec::McCodec;
use arrow_protocol::packets::PacketKind;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use crate::error::NetError;

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
    pub async fn connect(&mut self) -> Result<(), NetError> {
        match self.next_packet().await? {
            PacketKind::Handshake {
                protocol_version: _,
                host: _,
                port: _,
                next_state,
            } => match next_state {
                1 => todo!("status"),
                i => Err(NetError::InvalidStatus(i)),
            },
            _ => Err(NetError::UnexpectedPacket),
        }
    }

    async fn next_packet(&mut self) -> Result<PacketKind, NetError> {
        Ok(self.framed.next().await.ok_or(NetError::UnexpectedEof)??)
    }
}
