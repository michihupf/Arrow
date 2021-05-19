pub mod error;

use arrow_codec::codec::McCodec;
use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpListener};
use tokio_util::codec::Decoder;

use error::{NetError, Result};

pub async fn start_server(host: &str, port: u16) -> Result<()> {
    let listener = TcpListener::bind((host, port))
        .await
        .map_err(|e| NetError::ServerBindError(format!("{}", e)))?;

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .map_err(|e| NetError::ClientAcceptError(format!("{}", e)))?;

        let mut buf = BytesMut::new();
        let mut decoder = McCodec;

        let handshake = loop {
            socket.read_buf(&mut buf).await.unwrap();

            if let Some(handshake) = decoder.decode(&mut buf).unwrap() {
                break handshake;
            }
        };

        dbg!(handshake.len(), handshake.id(), handshake.data());
    }
}
