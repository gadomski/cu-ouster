use crate::{handlers, Manager};
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn api(manager: Manager) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let cors = warp::cors().allow_any_origin();
    status(manager).with(cors)
}

fn status(manager: Manager) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("status")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(handlers::status)
}

fn with_manager(manager: Manager) -> impl Filter<Extract = (Manager,), Error = Infallible> + Clone {
    warp::any().map(move || manager.clone())
}
