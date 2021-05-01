use std::{io::Cursor, sync::Arc};

use log::{debug, info};
use serde::{Deserialize, Serialize};
use status::legacy::serverbound::PluginMessage;
use tokio::{io::AsyncReadExt, net::TcpStream, sync::Mutex};
use uuid::Uuid;

use crate::serde::{read_varint, status, varint_bytes, Deserializer};
use crate::server::player::Player;
use crate::{
    config::{Config, Description},
    serde::{
        handshake::serverbound::Handshake,
        login::{clientbound::LoginSuccess, serverbound::LoginStart},
    },
};
use crate::{net::error::NetError, serde::Serializer, server::Server};

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
    pub async fn handshake(
        mut self,
        config: Arc<Config>,
        server: Arc<Mutex<Server>>,
    ) -> Result<Option<Player>, NetError> {
        let byte = &mut [0];
        self.stream
            .peek(byte)
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        if byte[0] == 0xfe {
            self.legacy_ping(config).await?;
            return Ok(None);
        }

        let handshake: Handshake = self.next_packet(0).await?;

        match handshake.next_state.0 {
            1 => return self.status(config).await.map(|_| None),
            2 => return self.login(server).await,
            _ => unreachable!("Invalid next status"),
        }
    }

    /// expects that handshake has been handled
    pub async fn login(mut self, server: Arc<Mutex<Server>>) -> Result<Option<Player>, NetError> {
        let login_start: LoginStart = self.next_packet(0).await?;

        let uuid = Uuid::new_v3(&Uuid::NAMESPACE_OID, login_start.name.as_bytes());

        if server.lock().await.has_uuid(&uuid).await {
            info!(
                "Client with address {} tried to login as online player with uuid {}",
                self.stream
                    .local_addr()
                    .map_err(|e| NetError::CantGetAddressError(format!("{}", e)))?,
                uuid
            );
            todo!("Add disconnect packet");
        }

        self.send_packet(
            0x02,
            LoginSuccess {
                uuid: &uuid,
                username: login_start.name.clone(),
            },
        )
        .await?;

        info!("Player {} with uuid {} logged in", login_start.name, uuid);

        Ok(Some(Player::new(self, uuid)))
    }

    /// expects that handshake has been handled
    pub async fn status(mut self, config: Arc<Config>) -> Result<(), NetError> {
        debug!("Status requested");
        let _request: status::serverbound::Request = self.next_packet(0).await?;

        let response = serde_json::to_string(&Response::new(&config))
            .map_err(|e| NetError::SerializeError(format!("{}", e)))?;

        self.send_packet(0x0, status::clientbound::Response { response })
            .await?;

        let mut out = [0; 10];

        self.stream
            .read_exact(&mut out)
            .await
            .map_err(|e| NetError::ReadError(format!("Failed reading ping packet: {}", e)))?;

        self.stream
            .try_write(&mut out)
            .map_err(|e| NetError::WriteError(format!("Failed writing ping packet: {}", e)))?;

        Ok(())
    }

    async fn legacy_ping(mut self, config: Arc<Config>) -> Result<(), NetError> {
        let pack_id = self
            .stream
            .read_u8()
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        if pack_id != 0xfe {
            unreachable!("Expected 0xfe, got {}", pack_id);
        }

        let mut buf = vec![];

        self.stream
            .read(buf.as_mut_slice())
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        let _ = PluginMessage::deserialize(&mut Deserializer::new(buf))
            .map_err(|e| NetError::DeserializeError(format!("{}", e)))?;

        let mut data = vec![
            0x00, 0xa7, 0x00, 0x31, 0x00, 0x00, 0x00, 0x31, 0x00, 0x32, 0x00, 0x37, 0x00, 0x00,
            0x00, 0x31, 0x00, 0x2e, 0x00, 0x38, 0x00, 0x2e, 0x00, 0x39, 0x00, 0x00,
        ];

        data.append(
            &mut config
                .description()
                .text
                .encode_utf16()
                .map(|v| v.to_be_bytes())
                .collect::<Vec<[u8; 2]>>()
                .concat(),
        );
        data.append(&mut vec![0x00, 0x30, 0x00, 0x00, 0x00, 0x34, 0x00, 0x32]);

        self.stream
            .try_write(data.as_slice())
            .map_err(|e| NetError::SerializeError(format!("{}", e)))?;

        Ok(())
    }

    pub async fn play_recv_loop(&mut self, _server: Arc<Mutex<Server>>) -> Result<(), NetError> {
        loop {
            self.stream
                .readable()
                .await
                .map_err(|e| NetError::ReadError(format!("{}", e)))?;

            match self.next_packet_id().await? {
                _ => {}
//                id => debug!("Unimplemented packet with id {}", id),
            }
        }
    }

    async fn next_packet_id(&mut self) -> Result<i32, NetError> {
        let mut buf = [0; 10];
        self.stream
            .peek(&mut buf)
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        let reader = &mut Cursor::new(&mut buf);

        let _ = read_varint(reader);
        let packet_id = read_varint(reader);

        Ok(packet_id.0)
    }

    /// deserialize next packet
    pub async fn next_packet<'d, P>(&mut self, id: i32) -> Result<P, NetError>
    where
        P: Deserialize<'d>,
    {
        let mut buf = [0; 10];
        self.stream
            .peek(&mut buf)
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        let reader = &mut Cursor::new(&mut buf);

        let len = read_varint(reader);
        let packet_id = read_varint(reader);

        if packet_id.0 != id {
            unreachable!("got id {} expected id {} data: {:?}", packet_id.0, id, buf)
        }

        self.stream
            .read_exact(vec![0; len.len()].as_mut_slice())
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        let mut buf = vec![0; len.0 as usize];

        self.stream
            .read_exact(buf.as_mut_slice())
            .await
            .map_err(|e| NetError::ReadError(format!("{}", e)))?;

        buf = buf[packet_id.len()..].to_vec();

        P::deserialize(&mut Deserializer::new(buf))
            .map_err(|e| NetError::DeserializeError(format!("{}", e)))
    }

    pub async fn send_packet<P>(&mut self, id: i32, packet: P) -> Result<(), NetError>
    where
        P: Serialize,
    {
        let mut ser = Serializer { output: vec![] };
        packet
            .serialize(&mut ser)
            .map_err(|e| NetError::SerializeError(format!("{}", e)))?;

        let mut output = ser.output;

        let mut bytes = varint_bytes(output.len() as i32);
        bytes.append(&mut varint_bytes(id));
        bytes.append(&mut output);

        self.stream
            .try_write(bytes.as_slice())
            .map_err(|e| NetError::WriteError(format!("{}", e)))?;

        Ok(())
    }
}

#[derive(Serialize)]
struct Response<'a> {
    version: Version<'a>,
    players: Players,
    description: &'a Description,
}

#[derive(Serialize)]
struct Version<'a> {
    name: &'a String,
    protocol: i32,
}

#[derive(Serialize)]
struct Players {
    max: usize,
    online: usize,
}

impl<'a> Response<'a> {
    pub fn new(config: &'a Arc<Config>) -> Self {
        Self {
            version: Version {
                name: config.version_name(),
                protocol: *config.protocol_version().start(),
            },
            players: Players { max: 42, online: 0 },
            description: &config.description(),
        }
    }
}
