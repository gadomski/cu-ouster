use crate::{Command, Manager};
use serde::Serialize;
use serde_json::Value;
use std::convert::Infallible;
use tokio::sync::{mpsc::Sender, oneshot};
use warp::{reply, Reply};

#[derive(Debug, Serialize)]
pub struct Status {
    is_scanner_connected: bool,
    scanner_addr: String,
    sensor_info: Value,
    alerts: Value,
}

pub async fn status(manager: Manager) -> Result<impl Reply, Infallible> {
    let sensor_info = get_json(manager.sender(), "sensor_info").await;
    let alerts = if !sensor_info.is_null() {
        get_json(manager.sender(), "alerts").await
    } else {
        Value::Null
    };
    let status = Status {
        is_scanner_connected: !(sensor_info.is_null() | alerts.is_null()),
        scanner_addr: manager.config().scanner_addr.to_string(),
        sensor_info,
        alerts,
    };
    Ok(reply::json(&status))
}

async fn get_json(sender: &Sender<Command>, key: &str) -> Value {
    let (responder, receiver) = oneshot::channel();
    let command = Command::Get {
        key: key.to_string(),
        responder,
    };
    if let Err(err) = sender.send(command).await {
        log::error!(
            "error when sending Command::Get {{ key: {} }} to manager: {}",
            key,
            err
        );
        return Value::Null;
    }
    match receiver.await {
        Ok(result) => match result {
            Ok(json) => json,
            Err(err) => {
                log::warn!(
                    "unable to get json via Command::Get {{ key: {} }}: {}",
                    key,
                    err
                );
                Value::Null
            }
        },
        Err(err) => {
            log::error!(
                "error when receiving response from Command::Get {{ key: {} }} from manager: {}",
                key,
                err
            );
            Value::Null
        }
    }
}
