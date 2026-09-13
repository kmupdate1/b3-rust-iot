#![no_std]
#![no_main]

use core::default::Default;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Timer;
use capability::{Http, Ipv4NetworkDevice, Ipv6NetworkDevice, Wifi};
use rp235x::{Pico2wCyw43Resources, Rp235xCyw43, Rp235xWifi};
use embedded_alloc::LlffHeap;
use debugger::Indicator;
use rp235x::debugger::{Rp235xDebugger};

bind_interrupts!(struct UsbIrqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn usb_logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let usb_driver = Driver::new(p.USB, UsbIrqs);
    spawner.spawn(usb_logger_task(usb_driver).unwrap());

    // Give macOS time to enumerate the USB serial device before startup logs.
    Timer::after_secs(2).await;
    log::info!("wf example started");

    let wifi_resources = Pico2wCyw43Resources::new(
        p.PIN_23,
        p.PIN_25,
        p.PIN_24,
        p.PIN_29,
        p.PIO0,
        p.DMA_CH0,
        p.DMA_CH1,
    );

    let mut debugger = Rp235xDebugger::new(
        p.PIN_0,
        p.PIN_1,
        p.PIN_2,
    );

    let network = Rp235xCyw43::builder(wifi_resources, spawner).await;
    let mut wifi = Rp235xWifi::new(&network);

    log::info!("connecting to Wi-Fi");

    let con = wifi
        .connect("Buffalo-2G-8F20", "hrtsedgmndi6c")
        .await;

    if con.is_err() {
        log::error!("Wi-Fi connect failed");

        debugger.indicator.red(true);
        return;
    }

    let ipv4 = wifi
        .wait_ipv4_addr()
        .await
        .unwrap();

    /*
    let ipv6 = wifi
        .ipv6_addr()
        .unwrap();
    */

    log::info!("Wi-Fi connected");
    log::info!("  - Ipv4: {:?}", ipv4);
    // log::info!("  - Ipv6: {:?}", ipv6);

    debugger.indicator.green(true);

    let mut http = network.http();

    let mut buffer = [0u8; 16 * 1024];

    log::info!("downloading manifest.json");

    let res = http
        .get(
            "https://github.com/kmupdate1/b3-rust-iot/releases/latest/download/manifest.json",
            &mut buffer,
        )
        .await;

    match res {
        Ok(len) => {
            log::info!(
                "manifest.json downloaded ({} bytes): {}",
                len,
                core::str::from_utf8(&buffer[..len]).unwrap_or("invalid utf-8"),
            );

            debugger.indicator.red(false);
            debugger.indicator.blue(true);
        }

        Err(_) => {
            log::error!("manifest.json download failed");

            debugger.indicator.red(true);
            debugger.indicator.blue(false);
        }
    }
}
