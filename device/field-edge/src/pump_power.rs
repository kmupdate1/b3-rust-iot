use capability::IntoDigitalOutput;
use pump::PumpPower;

pub fn create_pump_power<P>(pin: P) -> PumpPower<P::Output>
where
    P: IntoDigitalOutput,
{
    PumpPower::new(pin.into_digital_output())
}
