use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_rp::{bind_interrupts, dma};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use crate::Pico2wCyw43Resources;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>;
});

pub(crate) struct Bus {
    pub pwr: Output<'static>,
    pub spi: PioSpi<'static, PIO0, 0>,
}

pub(crate) fn init(
    resources: Pico2wCyw43Resources<'static>,
) -> Bus {
    let pwr = Output::new(resources.pwr, Level::Low);
    let cs = Output::new(resources.cs, Level::High);

    let mut pio = Pio::new(resources.pio, Irqs);

    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        resources.dio,
        resources.clk,
        dma::Channel::new(resources.dma0, Irqs),
        dma::Channel::new(resources.dma1, Irqs),
    );

    Bus { pwr, spi }
}
