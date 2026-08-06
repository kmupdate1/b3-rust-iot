use core::result::Result;

pub trait DigitalInput {
    type Error;
    fn is_high(&self) -> Result<bool, Self::Error>;
    fn is_low(&self) -> Result<bool, Self::Error>;
}
