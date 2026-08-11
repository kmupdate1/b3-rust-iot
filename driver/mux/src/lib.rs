#![no_std]

use capability::DigitalOutput;

pub struct Mux<S0, S1, S2>
where
    S0: DigitalOutput,
    S1: DigitalOutput,
    S2: DigitalOutput,
{
    s0: S0,
    s1: S1,
    s2: S2,
}

impl<S0, S1, S2> Mux<S0, S1, S2>
where
    S0: DigitalOutput,
    S1: DigitalOutput,
    S2: DigitalOutput,
{
    pub fn new(s0: S0, s1: S1, s2: S2) -> Self {
        Self { s0, s1, s2 }
    }

    pub fn select(&mut self, channel: u8) {
        self.s0.set_low();
        self.s1.set_low();
        self.s2.set_low();

        if channel & 0b001 != 0 {
            self.s0.set_high();
        }

        if channel & 0b010 != 0 {
            self.s1.set_high();
        }

        if channel & 0b100 != 0 {
            self.s2.set_high();
        }
    }
}
