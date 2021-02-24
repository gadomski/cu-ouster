use std::io::Result as IoResult;
use tokio::net::{TcpStream, ToSocketAddrs};

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
}
