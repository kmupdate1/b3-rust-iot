pub trait DigitalOutput {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

pub trait IntoDigitalOutput {
    type Output: DigitalOutput;
    
    fn into_digital_output(self) -> Self::Output;
}
