use anyhow::{anyhow, Error};
use serde_json::Value;
use std::io::Result as IoResult;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs},
};

/// A client for talking to an Ouster scanner.
pub struct Client {
    stream: TcpStream,
}

impl Client {
    /// Connect to an Ouster client.
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> IoResult<Client> {
        TcpStream::connect(addr)
            .await
            .map(|stream| Client { stream })
    }

    /// Query sensor information.
    ///
    /// These map to the "get_*" commands from the TCP API command set,
    /// but you don't need to provide the get, i.e. use `get("config_txt")`.
    pub async fn get(&mut self, key: &str) -> Result<Value, Error> {
        let command: Vec<u8> = format!("get_{}\n", key).bytes().collect();
        self.stream.write_all(&command).await?;
        let response = self.response().await?;
        serde_json::from_str(&response).map_err(Error::from)
    }

    /// Sets a config value.
    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let command: Vec<u8> = format!("set_config_param {} {}\n", key, value)
            .bytes()
            .collect();
        self.stream.write_all(&command).await?;
        self.compare_response("set_config_param").await
    }

    /// Reinitializes the sensor
    pub async fn reinitialize(&mut self) -> Result<(), Error> {
        let command: Vec<u8> = format!("reinitialize\n").bytes().collect();
        self.stream.write_all(&command).await?;
        self.compare_response("reinitialize").await
    }

    async fn compare_response(&mut self, expected: &str) -> Result<(), Error> {
        let response = self.response().await?;
        if response == expected {
            Ok(())
        } else {
            Err(anyhow!(
                "response does not match '{}': '{}'",
                expected,
                response
            ))
        }
    }

    async fn response(&mut self) -> Result<String, Error> {
        let mut response = String::new();
        let mut stream = BufReader::new(&mut self.stream);
        let count = stream.read_line(&mut response).await?;
        let response = response.trim();
        if count == 0 {
            Err(anyhow!("end of file when expecting response"))
        } else if response.starts_with("error") {
            Err(anyhow!("ouster client {}", response))
        } else {
            Ok(response.to_string())
        }
    }
}
