use crate::{
    lidar::{Listener, Packet, Point},
    Client,
};
use anyhow::Error;
use serde_json::{json, Value};
use tokio::{
    net::ToSocketAddrs,
    sync::{mpsc, oneshot},
};

const CHANNEL_CAPACITY: usize = 128;
const MAX_RANGE: f64 = 100.;

/// An ouster lidar scanner.
#[derive(Debug)]
pub struct Scanner {
    client: Client,
}

struct PointComputer {
    lidar_origin_to_beam_origin_mm: f64,
    beam_altitude_angles: Vec<f64>,
    beam_azimuth_angles: Vec<f64>,
    max_range: f64,
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

    /// Starts the lidar stream to the provided IP.
    ///
    /// Returns a receiver that will be sent the points.
    pub async fn start_lidar_stream(
        &mut self,
        ip: &str,
    ) -> Result<mpsc::Receiver<Vec<Point>>, Error> {
        self.client.set("udp_ip", ip).await?;
        self.client.reinitialize().await?;
        let point_computer = self.point_computer().await?;
        let port = self
            .client
            .get_key("config_param active", "udp_port_lidar")
            .await?
            .as_u64()
            .expect("udp_port_lidar should be a number");
        let addr = format!("0.0.0.0:{}", port);
        let product = self
            .client
            .get_key("sensor_info", "prod_line")
            .await?
            .as_str()
            .expect("prod_line should be a string")
            .parse()?;
        let (bind_sender, bind_receiver) = oneshot::channel();
        let (packet_sender, mut packet_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        let (point_sender, point_receiver) = mpsc::channel(CHANNEL_CAPACITY);
        tokio::spawn(async move {
            log::info!("binding listener to {} with product {}", addr, product);
            let mut listener = match Listener::bind(addr, product).await {
                Ok(listener) => {
                    let _ = bind_sender.send(Ok(()));
                    listener
                }
                Err(err) => {
                    let _ = bind_sender.send(Err(err));
                    return;
                }
            };
            log::info!("bound ok");
            loop {
                match listener.read_packet().await {
                    Ok(option) => {
                        if let Some(packet) = option {
                            let _ = packet_sender.send(packet).await;
                        } else {
                            return;
                        }
                    }
                    Err(err) => {
                        log::warn!("error in udp listener: {}", err);
                        return;
                    }
                }
            }
        });
        bind_receiver.await??;
        tokio::spawn(async move {
            while let Some(packet) = packet_receiver.recv().await {
                let points = point_computer.compute_points(packet);
                let _ = point_sender.send(points).await;
            }
        });
        Ok(point_receiver)
    }

    async fn point_computer(&mut self) -> Result<PointComputer, Error> {
        let beam_intrinsics = self.client.get("beam_intrinsics").await?;
        let lidar_origin_to_beam_origin_mm = beam_intrinsics["lidar_origin_to_beam_origin_mm"]
            .as_f64()
            .expect("value should be an integer");
        let beam_altitude_angles = beam_intrinsics["beam_altitude_angles"]
            .as_array()
            .expect("should be an array")
            .into_iter()
            .map(|v| v.as_f64().expect("should be a float"))
            .collect();
        let beam_azimuth_angles = beam_intrinsics["beam_azimuth_angles"]
            .as_array()
            .expect("should be an array")
            .into_iter()
            .map(|v| v.as_f64().expect("should be a float"))
            .collect();
        Ok(PointComputer {
            lidar_origin_to_beam_origin_mm,
            beam_altitude_angles,
            beam_azimuth_angles,
            max_range: MAX_RANGE,
        })
    }
}

impl PointComputer {
    fn compute_points(&self, packet: Packet) -> Vec<Point> {
        use std::f64::consts::PI;

        let mut points = Vec::new();
        let origin_offset = self.lidar_origin_to_beam_origin_mm / 1000.;
        for measurement in packet.measurements {
            let encoder_count = f64::from(measurement.header.encoder_count);
            let theta_encoder = 2. * PI * (1. - (encoder_count / 90112.));
            for (i, data) in measurement.data.into_iter().enumerate() {
                let range = f64::from(data.range) / 1000.;
                if range > self.max_range {
                    continue;
                }
                let theta_azimuth = -2. * PI * (self.beam_azimuth_angles[i] / 360.);
                let theta = theta_encoder + theta_azimuth;
                let phi = 2. * PI * (self.beam_altitude_angles[i] / 360.);
                let x = (range - origin_offset) * theta.cos() * phi.cos()
                    + origin_offset * theta_encoder.cos();
                let y = (range - origin_offset) * theta.sin() * phi.cos()
                    + origin_offset * theta_encoder.sin();
                let z = (range - origin_offset) * phi.sin();
                points.push(Point { x, y, z })
            }
        }
        points
    }
}
