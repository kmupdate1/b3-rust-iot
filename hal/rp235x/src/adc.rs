use embassy_rp::adc::Config;
use embassy_rp::bind_interrupts;
use hal_traits::analog_in::AnalogIn;

pub struct AdcPin<'a, T: embassy_rp::adc::AdcChannel> {
    channel: T,
    adc: &'a mut embassy_rp::adc::Adc<'static, embassy_rp::adc::Async>,
}

impl<'a, T: embassy_rp::adc::AdcChannel> AnalogIn for AdcPin<'a, T> {
    type Error = embassy_rp::adc::Error;

    async fn read(&mut self) -> Result<u16, Self::Error> {
        self.adc.read(&mut self.channel).await
    }
}

impl<'a, T> AdcPin<'a, T> {
    pub fn new(
        channel: T,
        adc: &'a mut embassy_rp::adc::Adc<'static, embassy_rp::adc::Async>,
    ) -> Self {
        Self { channel, adc }
    }
}

pub struct AdcDriver {
    adc: embassy_rp::adc::Adc<'static, embassy_rp::adc::Async>,
}

impl AdcDriver {
    pub fn init(p: embassy_rp::Peripherals) -> Self {
        bind_interrupts!(struct Irqs { ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler; });
        let adc = embassy_rp::adc::Adc::new(p.ADC, Irqs, Config::default());
        Self { adc }
    }

    pub fn create_pin<T: embassy_rp::adc::AdcChannel>(&mut self, channel: T) -> AdcPin<'_, T> {
        AdcPin::new(channel, &mut self.adc)
    }
}
