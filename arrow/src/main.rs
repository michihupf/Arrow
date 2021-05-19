

#[tokio::main]
async fn main() {
    arrow_net::start_server("0.0.0.0", 25565).await.unwrap();
}
