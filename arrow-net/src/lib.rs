#[deny(missing_docs)]

/// A client of the Minecraft protocol.
pub mod client;
/// The error for errors when binding, accepting, reading and writing.
pub mod error;

use tokio::net::TcpListener;

use error::{NetError, Result};

/// Starts the server by taking the host name and the port as arguments.
pub async fn start_server(host: &str, port: u16) -> Result<()> {
    let listener = TcpListener::bind((host, port))
        .await
        .map_err(|e| NetError::ServerBindError(format!("{}", e)))?;

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .map_err(|e| NetError::ClientAcceptError(format!("{}", e)))?;

        tokio::spawn(async move {
            let mut buf = [0];

            socket.peek(&mut buf).await.unwrap();

            match buf[0] {
                0xfe => todo!("Implement legacy server ping."),
                _ => {}
            }

            let mut client = client::Client::new(socket);

            if client.connect().await.is_err() {
                return;
            }
        });
    }
}
