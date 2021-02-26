use cu_ouster::{lidar::LidarDecoder, PointComputer, Product};
use etherparse::SlicedPacket;
use futures::stream::StreamExt;
use las::{point::Format, Builder, Point, Write as LasWrite, Writer};
use pcap::Capture;
use serde_json::Value;
use std::{env, fs::File, io::Cursor};
use tokio_util::codec::FramedRead;

const MAX_RANGE: f64 = 200.;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "cu_ouster=info")
    }
    pretty_env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cu_ouster [pcap] [json] [las]");
        return;
    }
    let ref pcap = args[1];
    let json_file = File::open(&args[2]).unwrap();
    let ref outfile = args[3];
    let json: Value = serde_json::from_reader(json_file).unwrap();
    let product: Product = json["prod_line"]
        .as_str()
        .expect("prod_line should be a string")
        .parse()
        .unwrap();
    let lidar_origin_to_beam_origin_mm = json["lidar_origin_to_beam_origin_mm"]
        .as_f64()
        .expect("value should be an integer");
    let beam_altitude_angles = json["beam_altitude_angles"]
        .as_array()
        .expect("should be an array")
        .into_iter()
        .map(|v| v.as_f64().expect("should be a float"))
        .collect();
    let beam_azimuth_angles = json["beam_azimuth_angles"]
        .as_array()
        .expect("should be an array")
        .into_iter()
        .map(|v| v.as_f64().expect("should be a float"))
        .collect();
    let lidar_to_sensor_transform = json["lidar_to_sensor_transform"]
        .as_array()
        .expect("should be an array")
        .into_iter()
        .map(|v| v.as_f64().expect("should be a float"))
        .collect();
    let point_computer = PointComputer::new(
        lidar_origin_to_beam_origin_mm,
        beam_altitude_angles,
        beam_azimuth_angles,
        lidar_to_sensor_transform,
        MAX_RANGE,
    );

    let mut capture = Capture::from_file(pcap).unwrap();
    let decoder = LidarDecoder::new(product);
    let mut builder = Builder::from((1, 2));
    builder.point_format = Format::new(1).unwrap();
    let header = builder.into_header().unwrap();
    let mut writer = Writer::from_path(outfile, header).unwrap();

    let mut count = 0;
    while let Ok(packet) = capture.next() {
        let packet = SlicedPacket::from_ethernet(packet.data).unwrap();
        let data = Cursor::new(packet.payload);
        let mut framed_read = FramedRead::new(data, decoder);
        if let Some(packet) = framed_read.next().await {
            if let Ok(packet) = packet {
                let points = point_computer.compute_points(packet);
                for point in points {
                    count += 1;
                    let point = Point {
                        x: point.x,
                        y: point.y,
                        z: point.z,
                        intensity: point.reflectivity,
                        gps_time: Some(point.time),
                        ..Default::default()
                    };
                    writer.write(point).unwrap();
                }
            }
        }
    }
    println!("{} points writen to {}", count, outfile);
}
