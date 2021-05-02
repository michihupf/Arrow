//#![deny(missing_docs)]

pub mod config;
pub mod net;
pub mod serde;
pub mod server;
pub mod world;
pub mod minecraft;

use log::debug;
use net::NetHandler;
use std::sync::Arc;
use tokio::sync::Mutex;

use config::read_config;
use fern::colors::ColoredLevelConfig;
use server::Server;

#[tokio::main]
async fn main() {
    setup_logger().unwrap();

    let config = Arc::new(read_config().await);

    let server = Server::new().await;

    let mut net_handler = NetHandler::new(config, Arc::new(Mutex::new(server))).await;

    net_handler.recv_loop().await.unwrap();
}

fn setup_logger() -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::new();

    fern::Dispatch::new()
        .level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(
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
                .chain(std::io::stderr()),
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}[{}] [{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file("output.log")?),
        )
        .apply()?;

    debug!("Done setting up logger");
    Ok(())
}
