use function::DigitalOutput;

pub struct Switch<O>
where
    O: DigitalOutput,
{
    output: O,
}

impl<O> Switch<O>
where
    O: DigitalOutput,
{
    pub fn new(output: O) -> Self {
        Self { output }
    }

    pub fn on(&mut self) {
        self.output.set_high();
    }

    pub fn off(&mut self) {
        self.output.set_low();
    }
}
