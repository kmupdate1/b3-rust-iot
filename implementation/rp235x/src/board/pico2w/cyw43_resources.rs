use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::Peri;

pub struct Pico2wCyw43Resources<'d> {
    pub(crate) pwr: Peri<'d, PIN_23>,
    pub(crate) cs: Peri<'d, PIN_25>,
    pub(crate) dio: Peri<'d, PIN_24>,
    pub(crate) clk: Peri<'d, PIN_29>,
    pub(crate) pio: Peri<'d, PIO0>,
    pub(crate) dma0: Peri<'d, DMA_CH0>,
    pub(crate) dma1: Peri<'d, DMA_CH1>,
}

impl<'d> Pico2wCyw43Resources<'d> {
    pub fn new(
        pwr: Peri<'d, PIN_23>,
        cs: Peri<'d, PIN_25>,
        dio: Peri<'d, PIN_24>,
        clk: Peri<'d, PIN_29>,
        pio: Peri<'d, PIO0>,
        dma0: Peri<'d, DMA_CH0>,
        dma1: Peri<'d, DMA_CH1>,
    ) -> Self {
        Self {
            pwr, cs, dio, clk, pio, dma0, dma1,
        }
    }
}
