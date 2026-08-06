use core::result::Result;
use crate::connection::Rp235xCon;
use embassy_rp::adc::Channel;
use embassy_rp::gpio::Pull;
use embassy_rp::peripherals::ADC;
use embassy_rp::Peripherals;

pub struct Adc {
    inner: ADC,
}

impl Adc {
    pub fn new(adc: ADC) -> Self {
        Self { inner: adc }
    }

    pub fn attach_pin26(self, p: Peripherals) -> Rp235xCon {
        Rp235xCon::new(Channel::new_pin(p.PIN_26, Pull::None))
    }
    pub fn attach_pin27(self, p: Peripherals) -> Rp235xCon {
        Rp235xCon::new(Channel::new_pin(p.PIN_27, Pull::None))
    }
    pub fn attach_pin28(self, p: Peripherals) -> Rp235xCon {
        Rp235xCon::new(Channel::new_pin(p.PIN_28, Pull::None))
    }

    pub async fn read(&mut self, con: &mut Rp235xCon) -> Result<u16, Self::Error> {
        Ok(self.inner.read(con.channel_mut()).await)
    }
}
