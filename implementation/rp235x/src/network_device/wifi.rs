use cyw43::aligned_bytes;
use cyw43_pio::{PioSpi, DEFAULT_CLOCK_DIVIDER};
use embassy_rp::{bind_interrupts, dma};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use static_cell::StaticCell;
use crate::Pico2wWifiResources;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>;
});

static STATE: StaticCell<cyw43::State> = StaticCell::new();

pub struct Rp235xWifi;

impl Rp235xWifi {
    pub async fn new(resources: Pico2wWifiResources<'static>) -> Self {
        let fw = aligned_bytes!("../../firmware/43439A0.bin");
        let nvram = aligned_bytes!("../../firmware/nvram_rp2040.bin");

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

        let state = STATE.init(cyw43::State::new());

        let (_net_device, _control, _runner) =
            cyw43::new(state, pwr, spi, fw, nvram).await;

        Self
    }
}
