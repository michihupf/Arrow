use std::{io::Cursor, sync::Arc};

use log::{info};
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};
use uuid::Uuid;

use crate::{net::error::NetError, serde::Serializer, server::Server};
use crate::serde::{Deserializer, read_varint, varint_bytes};
use crate::serde::{handshake::serverbound::Handshake, login::{serverbound::LoginStart}};
use crate::server::player::Player;

/// a client
pub struct Client {
    /// tcp connection of client
    stream: TcpStream,
}

impl Client {
    /// takes a `tokio::net:TcpStream` and returns a `Client`
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    /// called when a client connects
    pub async fn handshake(mut self) -> Result<Option<Player>, NetError> {
        let handshake: Handshake = self.next_packet(0).await?;

        match handshake.next_state.0 {
            1 => self.status().await,
            2 => return self.login().await,
            _ => unreachable!("Invalid next status")
        }

        Ok(None)
    }

    /// expects that handshake has been handled
    pub async fn login(mut self) -> Result<Option<Player>, NetError> {
        let login_start: LoginStart = self.next_packet(0).await?;

        let uuid = Uuid::new_v3(&Uuid::NAMESPACE_OID, login_start.name.as_bytes());

        info!("Player {} with uuid {} logged in", login_start.name, uuid);

        Ok(Some(Player::new(self, uuid)))
    }

    /// expects that handshake has been handled
    pub async fn status(self) {
        loop {}
    }

    /// deserialize next packet
    pub async fn next_packet<'d, P>(&mut self, id: i32) -> Result<P, NetError> where P: Deserialize<'d> {
        let buf = &mut [0; 10];
        self.stream.peek(buf).await.map_err(|e| NetError::ReadError(format!("{}", e)))?;

        let reader = &mut Cursor::new(buf);

        let len = read_varint(reader);
        let packet_id = read_varint(reader);

        if packet_id.0 != id {
            unreachable!("got id {} expected id {}", packet_id.0, id)
        }

        self.stream.read_exact(vec![0; len.len()].as_mut_slice()).await.map_err(|e| NetError::ReadError(format!("{}", e)))?;

        let mut buf = vec![0; len.0 as usize];

        self.stream.read_exact(buf.as_mut_slice()).await.map_err(|e| NetError::ReadError(format!("{}", e)))?;

        buf = buf[packet_id.len()..].to_vec();

        P::deserialize(&mut Deserializer::new(buf)).map_err(|e| NetError::DeserializeError(format!("{}", e)))
    }

    pub async fn send_packet<P>(&mut self, id: i32, packet: P) -> Result<(), NetError> where P: Serialize {
        let mut ser = Serializer {output: vec![] };
        packet.serialize(&mut ser).map_err(|e| NetError::SerializeError(format!("{}", e)))?;
      
        let mut output = ser.output;

        let mut bytes = varint_bytes(id);
        bytes.append(&mut varint_bytes(output.len() as i32));
        bytes.append(&mut output);

        self.stream.try_write(bytes.as_slice()).map_err(|e| NetError::WriteError(format!("{}", e)))?;

        Ok(())
    }

    pub async fn play_recv_loop(&mut self, _server: Arc<Mutex<Server>>) {
        loop { 
        }
    }
}
