mod config;
mod log;

#[tokio::main]
async fn main() {
    if let Err(e) = log::init_logger() {
        panic!("Failed setting up logger: {}", e);
    }

    let config = config::load_config().await;

    arrow_net::start_server(config.host(), *config.port())
        .await
        .unwrap();
}
