use core::time::Duration;

// TODO(b3): Revisit API after b3-rust-stdlib time module is available.
pub trait Delay {
    fn delay_ms(&mut self, duration: Duration);
    fn delay_us(&mut self, duration: Duration);
}
