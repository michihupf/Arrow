#[deny(missing_docs)]

/// A client of the Minecraft protocol.
pub mod client;
/// The error for errors when binding, accepting, reading and writing.
pub mod error;
/// A player in the play state.
pub mod player;
/// The server that stores all players.
pub mod server;

use log::{error, info};
use tokio::net::TcpListener;

use error::{NetError, Result};

/// Starts the server by taking the host name and the port as arguments.
pub async fn start_server(host: &str, port: u16) -> Result<()> {
    let listener = TcpListener::bind((host, port))
        .await
        .map_err(|e| NetError::ServerBindError(format!("{}", e)))?;

    info!("Started server on {}:{}.", host, port);

    loop {
        let (socket, ip) = listener
            .accept()
            .await
            .map_err(|e| NetError::ClientAcceptError(format!("{}", e)))?;

        info!(
            "Client with ip {} and port {} connected.",
            ip.ip(),
            ip.port()
        );

        tokio::spawn(async move {
            let mut buf = [0];

            socket.peek(&mut buf).await.unwrap();

            match buf[0] {
                0xfe => {
                    error!("Implement legacy server ping.");
                    return;
                }
                _ => {}
            }

            let client = client::Client::new(socket);
            client.connect().await;
        });
    }
}
