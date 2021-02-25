use crate::Scanner;
use anyhow::Error;
use serde::Serialize;
use serde_json::Value;
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

    pub fn set_scanner_addr(&mut self, addr: String) {
        self.scanner_addr = addr;
    }

    pub async fn scanner_metadata(&self) -> Result<Value, Error> {
        let mut scanner = Scanner::connect(&self.scanner_addr).await?;
        scanner.metadata().await
    }
}
