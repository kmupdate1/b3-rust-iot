use core::result::Result;

pub trait Spi {
    type Error;
    async fn transfer(&mut self, read: &[u8], write: &[u8]) -> Result<(), Self::Error>;
}
