use crate::peripherals::adc::Adc;
use crate::peripherals::pins::{Pin26, Pin27, Pin28};

pub struct Rp235xBoard {
    pub pin26: Pin26,
    pub pin27: Pin27,
    pub pin28: Pin28,
    
    pub adc: Adc,
    pub spi0: Spi0,
    pub uart0: Uart0,
}
