use clap::{App, Arg};
use hyper::server::Server;
use listenfd::ListenFd;
use ouster::{filters, Config, Manager};
use std::{convert::Infallible, env};

#[tokio::main]
async fn main() {
    let matches = App::new("cu-ouster")
        .arg(Arg::with_name("CONFIG").takes_value(true).required(true))
        .get_matches();
    let config = Config::from_path(matches.value_of("CONFIG").unwrap()).unwrap();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    let manager = Manager::new(config);
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
        Server::bind(&([127, 0, 0, 1], 4242).into())
    };

    server.serve(make_service).await.unwrap();
}
