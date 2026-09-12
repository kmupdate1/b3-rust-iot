pub mod led;

use embassy_rp::Peri;
use embassy_rp::peripherals::{PIN_0, PIN_1, PIN_2};
pub use led::*;

pub struct Rp235xDebugger<'d> {
    pub led: Rp235xLedDebugger<'d>,
}

impl<'d> Rp235xDebugger<'d> {
    pub fn new(
        pin0: Peri<'d, PIN_0>,
        pin1: Peri<'d, PIN_1>,
        pin2: Peri<'d, PIN_2>,
    ) -> Self {
        Self {
            led: Rp235xLedDebugger::new(pin0, pin1, pin2),
        }
    }
}
