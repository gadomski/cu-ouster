use anyhow::Error;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub scanner_addr: String,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut config = String::new();
        File::open(path).and_then(|mut f| f.read_to_string(&mut config))?;
        toml::from_str(&config).map_err(Error::from)
    }
}
