use crate::error::AnalogInputError;
use crate::gpio::adc::Adc;
use crate::peripherals::pins::{Pin26, Pin27, Pin28};
use core::result::Result;
use function::analog::analog_input::AnalogInput;

pub struct AnalogInputPin<P> {
    pin: P,
    adc: Adc,
}

impl<P> AnalogInput for AnalogInputPin<P> {
    type Error = AnalogInputError;
    fn read(&mut self) -> Result<u16, Self::Error> { todo!() }
}

macro_rules! impl_analog_input_pin {
    ($($p:ty),* $(,)?) => {$(
        impl AnalogInputPin<$p> {
            pub fn attach(adc: Adc, pin: $p) -> Self {
                Self { pin, adc }
            }
        }
    )*};
}

impl_analog_input_pin!(Pin26, Pin27, Pin28);
