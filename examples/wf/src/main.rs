#![no_std]
#![no_main]

use core::default::Default;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use capability::{Http, Wifi};
use rp235x::{Pico2wCyw43Resources, Rp235xCyw43, Rp235xWifi};
use embedded_alloc::LlffHeap;

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let wifi_resources = Pico2wCyw43Resources::new(
        peripherals.PIN_23,
        peripherals.PIN_25,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.PIO0,
        peripherals.DMA_CH0,
        peripherals.DMA_CH1,
    );

    let network = Rp235xCyw43::builder(wifi_resources, spawner).await;
    let mut wifi = Rp235xWifi::new(&network);

    wifi
        .connect("Buffalo-2G-8F20", "hrtsedgmndi6c")
        .await
        .unwrap();

    let mut http = network.http();

    let mut buffer = [0u8; 4096];

    match http
        .get(
            "https://github.com/kmupdate1/b3-rust-iot/releases/latest/download/manifest.json",
            &mut buffer,
        )
        .await
    {
        Ok(len) => {
            info!(
                "Download complete!: {}",
                core::str::from_utf8(&buffer[..len]).unwrap_or("invalid utf-8"),
            );
        }

        Err(_) => {
            error!("manifest download failed");
        }
    }
}
