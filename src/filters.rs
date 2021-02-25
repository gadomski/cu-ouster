use crate::{handlers, RwManager};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn api(manager: RwManager) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    status(manager.clone())
        .or(self::manager(manager.clone()))
        .or(set_scanner_addr(manager.clone()))
        .or(scanner_metadata(manager.clone()))
}

fn status(manager: RwManager) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("status")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(handlers::status)
}

fn manager(manager: RwManager) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("manager")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(handlers::manager)
}

fn set_scanner_addr(
    manager: RwManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("scanner" / "set_addr" / String)
        .and(warp::post())
        .and(with_manager(manager))
        .and_then(handlers::set_scanner_addr)
}

fn scanner_metadata(
    manager: RwManager,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("scanner" / "metadata")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(handlers::scanner_metadata)
}

fn with_manager(
    manager: RwManager,
) -> impl Filter<Extract = (RwManager,), Error = Infallible> + Clone {
    warp::any().map(move || manager.clone())
}
