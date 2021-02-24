use anyhow::{anyhow, Error};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Product {
    generation: u8,
    channels: u16,
}

impl Product {
    pub fn new(generation: u8, channels: u16) -> Result<Product, Error> {
        if channels == 16 {
            if generation != 1 {
                return Err(anyhow!(
                    "invalid generation for 16 channels: {}",
                    generation
                ));
            }
        } else if (channels == 32) || (channels == 64) || (channels == 128) {
            if generation > 2 {
                return Err(anyhow!(
                    "invalid generation for {} channels: {}",
                    channels,
                    generation
                ));
            }
        } else {
            return Err(anyhow!("invalid number of channels: {}", channels));
        }
        Ok(Product {
            generation,
            channels,
        })
    }
}

impl FromStr for Product {
    type Err = Error;

    fn from_str(s: &str) -> Result<Product, Error> {
        let regex = Regex::new(r"^(os|OS)(\d+)-(\d+)$")?;
        if let Some(captures) = regex.captures(s) {
            let generation = captures.get(2).unwrap().as_str().parse().unwrap();
            let channels = captures.get(3).unwrap().as_str().parse().unwrap();
            Product::new(generation, channels)
        } else {
            Err(anyhow!("invalid product specifier: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert_eq!(
            "os1-32".parse::<Product>().unwrap(),
            Product::new(1, 32).unwrap()
        );
        assert_eq!(
            "OS1-32".parse::<Product>().unwrap(),
            Product::new(1, 32).unwrap()
        );
        assert!("OS1-127".parse::<Product>().is_err());
    }
}
