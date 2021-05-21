mod log;

#[tokio::main]
async fn main() {
    if let Err(e) = log::init_logger() {
        panic!("Failed setting up logger: {}", e);
    }

    arrow_net::start_server("0.0.0.0", 25565).await.unwrap();
}
