#![no_std]
#![no_main]

use core::default::Default;
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::watchdog::Watchdog;
use embassy_time::Timer;
use capability::{Ipv4NetworkDevice, Wifi};
use rp235x::{Pico2wCyw43Resources, Rp235xCyw43, Rp235xWifi};
use embedded_alloc::LlffHeap;
use debugger::Indicator;
use rp235x::debugger::{Rp235xDebugger};
use runtime_core::OtaStatus;

const CURRENT_VERSION: &str = match option_env!("B3_FIRMWARE_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

const UPDATE_MANIFEST_URL: &str =
    "https://github.com/kmupdate1/b3-rust-iot/releases/latest/download/update-manifest.json";

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
    // A tentative image inherits the bootloader watchdog. Keep it running
    // until the board and network stack have initialized successfully.
    let mut watchdog = Watchdog::new(p.WATCHDOG);

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

    let network = Rp235xCyw43::new(wifi_resources, spawner).await;
    let mut ota = network.ota(p.FLASH, UPDATE_MANIFEST_URL);
    if let Err(error) = ota.confirm_boot() {
        log::error!("updater: failed to confirm current firmware: {:?}", error);
        debugger.indicator.red(true);
        return;
    }
    watchdog.stop();
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

    log::info!("Wi-Fi connected");
    log::info!("  - Ipv4: {:?}", ipv4);
    debugger.indicator.green(true);

    log::info!("updater: checking for update from {}", CURRENT_VERSION);
    match runtime_core::check(&mut ota, CURRENT_VERSION).await {
        Ok(OtaStatus::UpToDate) => {
            log::info!("updater: firmware is up to date");
            debugger.indicator.blue(true);
        }
        Ok(OtaStatus::ReadyToReboot) => {
            log::info!("updater: update verified; rebooting");
            debugger.indicator.blue(true);
            Timer::after_secs(1).await;
            cortex_m::peripheral::SCB::sys_reset();
        }
        Err(error) => {
            log::error!("updater: update failed: {:?}", error);
            debugger.indicator.red(true);
            debugger.indicator.blue(false);
        }
    }
}
