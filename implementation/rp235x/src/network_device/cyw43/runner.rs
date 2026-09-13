use cyw43::{Cyw43439, Runner, SpiBus};
use cyw43_pio::PioSpi;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::PIO0;

#[embassy_executor::task]
pub(super) async fn cyw43_task(
    runner: Runner<
        'static,
        SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>,
        Cyw43439,
    >,
) -> ! { runner.run().await; }
