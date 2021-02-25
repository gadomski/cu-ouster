use crate::Scanner;
use anyhow::Error;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Manager {
    scanner_addr: String,
}

impl Manager {
    pub fn new(scanner_addr: String) -> Manager {
        Manager { scanner_addr }
    }

    pub async fn scanner_metadata(&self) -> Result<Value, Error> {
        let mut scanner = Scanner::connect(&self.scanner_addr).await?;
        scanner.metadata().await
    }
}
