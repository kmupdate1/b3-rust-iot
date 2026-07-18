use core::result::Result;

pub trait Uart {
    type Error;
    async fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error>;
    async fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
