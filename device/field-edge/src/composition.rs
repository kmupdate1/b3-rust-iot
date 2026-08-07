use function::IntoDigitalOutput;
use pump::PumpPower;
use rp235x::{Pin0, Pin1};
use crate::create_pump_power;

pub fn composePin0<'d>(
    pin0: Pin0<'d>,
) -> PumpPower<<Pin0<'d> as IntoDigitalOutput>::Output> {
    create_pump_power(pin0)
}

pub fn composePin1<'d>(
    pin1: Pin1<'d>,
) -> PumpPower<<Pin1<'d> as IntoDigitalOutput>::Output> {
    create_pump_power(pin1)
}
