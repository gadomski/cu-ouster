use cu_ouster::lidar::Server;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut server = Server::bind(&args[1], args[2].parse().unwrap())
        .await
        .unwrap();
    let _frame = server.read_frame().await.unwrap();
}
