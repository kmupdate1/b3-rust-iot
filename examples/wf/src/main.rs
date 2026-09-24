#![no_std]
#![no_main]

mod application;

use core::default::Default;
use capability::l3::Ipv4NetworkDevice;
use capability::Wifi;
use debugger::Indicator;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::watchdog::Watchdog;
use embassy_time::Timer;
use embedded_alloc::LlffHeap;
use ota::{UpdateOutcome, Updater};
use panic_probe as _;
use rp235x::debugger::Rp235xDebugger;
use rp235x::{Pico2wCyw43Resources, Rp235xFirmwareTarget, Rp235xUpdateSource};
use rp235x::device::cyw43::Rp235xCyw43;

const CURRENT_VERSION: &str = match option_env!("B3_FIRMWARE_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};

const WIFI_SSID: &str = env!("B3C_ALPHA_WIFI_SSID");
const WIFI_PASSWORD: &str = env!("B3C_ALPHA_WIFI_PASSWORD");

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
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    watchdog.stop();

    let usb_driver = Driver::new(p.USB, UsbIrqs);
    spawner.spawn(usb_logger_task(usb_driver).unwrap());

    Timer::after_secs(2).await;

    log::info!("BOOT: Via 'USB' installed binary launched");
    log::info!("BOOT: firmware={}", CURRENT_VERSION);
    log::info!("BOOT: watchdog stopped");

    let nw_resources = Pico2wCyw43Resources::new(
        p.PIN_23, p.PIN_25, p.PIN_24, p.PIN_29, p.PIO0, p.DMA_CH0, p.DMA_CH1,
    );

    let mut debugger = Rp235xDebugger::new(p.PIN_0, p.PIN_1, p.PIN_2);

    let network = Rp235xCyw43::new(nw_resources, spawner).await;

    let mut wifi = network.wifi();

    if wifi
        .connect(WIFI_SSID, WIFI_PASSWORD)
        .await
        .is_err() {
        log::error!("Wi-Fi connect failed");
        debugger.indicator.red(true);
        return;
    }

    let ipv4 = wifi
        .wait_ipv4_addr()
        .await
        .unwrap();

    log::info!("Wi-Fi connected: {:?}", ipv4);

    let source = Rp235xUpdateSource::new(wifi.http(), UPDATE_MANIFEST_URL);
    let target = Rp235xFirmwareTarget::new(p.FLASH);
    let mut updater = Updater::new(source, target);

    // The application owns the health policy; the target only persists it.
    if let Err(error) = updater.confirm_boot() {
        log::error!("updater: failed to confirm current firmware: {:?}", error);
        debugger.indicator.red(true);
        return;
    }

    // watchdog.stop();
    debugger.indicator.green(true);

    match updater.check_and_update(CURRENT_VERSION).await {
        Ok(UpdateOutcome::UpToDate) => {
            log::info!("updater: firmware is up to date");
            debugger.indicator.blue(true);
        }

        Ok(UpdateOutcome::ReadyToReboot) => {
            debugger.indicator.blue(true);
            Timer::after_secs(1).await;
            log::info!("updater: update verified");

            log::info!("updater: wifi disconnecting before reboot");
            match wifi.disconnect().await {
                Ok(()) => log::info!("wifi disconnected"),
                Err(error) => log::error!("wifi: disconnecting failed: {:?}", error),
            }

            debugger.indicator.blue(false);
            log::info!("updater: rebooting...");
            cortex_m::peripheral::SCB::sys_reset();
        }

        Err(error) => {
            log::error!("updater: update failed: {:?}", error);
            debugger.indicator.red(true);
            debugger.indicator.blue(false);
        }
    }
}
