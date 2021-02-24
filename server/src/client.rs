use anyhow::{Error, anyhow};
use serde_json::Value;
use std::fmt;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs},
};

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

#[derive(Debug, thiserror::Error)]
pub struct EndOfFile;

impl Client {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Client, Error> {
        TcpStream::connect(addr)
            .await
            .map(|s| Client { stream: s })
            .map_err(Error::from)
    }

    pub async fn get(&mut self, key: &str) -> Result<Value, Error> {
        log::debug!("getting {}", key);
        let query = format!("get_{}\n", key);
        let key: Vec<u8> = query.bytes().collect();
        self.stream.write_all(&key).await?;
        let mut response = String::new();
        let mut stream = BufReader::new(&mut self.stream);
        let count = stream.read_line(&mut response).await?;
        if count == 0 {
            log::warn!("end of file");
            return Err(Error::from(EndOfFile));
        }
        log::debug!("received response: {}", response);
        if response.starts_with("error") {
            Err(anyhow!("ouster client {}", response.trim()))
        } else {
            serde_json::from_str(&response).map_err(Error::from)
        }
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        log::debug!("setting {}={}", key, value);
        let command: Vec<u8> = format!("set_config_param {} {}\n", key, value).bytes().collect();
        self.stream.write_all(&command).await?;
        let mut response = String::new();
        let mut stream = BufReader::new(&mut self.stream);
        let count = stream.read_line(&mut response).await?;
        if count == 0 {
            log::warn!("end of file");
            return Err(Error::from(EndOfFile));
        }
        log::debug!("received response: {}", response);
        if response.starts_with("error") {
            Err(anyhow!("ouster client {}", response.trim()))
        } else if response.trim() != "set_config_param" {
            Err(anyhow!("response {} does not match {}", response.trim(), "set_config_param"))
        } else {
            Ok(())
        }
    }

    pub async fn reinitialize(&mut self) -> Result<(), Error> {
        log::debug!("reinitializing");
        let command: Vec<u8> = format!("reinitialize\n").bytes().collect();
        self.stream.write_all(&command).await?;
        let mut response = String::new();
        let mut stream = BufReader::new(&mut self.stream);
        let count = stream.read_line(&mut response).await?;
        if count == 0 {
            log::warn!("end of file");
            return Err(Error::from(EndOfFile));
        }
        log::debug!("received response: {}", response);
        if response.starts_with("error") {
            Err(anyhow!("ouster client {}", response.trim()))
        } else if response.trim() != "reinitialize" {
            Err(anyhow!("response {} does not match {}", response.trim(), "reinitialize"))
        } else {
            Ok(())
        }
    }
}

impl fmt::Display for EndOfFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "end of file")
    }
}
