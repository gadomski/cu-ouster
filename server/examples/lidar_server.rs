use cu_ouster::lidar::Server;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let product = args[2].parse().unwrap();
    let mut server = Server::bind(&args[1], product).await.unwrap();
    let _frame = server.read_frame().await.unwrap();
}
