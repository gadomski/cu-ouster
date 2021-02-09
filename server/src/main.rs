use couster::{filters, Manager};
use hyper::server::Server;
use listenfd::ListenFd;
use std::{convert::Infallible, env};

const SCANNER_ADDR: &'static str = "os1-991906000126.local:7501";

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    let manager = Manager::new(SCANNER_ADDR);
    let api = filters::api(manager);
    let service = warp::service(api);
    let make_service = hyper::service::make_service_fn(|_: _| {
        let service = service.clone();
        async move { Ok::<_, Infallible>(service) }
    });
    let mut listenfd = ListenFd::from_env();
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&([127, 0, 0, 1], 3030).into())
    };

    server.serve(make_service).await.unwrap();
}
