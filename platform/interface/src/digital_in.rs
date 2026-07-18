use core::result::Result;
use crate::level::Level;

pub trait DigitalIn {
    type Error;
    fn get_level(&self) -> Result<Level, Self::Error>;
}
