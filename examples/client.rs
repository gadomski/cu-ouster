use cu_ouster::Client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut client = Client::connect(&args[1]).await.unwrap();
    println!("{}", client.get("config_txt").await.unwrap());
    client.set("udp_ip", &args[2]).await.unwrap();
    client.reinitialize().await.unwrap();
}
