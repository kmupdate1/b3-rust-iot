use driver_core::Switch;
use function::DigitalOutput;

pub struct PumpPower<O>
where
    O: DigitalOutput,
{
    switch: Switch<O>,
}

impl<O> PumpPower<O>
where
    O: DigitalOutput,
{
    pub fn new(output: O) -> Self {
        Self { switch: Switch::new(output) }
    }

    pub fn start(&mut self) {
        self.switch.on();
    }

    pub fn stop(&mut self) {
        self.switch.off();
    }
}
