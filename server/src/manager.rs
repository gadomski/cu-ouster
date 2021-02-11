use crate::{Client, Config};
use anyhow::{anyhow, Error};
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

const CHANNEL_BUFFER: usize = 32;

#[derive(Debug, Clone)]
pub struct Manager {
    config: Config,
    sender: mpsc::Sender<Command>,
}

#[derive(Debug)]
pub enum Command {
    Get {
        key: String,
        responder: oneshot::Sender<Result<Value, Error>>,
    },
}

impl Manager {
    pub fn new(config: Config) -> Manager {
        let (sender, mut receiver) = mpsc::channel(CHANNEL_BUFFER);
        let addr = config.scanner_addr.clone();
        tokio::spawn(async move {
            let a = addr.clone();
            let mut client = match Client::connect(a).await {
                Ok(client) => Some(client),
                Err(err) => {
                    log::warn!("unable to connect client at boot: {}", err);
                    None
                }
            };
            while let Some(command) = receiver.recv().await {
                if client.is_none() {
                    let a = addr.clone();
                    client = match Client::connect(a).await {
                        Ok(client) => Some(client),
                        Err(err) => {
                            log::warn!("unable to connect client during response: {}", err);
                            None
                        }
                    }
                }
                match command {
                    Command::Get { key, responder } => {
                        let response = if let Some(client) = client.as_mut() {
                            client.get(&key).await
                        } else {
                            Err(anyhow!("not connected to client"))
                        };
                        if response.is_err() {
                            log::info!("resetting client connection");
                            client = None;
                        }
                        let _ = responder.send(response);
                    }
                }
            }
        });
        Manager { config, sender }
    }

    pub fn sender(&self) -> &mpsc::Sender<Command> {
        &self.sender
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
