use crate::Manager;
use warp::{reject, reply, Rejection, Reply};

pub async fn scanner_metadata(manager: Manager) -> Result<impl Reply, Rejection> {
    match manager.scanner_metadata().await {
        Ok(metadata) => Ok(reply::json(&metadata)),
        Err(err) => {
            log::warn!("error when fetching scanner metadata: {}", err);
            Err(reject::not_found())
        }
    }
}
