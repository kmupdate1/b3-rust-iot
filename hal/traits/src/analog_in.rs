use core::result::Result;

pub trait AnalogIn {
    type Error;
    async fn read(&mut self) -> Result<u16, Self::Error>;
}
