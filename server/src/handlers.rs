use crate::RwManager;
use std::convert::Infallible;
use warp::{reject, reply, Rejection, Reply};

pub async fn manager(manager: RwManager) -> Result<impl Reply, Infallible> {
    let manager = manager.read().await;
    Ok(reply::json(&*manager))
}

pub async fn set_scanner_addr(addr: String, manager: RwManager) -> Result<impl Reply, Infallible> {
    let mut manager = manager.write().await;
    manager.set_scanner_addr(addr);
    Ok(reply::json(&()))
}

pub async fn scanner_metadata(manager: RwManager) -> Result<impl Reply, Rejection> {
    let manager = manager.read().await;
    match manager.scanner_metadata().await {
        Ok(metadata) => Ok(reply::json(&metadata)),
        Err(err) => {
            log::warn!("error when fetching scanner metadata: {}", err);
            Err(reject::not_found())
        }
    }
}
