use crate::{handlers, Manager};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn api(manager: Manager) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    scanner_metadata(manager.clone())
}

fn scanner_metadata(
    manager: Manager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("scanner" / "metadata")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(handlers::scanner_metadata)
}

fn with_manager(manager: Manager) -> impl Filter<Extract = (Manager,), Error = Infallible> + Clone {
    warp::any().map(move || manager.clone())
}
