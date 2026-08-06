use crate::tcp::Result::Ok;
use core::result::Result;
use embassy_net::{IpAddress, IpEndpoint};
use embassy_net::tcp::TcpSocket;
use embedded_io_async::Write;
use function::network::tcp::Tcp;

pub enum TcpError {
    Connect(embassy_net::tcp::ConnectError),
    Io(embassy_net::tcp::Error),
}

pub struct EmbassyTcp<'a> {
    socket: TcpSocket<'a>
}

impl<'a> EmbassyTcp<'a> {
    pub fn new(socket: TcpSocket<'a>) -> Self {
        Self { socket }
    }
}

impl Tcp for EmbassyTcp<'_> {
    type Error = TcpError;

    async fn connect(&mut self, address: &str, port: u16) -> Result<(), Self::Error> {
        let endpoint = IpEndpoint::new(
            IpAddress::v4(
                address.as_bytes()[0],
                address.as_bytes()[1],
                address.as_bytes()[2],
                address.as_bytes()[3],
            ),
            port,
        );

        self.socket
            .connect(endpoint)
            .await
            .map_err(TcpError::Connect)
    }

    async fn send(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        self.socket
            .write_all(message)
            .await
            .map_err(TcpError::Io)
    }

    async fn close(&mut self) -> Result<(), Self::Error> {
        Ok(self.socket.close())
    }
}
