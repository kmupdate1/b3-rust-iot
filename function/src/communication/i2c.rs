use core::result::Result;

pub trait I2c {
    type Error;

    async fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error>;
    async fn read(&mut self, address: u8, data: &mut [u8]) -> Result<(), Self::Error>;
    async fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error>;
}
