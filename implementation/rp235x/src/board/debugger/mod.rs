pub mod led;

pub use led::*;

pub struct Rp235xDebugger<'d> {
    pub led: Rp235xLed<'d>,
}

impl<'d> Rp235xDebugger<'d> {
    pub fn new(led: Rp235xLed<'d>) -> Self {
        Self { led }
    }
}
