use core::str::FromStr;
use embassy_net::Ipv4Address;
use embassy_net::tcp::{ConnectError, TcpSocket};
use embedded_io_async::Write;

pub struct Rp235xTcp {
    stack: embassy_net::Stack<'static>,
}

pub struct Rp235xTcpConnection<'a> {
    socket: TcpSocket<'a>,
}

impl Rp235xTcp {
    pub(crate) fn new(stack: embassy_net::Stack<'static>) -> Self {
        Self { stack }
    }

    pub async fn connect<'a>(
        &self,
        address: &str,
        port: u16,
        rx_buffer: &'a mut [u8],
        tx_buffer: &'a mut [u8],
    ) -> Result<Rp235xTcpConnection<'a>, Rp235xTcpError> {
        let address =
            Ipv4Address::from_str(address)
                .map_err(|_| Rp235xTcpError::InvalidAddress)?;

        let mut socket =
            TcpSocket::new(self.stack, rx_buffer, tx_buffer);

        socket
            .connect((address, port))
            .await
            .map_err(Rp235xTcpError::Connect)?;

        Ok(Rp235xTcpConnection { socket })
    }
}

impl<'a> Rp235xTcpConnection<'a> {
    pub async fn send(
        &mut self, data: &[u8],
    ) -> Result<(), Rp235xTcpError> {
        self.socket
            .write_all(data)
            .await
            .map_err(|_| Rp235xTcpError::Write)
    }

    pub fn close(&mut self) {
        self.socket.close();
    }
}

#[derive(Debug)]
pub enum Rp235xTcpError {
    InvalidAddress,
    Connect(ConnectError),
    Write,
}
