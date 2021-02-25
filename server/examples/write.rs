use cu_ouster::Scanner;
use std::env;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();
    let mut scanner = Scanner::connect(&args[1]).await.unwrap();
    let mut receiver = scanner.start_lidar_stream(&args[2]).await.unwrap();
    let mut packets = 0;
    while let Some(points) = receiver.recv().await {
        for point in points {
            println!("{},{},{}", point.x, point.y, point.z);
        }
        packets += 1;
        if packets > 1000 {
            break;
        }
    }
}
