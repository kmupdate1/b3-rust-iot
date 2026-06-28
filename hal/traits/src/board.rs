use crate::analog_in::AnalogIn;

pub trait Board {
    type AnalogInput: AnalogIn;
    
    fn analog_input(&mut self) -> &mut Self::AnalogInput;
}
