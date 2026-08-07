use embassy_rp::adc::{Adc, Async, Channel, Config, Error, InterruptHandler};
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::ADC;
use embassy_rp::{bind_interrupts, Peri};

use function::{AnalogChannel, AnalogReader};
use crate::{Pin26, Pin27, Pin28};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

pub struct Rp235xAdc<'d> {
    inner: Adc<'d, Async>,
}

impl<'d> Rp235xAdc<'d> {
    pub fn new(adc: Peri<'d, ADC>) -> Self {
        Self {
            inner: Adc::new(adc, Irqs, Config::default()),
        }
    }
}

macro_rules! impl_define_adc {
    ($($analog:ident => $pin:ident),* $(,)?) => {$(
        pub struct $analog<'d> {
            channel: Channel<'d>
        }

        impl<'d> $analog<'d> {
            pub fn new(pin: $pin<'d>) -> Self {
                Self {
                    channel: Channel::new_pin(
                        pin.into_inner(),
                        Pull::None,
                    ),
                }
            }
        }

        impl AnalogChannel for $analog<'_> {}

        impl AnalogReader<$analog<'_>> for Rp235xAdc<'_> {
            type Error = Error;

            async fn read(
                &mut self,
                channel: &mut $analog<'_>,
            ) -> Result<u16, Self::Error> {
                self.inner.read(&mut channel.channel).await
            }
        }
    )*};
}

impl_define_adc!(
    AdcPin26 => Pin26,
    AdcPin27 => Pin27,
    AdcPin28 => Pin28,
);
