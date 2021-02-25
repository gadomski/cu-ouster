use cu_ouster::{filters, Manager};
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "cu_ouster=info")
    }
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cu_ouster [port] [scanner_addr]");
        return;
    }
    let port = args[1].parse().unwrap();
    let scanner_addr = args[2].clone();

    let manager = Manager::new(scanner_addr);
    let api = filters::api(manager);
    let cors = warp::cors().allow_any_origin();
    let routes = api.with(warp::log("cu_ouster")).with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
