use clap::{App, Arg};
use cu_ouster::{filters, Config, Manager};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let matches = App::new("cu-ouster")
        .arg(Arg::with_name("CONFIG").takes_value(true).required(true))
        .get_matches();
    let config = Config::from_path(matches.value_of("CONFIG").unwrap()).unwrap();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "cu_ouster=info");
    }
    pretty_env_logger::init();

    let server_addr = config.server_addr.parse::<SocketAddr>().unwrap();
    let manager = Manager::new(config);
    let api = filters::api(manager);
    warp::serve(api).run(server_addr).await; 
}
