use embassy_rp::gpio::{Level, Output, Pin};
use embassy_rp::Peri;
use capability::{DigitalOutput, IntoDigitalOutput};
use crate::*;

pub struct DigitalOutputPin<'d> {
    pin: Output<'d>,
}

impl<'d> DigitalOutputPin<'d> {
    pub fn new(pin: Peri<'d, impl Pin>, initial: Level) -> Self {
        Self {
            pin: Output::new(pin, initial),
        }
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

macro_rules! impl_digital_output {
    ($($pin:ident),* $(,)?) => {$(
        impl<'d> IntoDigitalOutput for $pin<'d> {
            type Output = DigitalOutputPin<'d>;

            fn into_digital_output(self) -> Self::Output {
                DigitalOutputPin::new(
                    self.into_inner(),
                    Level::Low,
                )
            }
        }
    )*};
}

impl_digital_output!(
    Pin0,
    Pin1,
    Pin2,
    Pin3,
    Pin4,
    Pin5,
    Pin6,
    Pin7,
    Pin8,
    Pin9,
    Pin10,
    Pin11,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16,
    Pin17,
    Pin18,
    Pin19,
    Pin20,
    Pin21,
    Pin22,
    Pin23,
    Pin24,
    Pin25,
    Pin26,
    Pin27,
    Pin28,
    Pin29,
);
