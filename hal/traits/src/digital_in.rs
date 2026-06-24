pub trait DigitalIn {
    fn get_level(&self) -> bool;
}
