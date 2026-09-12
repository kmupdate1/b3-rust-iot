use debugger::indicator::Indicator;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2};

pub struct Rp235xIndicatorDebugger<'d> {
    red: Output<'d>,
    green: Output<'d>,
    blue: Output<'d>,
}

impl<'d> Rp235xIndicatorDebugger<'d> {
    pub fn new(
        pin0: Peri<'d, PIN_0>,
        pin1: Peri<'d, PIN_1>,
        pin2: Peri<'d, PIN_2>,
    ) -> Self {
        Self {
            red: Output::new(pin0, Level::Low),
            green: Output::new(pin1, Level::Low),
            blue: Output::new(pin2, Level::Low),
        }
    }
}

impl Indicator for Rp235xIndicatorDebugger<'_> {
    fn red(&mut self, is_on: bool) {
        self.red.set_level(Level::from(is_on));
    }

    fn green(&mut self, is_on: bool) {
        self.green.set_level(Level::from(is_on));
    }

    fn blue(&mut self, is_on: bool) {
        self.blue.set_level(Level::from(is_on));
    }
}
