pub trait DigitalPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn get_level(&self) -> bool;
}
