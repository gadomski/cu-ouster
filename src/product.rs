use anyhow::{anyhow, Error};
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Product {
    generation: u8,
    channels: u16,
}

impl Product {
    /// Creates a new product from the given generation and number of channels.
    ///
    /// Checks for invalid values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cu_ouster::Product;
    /// let product = Product::new(1, 128).unwrap();
    /// assert_eq!(1, product.generation());
    /// assert_eq!(128, product.channels());
    /// assert_eq!("OS1-128", product.to_string());
    /// ```
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

    /// Returns the length of a measurement block, in bytes, for this product.
    ///
    /// Based on the number of channels.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cu_ouster::Product;
    /// let product = Product::new(1, 128).unwrap();
    /// assert_eq!(1556, product.measurement_block_len());
    /// ```
    pub fn measurement_block_len(&self) -> usize {
        match self.channels {
            16 => 212,
            32 => 404,
            64 => 788,
            128 => 1556,
            _ => panic!("shouldn't have a weird channel count: {}", self.channels),
        }
    }

    /// Returns the number of channels in this product.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cu_ouster::Product;
    /// let product = Product::new(1, 128).unwrap();
    /// assert_eq!(128, product.channels());
    /// ```
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Returns the generation of this product.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cu_ouster::Product;
    /// let product = Product::new(1, 128).unwrap();
    /// assert_eq!(1, product.generation());
    /// ```
    pub fn generation(&self) -> u8 {
        self.generation
    }
}

impl FromStr for Product {
    type Err = Error;

    fn from_str(s: &str) -> Result<Product, Error> {
        let regex = Regex::new(r"^(os|OS)-?(\d+)-(\d+)$")?;
        if let Some(captures) = regex.captures(s) {
            let generation = captures.get(2).unwrap().as_str().parse().unwrap();
            let channels = captures.get(3).unwrap().as_str().parse().unwrap();
            Product::new(generation, channels)
        } else {
            Err(anyhow!("invalid product specifier: {}", s))
        }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OS{}-{}", self.generation, self.channels)
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
        assert_eq!(
            "OS-1-128".parse::<Product>().unwrap(),
            Product::new(1, 128).unwrap()
        );
        assert!("OS1-127".parse::<Product>().is_err());
    }
}
