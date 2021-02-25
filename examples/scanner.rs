use cu_ouster::Scanner;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    let mut scanner = Scanner::connect(addr).await.unwrap();
    let metadata = scanner.metadata().await.unwrap();
    println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
}
