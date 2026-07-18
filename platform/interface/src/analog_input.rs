use core::result::Result;

pub trait AnalogInput {
    type Error;
    fn read(&mut self) -> Result<u16, Self::Error>;
}
