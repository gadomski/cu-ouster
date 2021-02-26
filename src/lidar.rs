use crate::Product;
use anyhow::{anyhow, Error};
use bytes::{Buf, BytesMut};
use futures::stream::StreamExt;
use regex::Regex;
use std::{io::Result as IoResult, str::FromStr};
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio_util::{codec::Decoder, udp::UdpFramed};

/// A listener for lidar data.
pub struct Listener {
    stream: UdpFramed<LidarDecoder>,
}

/// A lidar point.
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub time: f64,
    pub reflectivity: u16,
}

/// A lidar data packet.
#[derive(Debug)]
pub struct Packet {
    pub measurements: Vec<Measurement>,
}

/// A lidar measurement block.
#[derive(Debug)]
pub struct Measurement {
    pub header: Header,
    pub data: Vec<Data>,
    pub status: u32,
}

/// A lidar header.
#[derive(Debug)]
pub struct Header {
    pub timestamp: u64,
    pub measurement_id: u16,
    pub frame_id: u16,
    pub encoder_count: u32,
}

/// A lidar data block.
#[derive(Debug)]
pub struct Data {
    pub range: u32,
    pub signal_photons: u16,
    pub reflectivity: u16,
    pub near_infrared_photons: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct LidarDecoder {
    product: Product,
}

#[derive(Debug)]
pub struct Mode {
    pub horizontal_resolution: u32,
    pub rotation_rate: u8,
}

impl Listener {
    /// Binds this server to the provided address.
    pub async fn bind<A: ToSocketAddrs>(addr: A, product: Product) -> IoResult<Listener> {
        let decoder = LidarDecoder { product };
        UdpSocket::bind(addr).await.map(|socket| Listener {
            stream: UdpFramed::new(socket, decoder),
        })
    }

    /// Reads a single packet of lidar data from the socket.
    pub async fn read_packet(&mut self) -> Result<Option<Packet>, Error> {
        if let Some(packet) = self.stream.next().await {
            match packet {
                Ok((packet, _)) => Ok(Some(packet)),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }
}

impl LidarDecoder {
    pub fn new(product: Product) -> LidarDecoder {
        LidarDecoder { product }
    }

    fn packet_len(&self) -> usize {
        self.product.measurement_block_len() * 16
    }
}

impl Decoder for LidarDecoder {
    type Item = Packet;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Packet>, Self::Error> {
        let packet_len = self.packet_len();
        if src.len() < packet_len {
            return Ok(None);
        }
        let data = src.split_to(packet_len);
        Ok(Some(Packet::new(&self.product, data)))
    }
}

impl Packet {
    fn new(product: &Product, mut data: BytesMut) -> Packet {
        let measurement_block_len = product.measurement_block_len();
        let measurements = (0..16)
            .map(|_| Measurement::new(product, data.split_to(measurement_block_len)))
            .collect();
        Packet { measurements }
    }
}

impl Measurement {
    fn new(product: &Product, mut data: BytesMut) -> Measurement {
        let header = Header::new(data.split_to(16));
        let lidar_data = (0..product.channels())
            .map(|_| Data::new(data.split_to(12)))
            .collect();
        let status = data.get_u32_le();
        Measurement {
            header,
            data: lidar_data,
            status,
        }
    }
}

impl Header {
    fn new(mut data: BytesMut) -> Header {
        Header {
            timestamp: data.get_u64_le(),
            measurement_id: data.get_u16_le(),
            frame_id: data.get_u16_le(),
            encoder_count: data.get_u32_le(),
        }
    }
}

impl Data {
    fn new(mut data: BytesMut) -> Data {
        Data {
            range: data.get_u32_le(),
            signal_photons: data.get_u16_le(),
            reflectivity: data.get_u16_le(),
            near_infrared_photons: data.get_u16_le(),
        }
    }
}

impl FromStr for Mode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Mode, Error> {
        let regex = Regex::new(r"^(\d+)x(\d+)$")?;
        if let Some(captures) = regex.captures(s) {
            let horizontal_resolution = captures.get(1).unwrap().as_str().parse()?;
            let rotation_rate = captures.get(2).unwrap().as_str().parse()?;
            Mode::new(horizontal_resolution, rotation_rate)
        } else {
            Err(anyhow!("invalid mode string: {}", s))
        }
    }
}

impl Mode {
    fn new(horizontal_resolution: u32, rotation_rate: u8) -> Result<Mode, Error> {
        match horizontal_resolution {
            512 | 1024 => match rotation_rate {
                10 | 20 => Ok(Mode {
                    horizontal_resolution,
                    rotation_rate,
                }),
                _ => Err(anyhow!(
                    "invalid rotation rate for {} horizontal resoltion: {}",
                    horizontal_resolution,
                    rotation_rate
                )),
            },
            2048 => {
                if rotation_rate == 10 {
                    Ok(Mode {
                        horizontal_resolution,
                        rotation_rate,
                    })
                } else {
                    Err(anyhow!(
                        "invalid rotation rate for 2048 horizontal resolution: {}",
                        rotation_rate
                    ))
                }
            }
            _ => Err(anyhow!(
                "invalid horizontal resolution: {}",
                horizontal_resolution
            )),
        }
    }
}
