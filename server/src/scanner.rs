use crate::Client;
use anyhow::Error;
use serde_json::{json, Value};
use tokio::net::ToSocketAddrs;

/// An ouster lidar scanner.
#[derive(Debug)]
pub struct Scanner {
    client: Client,
}

impl Scanner {
    /// Creates a new scanner with the provided address.
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Scanner, Error> {
        let client = Client::connect(addr).await?;
        Ok(Scanner { client })
    }

    /// Returns this scanner's metadata.
    pub async fn metadata(&mut self) -> Result<Value, Error> {
        Ok(json!({
            "config_param": {
                "active": self.client.get("config_param active").await?,
                "staged": self.client.get("config_param staged").await?,
            },
            "sensor_info": self.client.get("sensor_info").await?,
            "time_info": self.client.get("time_info").await?,
            "beam_intrinsics": self.client.get("beam_intrinsics").await?,
            "imu_intrinsics": self.client.get("imu_intrinsics").await?,
            "lidar_intrinsics": self.client.get("lidar_intrinsics").await?,
        }))
    }
}
