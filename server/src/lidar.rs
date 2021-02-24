use crate::Product;
use anyhow::Error;
use bytes::BytesMut;
use futures::stream::StreamExt;
use std::io::Result as IoResult;
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio_util::{codec::Decoder, udp::UdpFramed};

const BUFFER_CAPACITY: usize = 12608 * 64;

/// A listener for lidar data.
pub struct Server {
    stream: UdpFramed<LidarDecoder>,
    buffer: BytesMut,
}

/// A lidar data frame.
pub struct Frame {}

#[derive(Debug)]
struct LidarDecoder {
    product: Product,
}

impl Server {
    /// Binds this server to the provided address.
    pub async fn bind<A: ToSocketAddrs>(addr: A, product: Product) -> IoResult<Server> {
        let decoder = LidarDecoder { product };
        UdpSocket::bind(addr).await.map(|socket| Server {
            stream: UdpFramed::new(socket, decoder),
            buffer: BytesMut::with_capacity(BUFFER_CAPACITY),
        })
    }

    /// Reads a single frame of lidar data from the socket.
    ///
    /// This is a buffered operation ... there may be several frames buffered,
    /// or several calls to read may have to happen before a frame can be returned.
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
        self.stream.next().await;
        unimplemented!()
    }
}

impl Decoder for LidarDecoder {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Frame>, Self::Error> {
        unimplemented!()
    }
}
