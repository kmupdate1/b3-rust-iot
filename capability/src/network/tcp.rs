use core::result::Result;

pub trait Tcp {
    type Error;

    async fn connect(&mut self, address: &str, port: u16) -> Result<(), Self::Error>;
    async fn send(&mut self, message: &[u8]) -> Result<(), Self::Error>;
    async fn close(&mut self) -> Result<(), Self::Error>;
}
