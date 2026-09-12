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
use debugger::Led;
use rp235x::debugger::{Rp235xDebugger, Rp235xLed};

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let wifi_resources = Pico2wCyw43Resources::new(
        p.PIN_23,
        p.PIN_25,
        p.PIN_24,
        p.PIN_29,
        p.PIO0,
        p.DMA_CH0,
        p.DMA_CH1,
    );

    let led = Rp235xLed::new(
        p.PIN_0,
        p.PIN_1,
        p.PIN_2,
    );

    let mut debugger = Rp235xDebugger::new(led);

    let network = Rp235xCyw43::builder(wifi_resources, spawner).await;
    let mut wifi = Rp235xWifi::new(&network);

    wifi
        .connect("Buffalo-2G-8F20", "hrtsedgmndi6c")
        .await
        .unwrap();

    debugger.led.green(true);

    let mut http = network.http();

    let mut buffer = [0u8; 4096];

    let res = http
        .get(
            "https://github.com/kmupdate1/b3-rust-iot/releases/latest/download/manifest.json",
            &mut buffer,
        )
        .await;

    match res {
        Ok(len) => {
            info!(
                "Download complete!: {}",
                core::str::from_utf8(&buffer[..len]).unwrap_or("invalid utf-8"),
            );

            debugger.led.red(false);
            debugger.led.green(false);
            debugger.led.blue(true);
        }

        Err(_) => {
            error!("manifest download failed");

            debugger.led.red(true);
            debugger.led.green(false);
            debugger.led.blue(false);
        }
    }
}
