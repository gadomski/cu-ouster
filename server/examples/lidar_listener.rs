use cu_ouster::lidar::Listener;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let product = args[2].parse().unwrap();
    let mut server = Listener::bind(&args[1], product).await.unwrap();
    let packet = server.read_packet().await.unwrap();
    println!("{:?}", packet);
}
