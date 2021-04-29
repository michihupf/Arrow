//#![deny(missing_docs)]

pub mod world;
pub mod server;
pub mod serde;
pub mod net;
pub mod config;

use std::sync::Arc;
use net::NetHandler;
use tokio::sync::Mutex;

use config::read_config;
use fern::colors::ColoredLevelConfig;
use server::Server;

#[tokio::main]
async fn main() {
    setup_logger().unwrap();

    let config = read_config().await;

    let server = Server::new().await;

    let mut net_handler = NetHandler::new(&config, Arc::new(Mutex::new(server))).await;
    net_handler.recv_loop().await.unwrap();
}

fn setup_logger() -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::new();

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] [{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                color.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}