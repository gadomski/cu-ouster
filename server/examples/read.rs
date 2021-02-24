use cu_ouster::Client;

const SCANNER_ADDR: &'static str = "os-992046000370.local:7501";

#[tokio::main]
async fn main() {
    let mut client = Client::connect(SCANNER_ADDR).await.unwrap();
    println!("{}", client.get("config_txt").await.unwrap());
    println!("{:?}", client.set("udp_ip", "169.254.72.149").await);
    println!("{:?}", client.reinitialize().await);
}