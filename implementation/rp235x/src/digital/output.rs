use embassy_rp::gpio::Output;
use function::digital::DigitalOutput;

pub struct DigitalOutputPin<'d> {
    pin: Output<'d>,
}

impl<'d> DigitalOutputPin<'d> {
    pub fn new(pin: Output<'d>) -> Self {
        Self { pin }
    }
}

impl DigitalOutput for DigitalOutputPin<'_> {
    fn set_high(&mut self) {
        self.pin.set_high();
    }

    fn set_low(&mut self) {
        self.pin.set_low();
    }
}
