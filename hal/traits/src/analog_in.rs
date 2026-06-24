use core::result::Result;

pub trait AnalogIn {
    type Error;
    fn read_mv(&mut self) -> Result<u16, Self::Error>;
}
