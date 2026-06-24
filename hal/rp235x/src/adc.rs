use hal_traits::analog_in::AnalogIn;

struct RpAdcPin<'a, T: embassy_rp::adc::AdcChannel> {
    channel: T,
    adc: &'a mut embassy_rp::adc::Adc<'static, embassy_rp::adc::Async>,
}

impl<'a, T: embassy_rp::adc::AdcChannel> AnalogIn for RpAdcPin<'a, T> {
    type Error = embassy_rp::adc::Error;

    async fn read(&mut self) -> Result<u16, Self::Error> {
        self.adc.read(&mut self.channel).await
    }
}

impl <'a, T> RpAdcPin<'a, T> {
    pub fn new(
        channel: T,
        adc: &'a mut embassy_rp::adc::Adc<'static, embassy_rp::adc::Async>,
    ) -> Self {
        Self { channel, adc }
    }
}
