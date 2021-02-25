use crate::Scanner;
use anyhow::Error;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize)]
pub struct Manager {
    scanner_addr: String,
}

pub type RwManager = Arc<RwLock<Manager>>;

impl Manager {
    pub fn new_rw(scanner_addr: String) -> RwManager {
        Arc::new(RwLock::new(Manager { scanner_addr }))
    }

    pub async fn status(&self) -> Value {
        match self.connect_to_scanner().await {
            Ok(mut scanner) => {
                let metadata = scanner.metadata().await.ok();
                json!({
                    "is_scanner_connected": metadata.is_some(),
                    "err": null,
                    "metadata": metadata,
                })
            }
            Err(err) => {
                json!({
                    "is_scanner_connected": false,
                    "err": err.to_string(),
                    "metadata": null,
                })
            }
        }
    }

    pub fn set_scanner_addr(&mut self, addr: String) {
        self.scanner_addr = addr;
    }

    pub async fn scanner_metadata(&self) -> Result<Value, Error> {
        let mut scanner = self.connect_to_scanner().await?;
        scanner.metadata().await
    }

    async fn connect_to_scanner(&self) -> Result<Scanner, Error> {
        Scanner::connect(&self.scanner_addr).await
    }
}
